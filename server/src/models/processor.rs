use mongodb::{
    Database,
    bson::{doc, to_bson},
};
use serde::{Deserialize, Serialize};

use crate::models::evidence::{Evidence, EvidenceQuery};

use super::event::EventKind;

const COLLECTION: &str = "processors";

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessorRequest {
    pub id: String,
    pub name: String,
    pub model: String,
    pub address: ProcessorAddress,
    pub version: i64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Processor {
    pub id: String,
    pub cluster_id: String,
    pub name: String,
    pub model: String,
    pub address: ProcessorAddress,
    pub version: i64, // Comparable version
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProcessorAddress {
    pub host: [u8; 4],
    pub port: u16,
}
#[derive(Debug, Deserialize)]
pub struct ProcessorQuery {
    pub cluster_id: Option<String>,
    pub processor_id: Option<String>,
    pub date_minimum: Option<i64>,
    pub date_maximum: Option<i64>,
    pub text: Option<String>,
    pub limit: Option<usize>,
    pub skip: Option<usize>,
}

impl Processor {
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
            &EvidenceQuery {
                cluster_id: None,
                processor_id: Some(self.id.clone()),
                camera_id: None,
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
}
