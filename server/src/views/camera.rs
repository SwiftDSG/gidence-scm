use futures::StreamExt;
use mongodb::{
    Database,
    bson::{Document, doc, from_document, to_bson},
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        camera::{CameraAddress, CameraQuery},
        event::EventKind,
        evidence::{Evidence, EvidenceQuery},
    },
    views::{cluster::ClusterRef, processor::ProcessorRef},
};

const COLLECTION: &str = "cameras";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CameraRef {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ViewCamera {
    pub id: String,
    pub cluster: ClusterRef,
    pub processor: ProcessorRef,
    pub address: CameraAddress,
    pub name: String,
    pub notification_count: usize,
    pub violation_count: usize,
}

impl ViewCamera {
    pub async fn find_many(query: &CameraQuery, db: &Database) -> Result<Vec<Self>, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut camera_query = Vec::new();
        let mut violation_query = Vec::from([doc! {
            "$eq": ["$camera_id", "$$camera_id"]
        }]);

        // Camera filters
        if let Some(text) = &query.text {
            camera_query.push(doc! {
                "$regexMatch": {
                    "input": "$name",
                    "options": "i",
                    "regex": to_bson::<String>(text).unwrap()
                }
            });
        }
        if let Some(cluster_id) = &query.cluster_id {
            camera_query.push(doc! {
                "$eq": ["$cluster_id", to_bson::<String>(cluster_id).unwrap()]
            });
        }
        if let Some(processor_id) = &query.processor_id {
            camera_query.push(doc! {
                "$eq": ["$processor_id", to_bson::<String>(processor_id).unwrap()]
            });
        }

        // Violation filters
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

        // Build aggregation pipeline
        let mut pipeline = vec![
            Self::create_match_stage(&camera_query),
            Self::create_cluster_lookup_stage(),
            Self::create_processor_lookup_stage(),
            Self::create_notification_count_stage(&violation_query),
            Self::create_violation_count_stage(&violation_query),
            Self::create_project_stage(),
        ];

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

    // Helper functions to create aggregation stages
    fn create_match_stage(query: &Vec<Document>) -> Document {
        doc! {
            "$match": {
                "$expr": {
                    "$and": query
                }
            }
        }
    }
    fn create_notification_count_stage(query: &Vec<Document>) -> Document {
        doc! {
            "$lookup": {
                "from": "evidences",
                "let": { "camera_id": "$id" },
                "as": "notification",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$and": query
                            }
                        }
                    },
                    {
                        "$count": "count"
                    }
                ]
            }
        }
    }
    fn create_violation_count_stage(query: &Vec<Document>) -> Document {
        doc! {
            "$lookup": {
                "from": "evidences",
                "let": { "camera_id": "$id" },
                "as": "violation",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$and": query
                            }
                        }
                    },
                    // Unwind persons
                    {
                        "$unwind": "$person"
                    },
                    // Filter persons with empty violations
                    {
                        "$match": {
                            "$expr": {
                                "$gt": [ { "$size": "$person.violation" }, 0 ]
                            }
                        }
                    },
                    // Count the number of documents
                    {
                        "$unwind": "$violation"
                    },
                    {
                        "$count": "count"
                    }
                ]
            }
        }
    }
    fn create_cluster_lookup_stage() -> Document {
        doc! {
            "$lookup": {
                "from": "clusters",
                "let": { "cluster_id": "$cluster_id" },
                "as": "cluster",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$eq": ["$id", "$$cluster_id"]
                            }
                        }
                    },
                    {
                        "$project": {
                            "id": "$id",
                            "name": "$name"
                        }
                    }
                ]
            }
        }
    }
    fn create_processor_lookup_stage() -> Document {
        doc! {
            "$lookup": {
                "from": "processors",
                "let": { "processor_id": "$processor_id" },
                "as": "processor",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$eq": ["$id", "$$processor_id"]
                            }
                        }
                    },
                    {
                        "$project": {
                            "id": "$id",
                            "name": "$name"
                        }
                    }
                ]
            }
        }
    }
    fn create_project_stage() -> Document {
        doc! {
            "$project": {
                "id": "$id",
                "cluster": {
                    "$cond": [
                        { "$first": "$cluster" },
                        { "$first": "$cluster" },
                        {
                            "id": "$cluster_id",
                            "name": "$cluster_id"
                        }
                    ]
                },
                "processor": {
                    "$cond": [
                        { "$first": "$processor" },
                        { "$first": "$processor" },
                        {
                            "id": "$processor_id",
                            "name": "$processor_id"
                        }
                    ]
                },
                "address": "$address",
                "name": "$name",
                "notification_count": {
                    "$cond": [
                        { "$first": "$notification" },
                        { "$first": "$notification.count" },
                        0
                    ]
                },
                "violation_count": {
                    "$cond": [
                        { "$first": "$violation" },
                        { "$first": "$violation.count" },
                        0
                    ]
                }
            }
        }
    }
}
