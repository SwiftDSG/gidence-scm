use mongodb::{Database, bson::doc};
use serde::{Deserialize, Serialize};

const COLLECTION: &str = "events";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: String,
    pub target: Option<EventTarget>,
    pub kind: EventKind,
    pub timestamp: i64,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EventTarget {
    Cluster(Option<String>),
    Processor(Option<String>),
    Evidence(Option<String>),
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EventKind {
    Saved,
    SavingFailed,
    Updated,
    UpdatingFailed,
    Deleted,
    DeletingFailed,
    FindingFailed,
    NotFound,
    InvalidCombination,
    InvalidToken,
    InvalidId,
}

impl EventKind {
    pub fn to_string(&self) -> String {
        match self {
            EventKind::Saved => String::from("Saved"),
            EventKind::SavingFailed => String::from("SavingFailed"),
            EventKind::Updated => String::from("Updated"),
            EventKind::UpdatingFailed => String::from("UpdatingFailed"),
            EventKind::Deleted => String::from("Deleted"),
            EventKind::DeletingFailed => String::from("DeletingFailed"),
            EventKind::FindingFailed => String::from("FindingFailed"),
            EventKind::NotFound => String::from("NotFound"),
            EventKind::InvalidCombination => String::from("InvalidCombination"),
            EventKind::InvalidToken => String::from("InvalidToken"),
            EventKind::InvalidId => String::from("InvalidId"),
        }
    }
}

impl Event {
    pub async fn save(&self, db: &Database) {
        let collection = db.collection::<Self>(COLLECTION);

        let _ = collection.insert_one(self, None).await;
    }
    pub async fn delete(&self, db: &Database) {
        let collection = db.collection::<Self>(COLLECTION);

        let _ = collection.delete_one(doc! { "id": &self.id }, None).await;
    }
}
