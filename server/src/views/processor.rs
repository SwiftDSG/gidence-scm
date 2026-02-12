use futures::StreamExt;
use mongodb::{
    Database,
    bson::{Document, doc, from_document, to_bson},
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        camera::CameraQuery,
        event::EventKind,
        processor::{ProcessorAddress, ProcessorQuery},
    },
    views::{camera::ViewCamera, cluster::ClusterRef},
};

const COLLECTION: &str = "processors";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessorRef {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ViewProcessor {
    pub id: String,
    pub cluster: ClusterRef,
    pub camera: Vec<ViewCamera>,
    pub address: ProcessorAddress,
    pub name: String,
    pub notification_count: usize,
    pub violation_count: usize,
}

impl ViewProcessor {
    pub async fn find_many(query: &ProcessorQuery, db: &Database) -> Result<Vec<Self>, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut processor_query = Vec::new();
        let mut violation_query = Vec::from([doc! {
            "$eq": ["$processor_id", "$$processor_id"]
        }]);

        // Processor filters
        if let Some(text) = &query.text {
            processor_query.push(doc! {
                "$regexMatch": {
                    "input": "$name",
                    "options": "i",
                    "regex": text
                }
            });
        }
        if let Some(cluster_id) = &query.cluster_id {
            processor_query.push(doc! {
                "$eq": ["$cluster_id", cluster_id]
            });
        }
        if let Some(processor_id) = &query.processor_id {
            processor_query.push(doc! {
                "$eq": ["$id", processor_id]
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

        let mut pipeline = vec![
            Self::create_match_stage(&processor_query),
            Self::create_cluster_lookup_stage(),
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
                let mut processors = Vec::new();
                while let Some(Ok(doc)) = cursor.next().await {
                    let mut processor = from_document::<Self>(doc).unwrap();
                    if let Ok(camera) = ViewCamera::find_many(
                        &CameraQuery {
                            cluster_id: query.cluster_id.clone(),
                            processor_id: Some(processor.id.clone()),
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
    pub async fn find_one(query: &ProcessorQuery, db: &Database) -> Result<Self, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut processor_query = Vec::new();
        let mut violation_query = Vec::from([doc! {
            "$eq": ["$processor_id", "$$processor_id"]
        }]);

        // Processor filters
        if let Some(text) = &query.text {
            processor_query.push(doc! {
                "$regexMatch": {
                    "input": "$name",
                    "options": "i",
                    "regex": text
                }
            });
        }
        if let Some(cluster_id) = &query.cluster_id {
            processor_query.push(doc! {
                "$eq": ["$cluster_id", cluster_id]
            });
        }
        if let Some(processor_id) = &query.processor_id {
            processor_query.push(doc! {
                "$eq": ["$id", processor_id]
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

        let pipeline = vec![
            Self::create_match_stage(&processor_query),
            Self::create_cluster_lookup_stage(),
            Self::create_notification_count_stage(&violation_query),
            Self::create_violation_count_stage(&violation_query),
            Self::create_project_stage(),
        ];

        match collection.aggregate(pipeline, None).await {
            Ok(mut cursor) => {
                if let Some(Ok(doc)) = cursor.next().await {
                    let mut processor = from_document::<Self>(doc).unwrap();
                    if let Ok(camera) = ViewCamera::find_many(
                        &CameraQuery {
                            cluster_id: None,
                            processor_id: Some(processor.id.clone()),
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
                        processor.camera = camera;
                    }
                    Ok(processor)
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
                    }
                ]
            }
        }
    }
    fn create_notification_count_stage(query: &Vec<Document>) -> Document {
        doc! {
            "$lookup": {
                "from": "evidences",
                "let": { "processor_id": "$id" },
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
                "let": { "processor_id": "$id" },
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
                        "$unwind": "$person.violation"
                    },
                    {
                        "$count": "count"
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
                "address": "$address",
                "camera": [],
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
                },
            }
        }
    }
}
