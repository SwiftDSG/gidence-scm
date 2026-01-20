use std::str::FromStr;

use futures::StreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_bson},
    Database,
};
use serde::{Deserialize, Serialize};

use super::{
    camera::{Camera, CameraMinimalResponse, CameraQuery, CameraResponse},
    cluster::Cluster,
    event::EventKind,
    uniform::UniformResponse,
    violation::{Violation, ViolationQuery},
};

const COLLECTION: &str = "processors";

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessorRequest {
    pub _id: ObjectId,
    pub cluster_id: ObjectId,
    pub address: ProcessorAddress,
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Processor {
    pub _id: ObjectId,
    pub cluster_id: ObjectId,
    pub address: ProcessorAddress,
    pub name: String,
    pub version: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessorResponse {
    pub id: String,
    pub cluster_id: String,
    pub address: ProcessorAddress,
    pub name: String,
    pub version: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessorMinimalResponse {
    pub id: String,
    pub cluster: String,
    pub address: ProcessorAddress,
    pub camera: Vec<CameraMinimalResponse>,
    pub name: String,
    pub version: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessorAddress {
    pub host: [u8; 4],
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct ProcessorQuery {
    pub cluster_id: Option<ObjectId>,
    pub date_minimum: Option<i64>,
    pub date_maximum: Option<i64>,
    pub text: Option<String>,
    pub limit: Option<usize>,
    pub skip: Option<usize>,
}
#[derive(Debug, Serialize)]
pub struct ProcessorData {
    pub processor: ProcessorResponse,
    pub camera: Vec<CameraResponse>,
    pub uniform: Vec<UniformResponse>,
}

impl From<ProcessorRequest> for Processor {
    fn from(a: ProcessorRequest) -> Self {
        Self {
            _id: a._id,
            cluster_id: a.cluster_id,
            address: a.address,
            name: a.name,
            version: ObjectId::new().to_string(),
        }
    }
}
impl From<Processor> for ProcessorResponse {
    fn from(a: Processor) -> Self {
        Self {
            id: a._id.to_string(),
            cluster_id: a.cluster_id.to_string(),
            address: a.address,
            name: a.name,
            version: a.version,
        }
    }
}

impl Processor {
    pub async fn save(&self, db: &Database) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        if collection.insert_one(self, None).await.is_ok() {
            Ok(())
        } else {
            Err(EventKind::SavingFailed)
        }
    }
    pub async fn delete(&self, db: &Database) -> Result<(), EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let _ = Violation::delete_many(
            &ViolationQuery {
                cluster_id: None,
                processor_id: Some(self._id),
                camera_id: None,
                date_minimum: None,
                date_maximum: None,
            },
            db,
        )
        .await;

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
    pub async fn find_many_minimal(
        query: &ProcessorQuery,
        db: &Database,
    ) -> Result<Vec<ProcessorMinimalResponse>, EventKind> {
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
                "$eq": ["$cluster_id", to_bson::<ObjectId>(cluster_id).unwrap()]
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
                "from": "clusters",
                "let": { "cluster_id": "$cluster_id" },
                "as": "cluster",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$eq": ["$_id", "$$cluster_id"]
                            }
                        }
                    }
                ]
            }
        });
        pipeline.push(doc! {
            "$project": {
                "id": {
                    "$toString": "$_id"
                },
                "cluster": {
                    "$cond": [
                        { "$first": "$cluster" },
                        { "$first": "$cluster.name" },
                        { "$toString": "$cluster_id" }
                    ]
                },
                "address": "$address",
                "camera": [],
                "name": "$name",
                "version": "$version",
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
                let mut processors = Vec::new();
                while let Some(Ok(doc)) = cursor.next().await {
                    let mut processor = from_document::<ProcessorMinimalResponse>(doc).unwrap();
                    if let Ok(camera) = Camera::find_many_minimal(
                        &CameraQuery {
                            cluster_id: query.cluster_id,
                            processor_id: Some(ObjectId::from_str(&processor.id).unwrap()),
                            date_minimum: query.date_minimum,
                            date_maximum: query.date_maximum,
                            text: None,
                            limit: None,
                            skip: None,
                        },
                        db,
                    )
                    .await
                    {
                        processor.camera = camera;
                    }
                    processors.push(processor);
                }
                if !processors.is_empty() {
                    Ok(processors)
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

impl ProcessorMinimalResponse {
    pub async fn from(a: Processor, db: &Database) -> Self {
        let cluster = match Cluster::find_by_id(&a.cluster_id, db).await {
            Ok(cluster) => cluster.name,
            Err(_) => a.cluster_id.to_string(),
        };
        let camera = match Camera::find_many_minimal(
            &CameraQuery {
                cluster_id: None,
                processor_id: Some(a._id),
                date_minimum: None,
                date_maximum: None,
                text: None,
                limit: None,
                skip: None,
            },
            db,
        )
        .await
        {
            Ok(camera) => camera,
            Err(_) => Vec::new(),
        };

        Self {
            id: a._id.to_string(),
            cluster,
            address: a.address,
            camera,
            name: a.name,
            version: a.version,
        }
    }
}
