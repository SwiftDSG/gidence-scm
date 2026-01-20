use futures::StreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_bson},
    Database,
};
use serde::{Deserialize, Serialize};

use super::{event::EventKind, uniform::UniformResponse};

const COLLECTION: &str = "clusters";

#[derive(Debug, Deserialize, Serialize)]
pub struct ClusterRequest {
    pub uniform_id: Vec<ObjectId>,
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Cluster {
    pub _id: ObjectId,
    pub uniform_id: Vec<ObjectId>,
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ClusterResponse {
    pub id: String,
    pub uniform_id: Vec<String>,
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ClusterMinimalResponse {
    pub id: String,
    pub uniform: Vec<UniformResponse>,
    pub name: String,
    pub processor_count: usize,
    pub notification_count: usize,
    pub violation_count: usize,
}

#[derive(Clone, Deserialize)]
pub struct ClusterQuery {
    pub cluster_id: Option<Vec<ObjectId>>,
    pub date_minimum: Option<i64>,
    pub date_maximum: Option<i64>,
    pub text: Option<String>,
}

impl From<ClusterRequest> for Cluster {
    fn from(a: ClusterRequest) -> Self {
        Self {
            _id: ObjectId::new(),
            uniform_id: a.uniform_id,
            name: a.name,
        }
    }
}
impl From<Cluster> for ClusterResponse {
    fn from(mut a: Cluster) -> Self {
        Self {
            id: a._id.to_string(),
            uniform_id: a.uniform_id.drain(..).map(|a| a.to_string()).collect(),
            name: a.name,
        }
    }
}

impl Cluster {
    pub async fn save(&self, db: &Database) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        if collection.insert_one(self, None).await.is_ok() {
            Ok(())
        } else {
            Err(EventKind::SavingFailed)
        }
    }
    pub async fn update(
        &mut self,
        request: ClusterRequest,
        db: &Database,
    ) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        self.name = request.name;

        if collection
            .update_one(
                doc! { "_id": self._id },
                doc! { "$set": to_bson::<Self>(self).unwrap() },
                None,
            )
            .await
            .is_ok()
        {
            Ok(())
        } else {
            Err(EventKind::UpdatingFailed)
        }
    }
    pub async fn delete(&self, db: &Database) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        if collection
            .delete_one(doc! { "_id": self._id }, None)
            .await
            .is_ok()
        {
            Ok(())
        } else {
            Err(EventKind::DeletingFailed)
        }
    }
    pub async fn find_by_id(_id: &ObjectId, db: &Database) -> Result<Self, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        match collection.find_one(doc! { "_id": _id }, None).await {
            Ok(Some(v)) => Ok(v),
            Ok(_) => Err(EventKind::NotFound),
            Err(e) => {
                println!("ERROR: {:?}", e);
                Err(EventKind::FindingFailed)
            }
        }
    }
    pub async fn find_all(db: &Database) -> Result<Vec<Self>, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        match collection.find(doc! {}, None).await {
            Ok(mut cursor) => {
                let mut clusters = Vec::new();
                while let Some(Ok(cluster)) = cursor.next().await {
                    clusters.push(cluster);
                }

                if clusters.is_empty() {
                    Err(EventKind::NotFound)
                } else {
                    Ok(clusters)
                }
            }
            Err(e) => {
                println!("ERROR: {:?}", e);
                Err(EventKind::FindingFailed)
            }
        }
    }
    pub async fn find_many_minimal(
        query: &ClusterQuery,
        db: &Database,
    ) -> Result<Vec<ClusterMinimalResponse>, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut pipeline = Vec::new();
        let mut queries = Vec::new();

        if let Some(text) = &query.text {
            queries.push(doc! {
                "$regexMatch": {
                    "input": "$name",
                    "options": "i",
                    "regex": to_bson::<String>(text).unwrap()
                }
            });
        }
        if let Some(cluster_id) = &query.cluster_id {
            queries.push(doc! {
                "$in": ["$_id", to_bson::<Vec<ObjectId>>(cluster_id).unwrap()]
            });
        }

        pipeline.push(doc! {
            "$match": {
                "$expr": {
                    "$and": queries
                }
            }
        });

        let mut violation_query = Vec::from([doc! {
            "$eq": ["$cluster_id", "$$cluster_id"]
        }]);

        if let Some(date) = &query.date_minimum {
            violation_query.push(doc! {
                "$gte": ["$timestamp", date]
            });
        }
        if let Some(date) = &query.date_maximum {
            violation_query.push(doc! {
                "$lte": ["$timestamp", date]
            });
        }

        pipeline.push(doc! {
            "$lookup": {
                "from": "violations",
                "let": { "cluster_id": "$_id" },
                "as": "notification",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$and": violation_query.clone()
                            }
                        }
                    },
                    {
                        "$count": "count"
                    }
                ]
            }
        });
        pipeline.push(doc! {
            "$lookup": {
                "from": "violations",
                "let": { "cluster_id": "$_id" },
                "as": "violation",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$and": violation_query
                            }
                        }
                    },
                    {
                        "$unwind": "$kind"
                    },
                    {
                        "$count": "count"
                    }
                ]
            }
        });
        pipeline.push(doc! {
            "$lookup": {
                "from": "processors",
                "let": { "cluster_id": "$_id" },
                "as": "processor",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$eq": ["$cluster_id", "$$cluster_id"]
                            }
                        }
                    },
                    {
                        "$count": "count"
                    }
                ]
            }
        });

        pipeline.push(doc! {
            "$lookup": {
                "from": "uniforms",
                "let": { "uniform_id": "$uniform_id" },
                "as": "uniform",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$in": ["$_id", "$$uniform_id"]
                            }
                        }
                    },
                    {
                        "$project": {
                            "id": {
                                "$toString": "$_id"
                            },
                            "name": "$name",
                            "attribute": "$attribute",
                        }
                    },
                ]
            }
        });
        pipeline.push(doc! {
            "$project": {
                "id": {
                    "$toString": "$_id"
                },
                "name": "$name",
                "uniform": "$uniform",
                "processor_count": {
                    "$cond": [
                        { "$first": "$processor" },
                        { "$first": "$processor.count" },
                        0
                    ]
                },
                "notification_count": {
                    "$cond": [
                        { "$first": "$notification" },
                        { "$first": "$notification.count" },
                        0
                    ]
                },
                "violation_count": {
                    "$cond": [
                        { "$first": "$violation" },
                        { "$first": "$violation.count" },
                        0
                    ]
                },
            }
        });

        match collection.aggregate(pipeline, None).await {
            Ok(mut cursor) => {
                let mut clusters = Vec::new();
                while let Some(Ok(doc)) = cursor.next().await {
                    let cluster = from_document::<ClusterMinimalResponse>(doc).unwrap();
                    clusters.push(ClusterMinimalResponse::from(cluster));
                }
                if !clusters.is_empty() {
                    Ok(clusters)
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
    pub async fn find_one_minimal(
        query: &ClusterQuery,
        db: &Database,
    ) -> Result<ClusterMinimalResponse, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut pipeline = Vec::new();
        let mut queries = Vec::new();

        if let Some(text) = &query.text {
            queries.push(doc! {
                "$regexMatch": {
                    "input": "$name",
                    "options": "i",
                    "regex": to_bson::<String>(text).unwrap()
                }
            });
        }
        if let Some(cluster_id) = &query.cluster_id {
            queries.push(doc! {
                "$in": ["$_id", to_bson::<Vec<ObjectId>>(cluster_id).unwrap()]
            });
        }

        pipeline.push(doc! {
            "$match": {
                "$expr": {
                    "$and": queries
                }
            }
        });
        pipeline.push(doc! {
            "$lookup": {
                "from": "processors",
                "let": { "cluster_id": "$_id" },
                "as": "processor",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$eq": ["$cluster_id", "$$cluster_id"]
                            }
                        }
                    },
                    {
                        "$count": "count"
                    }
                ]
            }
        });

        let mut violation_query = Vec::from([doc! {
            "$eq": ["$cluster_id", "$$cluster_id"]
        }]);

        if let Some(date) = &query.date_minimum {
            violation_query.push(doc! {
                "$gte": ["$timestamp", date]
            });
        }
        if let Some(date) = &query.date_maximum {
            violation_query.push(doc! {
                "$lte": ["$timestamp", date]
            });
        }

        pipeline.push(doc! {
            "$lookup": {
                "from": "violations",
                "let": { "cluster_id": "$_id" },
                "as": "notification",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$and": violation_query.clone()
                            }
                        }
                    },
                    {
                        "$count": "count"
                    }
                ]
            }
        });
        pipeline.push(doc! {
            "$lookup": {
                "from": "violations",
                "let": { "cluster_id": "$_id" },
                "as": "violation",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$and": violation_query
                            }
                        }
                    },
                    {
                        "$unwind": "$attribute"
                    },
                    {
                        "$count": "count"
                    }
                ]
            }
        });
        pipeline.push(doc! {
            "$lookup": {
                "from": "uniforms",
                "let": { "uniform_id": "$uniform_id" },
                "as": "uniform",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$in": ["$_id", "$$uniform_id"]
                            }
                        }
                    },
                    {
                        "$project": {
                            "id": {
                                "$toString": "$_id"
                            },
                            "name": "$name",
                            "attribute": "$attribute",
                        }
                    },
                ]
            }
        });

        pipeline.push(doc! {
            "$project": {
                "id": {
                    "$toString": "$_id"
                },
                "name": "$name",
                "uniform": "$uniform",
                "processor_count": {
                    "$cond": [
                        { "$first": "$processor" },
                        { "$first": "$processor.count" },
                        0
                    ]
                },
                "notification_count": {
                    "$cond": [
                        { "$first": "$notification" },
                        { "$first": "$notification.count" },
                        0
                    ]
                },
                "violation_count": {
                    "$cond": [
                        { "$first": "$violation" },
                        { "$first": "$violation.count" },
                        0
                    ]
                },
            }
        });

        match collection.aggregate(pipeline, None).await {
            Ok(mut cursor) => {
                if let Some(Ok(doc)) = cursor.next().await {
                    let cluster = from_document::<ClusterMinimalResponse>(doc).unwrap();
                    Ok(cluster)
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
}
