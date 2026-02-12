use futures::StreamExt;
use mongodb::{
    Database,
    bson::{doc, from_document, to_bson},
};
use serde::{Deserialize, Serialize};

use crate::models::evidence::{Evidence, EvidenceQuery};

use super::event::EventKind;

const COLLECTION: &str = "cameras";

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraRequest {
    pub id: String,
    pub address: CameraAddress,
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Camera {
    pub id: String,
    pub cluster_id: String,
    pub processor_id: String,
    pub address: CameraAddress,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CameraAddress {
    pub host: [u8; 4],
    pub port: u16,
    pub path: Option<String>,
    pub authentication: Option<(String, String)>,
}
#[derive(Debug, Deserialize)]
pub struct CameraQuery {
    pub cluster_id: Option<String>,
    pub processor_id: Option<String>,
    pub date_minimum: Option<i64>,
    pub date_maximum: Option<i64>,
    pub text: Option<String>,
    pub limit: Option<usize>,
    pub skip: Option<usize>,
}

impl Camera {
    pub async fn save(&self, db: &Database) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        if Self::find_by_id(&self.id, db).await.is_ok() {
            return self.update(db).await;
        }

        if collection.insert_one(self, None).await.is_ok() {
            Ok(())
        } else {
            Err(EventKind::SavingFailed)
        }
    }
    pub async fn update(&self, db: &Database) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        if collection
            .update_one(
                doc! { "id": &self.id },
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

        let _ = Evidence::delete_many(
            &&EvidenceQuery {
                cluster_id: None,
                processor_id: None,
                camera_id: Some(self.id.clone()),
                date_minimum: None,
                date_maximum: None,
            },
            db,
        )
        .await;

        if collection
            .delete_one(doc! { "id": &self.id }, None)
            .await
            .is_ok()
        {
            Ok(())
        } else {
            Err(EventKind::DeletingFailed)
        }
    }
    pub async fn find_by_id(id: &String, db: &Database) -> Result<Self, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        match collection.find_one(doc! { "id": id }, None).await {
            Ok(Some(v)) => Ok(v),
            Ok(_) => Err(EventKind::NotFound),
            Err(e) => {
                println!("ERROR: {:?}", e);
                Err(EventKind::FindingFailed)
            }
        }
    }
    pub async fn find_many(query: &CameraQuery, db: &Database) -> Result<Vec<Self>, EventKind> {
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
                "$eq": ["$cluster_id", to_bson::<String>(cluster_id).unwrap()]
            });
        }
        if let Some(processor_id) = &query.processor_id {
            queries.push(doc! {
                "$eq": ["$processor_id", to_bson::<String>(processor_id).unwrap()]
            });
        }

        pipeline.push(doc! {
            "$match": {
                "$expr": {
                    "$and": queries
                }
            }
        });

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
                let mut cameras = Vec::new();
                while let Some(Ok(doc)) = cursor.next().await {
                    let camera = from_document::<Self>(doc).unwrap();
                    cameras.push(Self::from(camera));
                }
                if !cameras.is_empty() {
                    Ok(cameras)
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
