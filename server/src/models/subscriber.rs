use futures::StreamExt;
use mongodb::{
    Database,
    bson::{doc, to_bson},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::event::EventKind;

const COLLECTION: &str = "subscribers";

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriberRequest {
    pub user_id: String,
    pub kind: SubscriberKind,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Subscriber {
    pub id: String,
    pub user_id: String,
    pub kind: SubscriberKind,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriberKind {
    Apple(String),
}

#[derive(Deserialize)]
pub struct SubscriberQuery {
    pub kind: Option<SubscriberQueryKind>,
    pub token: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriberQueryKind {
    Apple,
}

impl From<SubscriberRequest> for Subscriber {
    fn from(a: SubscriberRequest) -> Self {
        Self {
            id: String::new(),
            user_id: a.user_id,
            kind: a.kind,
        }
    }
}

impl Subscriber {
    pub async fn save(&mut self, db: &Database) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        self.id = Uuid::new_v4().to_string();

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

    pub async fn find_by_kind(kind: &SubscriberKind, db: &Database) -> Result<Self, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let query = match kind {
            SubscriberKind::Apple(token) => doc! { "kind.apple": token },
        };

        match collection.find_one(query, None).await {
            Ok(Some(v)) => Ok(v),
            Ok(_) => Err(EventKind::NotFound),
            Err(e) => {
                println!("ERROR: {:?}", e);
                Err(EventKind::FindingFailed)
            }
        }
    }
    pub async fn find_many_by_user_id(
        user_id: &String,
        db: &Database,
    ) -> Result<Vec<Self>, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        match collection.find(doc! { "user_id": user_id }, None).await {
            Ok(mut cursor) => {
                let mut subscribers = Vec::new();

                while let Some(Ok(subscriber)) = cursor.next().await {
                    subscribers.push(subscriber);
                }

                if subscribers.is_empty() {
                    Err(EventKind::NotFound)
                } else {
                    Ok(subscribers)
                }
            }
            Err(e) => {
                println!("ERROR: {:?}", e);
                Err(EventKind::FindingFailed)
            }
        }
    }
}
