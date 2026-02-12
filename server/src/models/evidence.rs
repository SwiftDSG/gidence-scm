use mongodb::{
    Database,
    bson::{doc, to_bson},
};
use serde::{Deserialize, Serialize};

use super::event::EventKind;

const COLLECTION: &str = "evidences";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EvidenceRequest {
    pub camera_id: String,
    pub frame_id: String,
    pub timestamp: i64,
    pub person: Vec<EvidencePerson>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Evidence {
    pub id: String,
    pub cluster_id: String,
    pub processor_id: String,
    pub camera_id: String,
    pub frame_id: String,
    pub timestamp: i64,
    pub person: Vec<EvidencePerson>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EvidencePerson {
    pub id: String,
    pub bbox: [f32; 4],
    pub confidence: f32,
    pub part: Vec<EvidencePersonPart>,
    pub equipment: Vec<EvidencePersonEquipment>,
    pub violation: Vec<EvidencePersonViolation>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EvidencePersonPart {
    pub label: EvidencePersonPartLabel,
    pub bbox: [f32; 4],
    pub confidence: f32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EvidencePersonEquipment {
    pub label: EvidencePersonEquipmentLabel,
    pub bbox: [f32; 4],
    pub confidence: f32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EvidencePersonPartLabel {
    Head,
    Hand,
    Face,
    Foot,
    Ear,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EvidencePersonEquipmentLabel {
    Hardhat,
    Gloves,
    Shoes,
    Safetyvest,
    Safetysuit,
    Facemask,
    Faceguard,
    Earmuffs,
    Glasses,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidencePersonViolation {
    MissingHardhat,
    MissingGloves,
    MissingShoes,
    MissingFacemask,
    MissingEarmuffs,
    MissingSafetyvest,
    ImproperlyWornHardhat,
    ImproperlyWornGloves,
    ImproperlyWornShoes,
    ImproperlyWornFacemask,
    ImproperlyWornEarmuffs,
}

#[derive(Debug, Deserialize)]
pub struct EvidenceQuery {
    pub cluster_id: Option<String>,
    pub processor_id: Option<String>,
    pub camera_id: Option<String>,
    pub date_minimum: Option<i64>,
    pub date_maximum: Option<i64>,
}

impl Evidence {
    pub async fn save(&self, db: &Database) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        if collection.insert_one(self, None).await.is_ok() {
            Ok(())
        } else {
            Err(EventKind::SavingFailed)
        }
    }
    pub async fn update(&mut self, db: &Database) -> Result<(), EventKind> {
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
    pub async fn delete_many(query: &EvidenceQuery, db: &Database) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut queries = Vec::new();

        if let Some(cluster_id) = &query.cluster_id {
            queries.push(doc! {
                "cluster_id": cluster_id
            });
        }
        if let Some(processor_id) = &query.processor_id {
            queries.push(doc! {
                "processor_id": processor_id
            });
        }

        if collection
            .delete_many(
                doc! {
                    "$and": queries
                },
                None,
            )
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
