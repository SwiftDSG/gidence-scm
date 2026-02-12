use futures::StreamExt;
use mongodb::{
    Database,
    bson::{Document, doc, from_document, to_bson},
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        event::EventKind,
        user::{User, UserQuery, UserRole},
    },
    views::cluster::{ClusterRef, ViewCluster},
};

const COLLECTION: &str = "users";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserRef {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ViewUser {
    pub id: String,
    pub cluster: Vec<ClusterRef>,
    pub number: String,
    pub name: String,
    pub role: UserRole,
}

impl ViewUser {
    pub async fn from(user: User, db: &Database) -> Self {
        let cluster_ids: Vec<String> = user.cluster_id.iter().map(|id| id.to_string()).collect();

        let clusters = ViewCluster::find_many(
            &crate::models::cluster::ClusterQuery {
                cluster_id: Some(cluster_ids),
                text: None,
                date_minimum: None,
                date_maximum: None,
            },
            db,
        )
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|c| ClusterRef {
            id: c.id,
            name: c.name,
        })
        .collect();

        Self {
            id: user.id.to_string(),
            cluster: clusters,
            number: user.number,
            name: user.name,
            role: user.role,
        }
    }
    pub async fn find_many(query: &UserQuery, db: &Database) -> Result<Vec<Self>, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut user_query = Vec::new();

        if let Some(text) = &query.text {
            user_query.push(doc! {
                "$regexMatch": {
                    "input": "$name",
                    "options": "i",
                    "regex": to_bson::<String>(text).unwrap()
                }
            });
        }
        if let Some(cluster_id) = &query.cluster_id {
            user_query.push(doc! {
                "$in": [to_bson::<String>(cluster_id).unwrap(), "$cluster_id"]
            });
        }
        if let Some(cluster_id) = &query.cluster_eid {
            user_query.push(doc! {
                "$eq": [
                    {
                        "$in": [to_bson::<String>(cluster_id).unwrap(), "$cluster_id"]
                    },
                    false
                ]
            });
            user_query.push(doc! {
                "$ne": [ "$role", "super_admin" ]
            });
        }

        let mut pipeline = vec![
            Self::create_match_stage(&user_query),
            Self::create_cluster_lookup_stage(),
            Self::create_project_stage(),
        ];

        if let Some(skip) = query.skip {
            pipeline.push(doc! {
                "$skip": to_bson::<usize>(&skip).unwrap()
            });
        }
        if let Some(limit) = query.limit {
            pipeline.push(doc! {
                "$limit": to_bson::<usize>(&limit).unwrap()
            });
        }

        match collection.aggregate(pipeline, None).await {
            Ok(mut cursor) => {
                let mut users = Vec::new();
                while let Some(Ok(doc)) = cursor.next().await {
                    let user = from_document::<Self>(doc).unwrap();
                    users.push(user);
                }
                if !users.is_empty() {
                    Ok(users)
                } else {
                    Err(EventKind::NotFound)
                }
            }
            Err(e) => {
                println!("ERROR: {:?}", e);
                Err(EventKind::FindingFailed)
            }
        }
    }

    // Helper functions to create aggregation stages
    fn create_match_stage(query: &Vec<Document>) -> Document {
        doc! {
            "$match": {
                "$expr": {
                    "$and": query
                }
            }
        }
    }
    fn create_cluster_lookup_stage() -> Document {
        doc! {
            "$lookup": {
                "from": "clusters",
                "let": { "cluster_id": "$cluster_id" },
                "as": "cluster",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$in": ["$id", "$$cluster_id"]
                            }
                        }
                    },
                    {
                        "$project": {
                            "id": "$id",
                            "name": "$name",
                        }
                    }
                ]
            }
        }
    }
    fn create_project_stage() -> Document {
        doc! {
            "$project": {
                "id": "$id",
                "cluster": "$cluster",
                "name": "$name",
                "number": "$number",
                "role": "$role"
            }
        }
    }
}
