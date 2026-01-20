use futures::StreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_bson},
    Database,
};
use serde::{Deserialize, Serialize};

use super::{
    camera::Camera,
    cluster::Cluster,
    event::EventKind,
    processor::Processor,
    uniform::{Uniform, UniformAttribute},
};

const COLLECTION: &str = "violations";

#[derive(Debug, Deserialize, Serialize)]
pub struct ViolationRequest {
    pub camera_id: String,
    pub uniform: Vec<ViolationUniformRequest>,
    pub timestamp: i64,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Violation {
    pub _id: ObjectId,
    pub cluster_id: ObjectId,
    pub processor_id: ObjectId,
    pub camera_id: ObjectId,
    pub uniform: Vec<ViolationUniform>,
    pub path: String,
    pub timestamp: i64,
    pub resolved: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ViolationResponse {
    pub id: String,
    pub processor_id: String,
    pub camera_id: String,
    pub cluster_id: String,
    pub uniform: Vec<ViolationUniform>,
    pub path: String,
    pub timestamp: i64,
    pub resolved: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViolationMinimalResponse {
    pub id: String,
    pub processor: String,
    pub camera: String,
    pub cluster: String,
    pub uniform: Vec<ViolationUniformResponse>,
    pub path: String,
    pub timestamp: i64,
    pub resolved: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViolationUniformRequest {
    pub uniform_id: String,
    pub bbox: (f64, f64, f64, f64),
    pub attribute: Vec<ViolationUniformAttribute>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViolationUniform {
    pub uniform_id: ObjectId,
    pub bbox: (f64, f64, f64, f64),
    pub attribute: Vec<ViolationUniformAttribute>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViolationUniformResponse {
    pub uniform: String,
    pub bbox: (f64, f64, f64, f64),
    pub attribute: Vec<ViolationUniformAttribute>, // (bbox, attribute)
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViolationUniformAttribute {
    pub kind: UniformAttribute,
    pub bbox: (f64, f64, f64, f64),
    pub violation: bool,
}

#[derive(Debug, Deserialize)]
pub struct ViolationQuery {
    pub cluster_id: Option<ObjectId>,
    pub processor_id: Option<ObjectId>,
    pub camera_id: Option<ObjectId>,
    pub date_minimum: Option<i64>,
    pub date_maximum: Option<i64>,
}

impl ViolationUniform {
    pub fn from(request: ViolationUniformRequest) -> Self {
        Self {
            uniform_id: ObjectId::parse_str(&request.uniform_id).unwrap(),
            bbox: request.bbox,
            attribute: request.attribute,
        }
    }
}
impl Violation {
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
    pub async fn delete_many(query: &ViolationQuery, db: &Database) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut queries = Vec::new();

        if let Some(cluster_id) = query.cluster_id {
            queries.push(doc! {
                "cluster_id": cluster_id
            });
        }
        if let Some(processor_id) = query.processor_id {
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
    pub async fn find_many_minimal(
        query: &ViolationQuery,
        db: &Database,
    ) -> Result<Vec<ViolationMinimalResponse>, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut pipeline = Vec::new();
        let mut queries = Vec::new();

        if let Some(cluster_id) = &query.cluster_id {
            queries.push(doc! {
                "$eq": ["$cluster_id", to_bson::<ObjectId>(cluster_id).unwrap()]
            });
        }
        if let Some(processor_id) = &query.processor_id {
            queries.push(doc! {
                "$eq": ["$processor_id", to_bson::<ObjectId>(processor_id).unwrap()]
            });
        }
        if let Some(camera_id) = &query.camera_id {
            queries.push(doc! {
                "$eq": ["$camera_id", to_bson::<ObjectId>(camera_id).unwrap()]
            });
        }
        if let Some(date) = &query.date_minimum {
            queries.push(doc! {
                "$gte": ["$timestamp", date]
            });
        }
        if let Some(date) = &query.date_maximum {
            queries.push(doc! {
                "$lte": ["$timestamp", date]
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
                "_id": "$_id",
                "processor_id": "$processor_id",
                "cluster_id": "$cluster_id",
                "camera_id": "$camera_id",
                "uniform": "$uniform",
                "path": "$path",
                "resolved": "$resolved",
                "timestamp": "$timestamp",
            }
        });
        pipeline.push(doc! {
            "$sort": {
                "resolved": 1,
            }
        });

        match collection.aggregate(pipeline, None).await {
            Ok(mut cursor) => {
                let mut violations = Vec::new();
                while let Some(Ok(doc)) = cursor.next().await {
                    let violation = from_document::<Violation>(doc).unwrap();
                    violations.push(ViolationMinimalResponse::from(&violation, db).await);
                }
                if !violations.is_empty() {
                    Ok(violations)
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

impl ViolationMinimalResponse {
    pub async fn from(a: &Violation, db: &Database) -> Self {
        let processor = match Processor::find_by_id(&a.processor_id, db).await {
            Ok(v) => v.name,
            _ => a.processor_id.to_string(),
        };
        let cluster = match Cluster::find_by_id(&a.cluster_id, db).await {
            Ok(v) => v.name,
            _ => a.cluster_id.to_string(),
        };
        let camera = match Camera::find_by_id(&a.camera_id, db).await {
            Ok(v) => v.name,
            _ => a.camera_id.to_string(),
        };

        let mut uniform = Vec::<ViolationUniformResponse>::new();
        for b in a.uniform.iter() {
            let u = match Uniform::find_by_id(&b.uniform_id, db).await {
                Ok(v) => v.name,
                _ => b.uniform_id.to_string(),
            };
            uniform.push(ViolationUniformResponse {
                uniform: u,
                bbox: b.bbox,
                attribute: b.attribute.clone(),
            });
        }
        Self {
            id: a._id.to_string(),
            processor,
            cluster,
            camera,
            uniform,
            path: a.path.clone(),
            timestamp: a.timestamp,
            resolved: a.resolved,
        }
    }
}
