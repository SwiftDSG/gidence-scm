use futures::StreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_bson},
    Database,
};
use serde::{Deserialize, Serialize};

use super::event::EventKind;

const COLLECTION: &str = "uniforms";

#[derive(Debug, Deserialize, Serialize)]
pub struct UniformRequest {
    pub name: String,
    pub attribute: Vec<UniformAttribute>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Uniform {
    pub _id: ObjectId,
    pub name: String,
    pub attribute: Vec<UniformAttribute>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UniformResponse {
    pub id: String,
    pub name: String,
    pub attribute: Vec<UniformAttribute>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UniformAttribute {
    Mask,
    HardHat,
    HairCap,
    SafetyVest,
}

#[derive(Clone, Deserialize)]
pub struct UniformQuery {
    pub uniform_id: Option<Vec<ObjectId>>,
    pub text: Option<String>,
}

impl From<UniformRequest> for Uniform {
    fn from(a: UniformRequest) -> Self {
        Self {
            _id: ObjectId::new(),
            name: a.name,
            attribute: a.attribute,
        }
    }
}
impl From<Uniform> for UniformResponse {
    fn from(a: Uniform) -> Self {
        Self {
            id: a._id.to_string(),
            name: a.name,
            attribute: a.attribute,
        }
    }
}

impl Uniform {
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
        request: UniformRequest,
        db: &Database,
    ) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        self.name = request.name;
        self.attribute = request.attribute;

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
                let mut uniforms = Vec::new();
                while let Some(Ok(uniform)) = cursor.next().await {
                    uniforms.push(uniform);
                }

                if uniforms.is_empty() {
                    Err(EventKind::NotFound)
                } else {
                    Ok(uniforms)
                }
            }
            Err(e) => {
                println!("ERROR: {:?}", e);
                Err(EventKind::FindingFailed)
            }
        }
    }
    pub async fn find_many(
        query: &UniformQuery,
        db: &Database,
    ) -> Result<Vec<UniformResponse>, EventKind> {
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
        if let Some(uniform_id) = &query.uniform_id {
            queries.push(doc! {
                "$in": ["$_id", to_bson::<Vec<ObjectId>>(uniform_id).unwrap()]
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
            "$project": {
                "id": {
                    "$toString": "$_id"
                },
                "name": "$name",
                "attribute": "$attribute",
            }
        });

        match collection.aggregate(pipeline, None).await {
            Ok(mut cursor) => {
                let mut uniforms = Vec::new();
                while let Some(Ok(doc)) = cursor.next().await {
                    let uniform = from_document::<UniformResponse>(doc).unwrap();
                    uniforms.push(uniform);
                }
                if !uniforms.is_empty() {
                    Ok(uniforms)
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
