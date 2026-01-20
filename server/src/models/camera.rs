use futures::StreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_bson},
    Database,
};
use serde::{Deserialize, Serialize};

use super::{
    event::EventKind,
    violation::{Violation, ViolationQuery},
};

const COLLECTION: &str = "cameras";

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraRequest {
    pub cluster_id: ObjectId,
    pub processor_id: ObjectId,
    pub address: CameraAddress,
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Camera {
    pub _id: ObjectId,
    pub cluster_id: ObjectId,
    pub processor_id: ObjectId,
    pub address: CameraAddress,
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CameraResponse {
    pub id: String,
    pub cluster_id: String,
    pub processor_id: String,
    pub address: CameraAddress,
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CameraMinimalResponse {
    pub id: String,
    pub cluster: String,
    pub processor: String,
    pub address: CameraAddress,
    pub name: String,
    pub notification_count: usize,
    pub violation_count: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraAddress {
    pub host: [u8; 4],
    pub port: u16,
    pub path: Option<String>,
    pub authentication: Option<(String, String)>,
}
#[derive(Debug, Deserialize)]
pub struct CameraQuery {
    pub cluster_id: Option<ObjectId>,
    pub processor_id: Option<ObjectId>,
    pub date_minimum: Option<i64>,
    pub date_maximum: Option<i64>,
    pub text: Option<String>,
    pub limit: Option<usize>,
    pub skip: Option<usize>,
}

impl From<CameraRequest> for Camera {
    fn from(a: CameraRequest) -> Self {
        Self {
            _id: ObjectId::new(),
            cluster_id: a.cluster_id,
            processor_id: a.processor_id,
            address: a.address,
            name: a.name,
        }
    }
}
impl From<Camera> for CameraResponse {
    fn from(a: Camera) -> Self {
        Self {
            id: a._id.to_string(),
            cluster_id: a.cluster_id.to_string(),
            processor_id: a.processor_id.to_string(),
            address: a.address,
            name: a.name,
        }
    }
}

impl Camera {
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
            &&ViolationQuery {
                cluster_id: None,
                processor_id: None,
                camera_id: Some(self._id),
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
    pub async fn find_many(
        query: &CameraQuery,
        db: &Database,
    ) -> Result<Vec<CameraResponse>, EventKind> {
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
        if let Some(processor_id) = &query.processor_id {
            queries.push(doc! {
                "$eq": ["$processor_id", to_bson::<ObjectId>(processor_id).unwrap()]
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
                    let camera = from_document::<Camera>(doc).unwrap();
                    cameras.push(CameraResponse::from(camera));
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

    pub async fn find_many_minimal(
        query: &CameraQuery,
        db: &Database,
    ) -> Result<Vec<CameraMinimalResponse>, EventKind> {
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
        if let Some(processor_id) = &query.processor_id {
            queries.push(doc! {
                "$eq": ["$processor_id", to_bson::<ObjectId>(processor_id).unwrap()]
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
            "$eq": ["$camera_id", "$$camera_id"]
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
                "let": { "camera_id": "$_id" },
                "as": "notification_count",
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
                "let": { "camera_id": "$_id" },
                "as": "violation_count",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$and": violation_query
                            }
                        }
                    },
                    {
                        "$unwind": "$uniform"
                    },
                    {
                        "$count": "count"
                    }
                ]
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
            "$lookup": {
                "from": "processors",
                "let": { "processor_id": "$processor_id" },
                "as": "processor",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$eq": ["$_id", "$$processor_id"]
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
                "processor": {
                    "$cond": [
                        { "$first": "$processor" },
                        { "$first": "$processor.name" },
                        { "$toString": "$processor_id" }
                    ]
                },
                "address": "$address",
                "name": "$name",
                "notification_count": {
                    "$cond": [
                        { "$first": "$notification_count" },
                        { "$first": "$notification_count.count" },
                        0
                    ]
                },
                "violation_count": {
                    "$cond": [
                        { "$first": "$violation_count" },
                        { "$first": "$violation_count.count" },
                        0
                    ]
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
                    let camera = from_document::<CameraMinimalResponse>(doc).unwrap();
                    cameras.push(camera);
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
