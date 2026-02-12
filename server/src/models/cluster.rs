use futures::StreamExt;
use mongodb::{
    Database,
    bson::{doc, to_bson},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::event::EventKind;

const COLLECTION: &str = "clusters";

#[derive(Debug, Deserialize, Serialize)]
pub struct ClusterRequest {
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Cluster {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Deserialize)]
pub struct ClusterQuery {
    pub cluster_id: Option<Vec<String>>,
    pub date_minimum: Option<i64>,
    pub date_maximum: Option<i64>,
    pub text: Option<String>,
}

impl From<ClusterRequest> for Cluster {
    fn from(a: ClusterRequest) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
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
}
