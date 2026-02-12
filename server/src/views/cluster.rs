use futures::StreamExt;
use mongodb::{
    Database,
    bson::{Document, doc, from_document, to_bson},
};
use serde::{Deserialize, Serialize};

use crate::models::{
    cluster::{Cluster, ClusterQuery},
    event::EventKind,
};

const COLLECTION: &str = "clusters";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClusterRef {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ViewCluster {
    pub id: String,
    pub name: String,
    pub processor_count: usize,
    pub notification_count: usize,
    pub violation_count: usize,
}

impl ViewCluster {
    pub async fn find_many(query: &ClusterQuery, db: &Database) -> Result<Vec<Self>, EventKind> {
        let collection = db.collection::<Cluster>(COLLECTION);

        let mut cluster_query = Vec::new();
        let mut violation_query = Vec::from([doc! {
            "$eq": ["$cluster_id", "$$cluster_id"]
        }]);

        // Cluster filters
        if let Some(text) = &query.text {
            cluster_query.push(doc! {
                "$regexMatch": {
                    "input": "$name",
                    "options": "i",
                    "regex": text
                }
            });
        }
        if let Some(cluster_id) = &query.cluster_id {
            cluster_query.push(doc! {
                "$in": ["$id", to_bson::<Vec<String>>(cluster_id).unwrap()]
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
        let pipeline = vec![
            Self::create_match_stage(&cluster_query),
            Self::create_processor_count_stage(),
            Self::create_notification_count_stage(&violation_query),
            Self::create_violation_count_stage(&violation_query),
            Self::create_project_stage(),
        ];

        match collection.aggregate(pipeline, None).await {
            Ok(mut cursor) => {
                let mut clusters = Vec::new();
                while let Some(Ok(doc)) = cursor.next().await {
                    let cluster = from_document::<Self>(doc).unwrap();
                    clusters.push(Self::from(cluster));
                }
                if !clusters.is_empty() {
                    Ok(clusters)
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
    pub async fn find_one(query: &ClusterQuery, db: &Database) -> Result<Self, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut cluster_query = Vec::new();
        let mut violation_query = Vec::from([doc! {
            "$eq": ["$cluster_id", "$$cluster_id"]
        }]);

        // Cluster filters
        if let Some(text) = &query.text {
            cluster_query.push(doc! {
                "$regexMatch": {
                    "input": "$name",
                    "options": "i",
                    "regex": to_bson::<String>(text).unwrap()
                }
            });
        }
        if let Some(cluster_id) = &query.cluster_id {
            cluster_query.push(doc! {
                "$in": ["$id", to_bson::<Vec<String>>(cluster_id).unwrap()]
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
        let pipeline = vec![
            Self::create_match_stage(&cluster_query),
            Self::create_processor_count_stage(),
            Self::create_notification_count_stage(&violation_query),
            Self::create_violation_count_stage(&violation_query),
            Self::create_project_stage(),
        ];

        match collection.aggregate(pipeline, None).await {
            Ok(mut cursor) => {
                if let Some(Ok(doc)) = cursor.next().await {
                    let cluster = from_document::<Self>(doc).unwrap();
                    Ok(cluster)
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
    fn create_processor_count_stage() -> Document {
        doc! {
            "$lookup": {
                "from": "processors",
                "let": { "cluster_id": "$id" },
                "as": "processor",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$eq": ["$cluster_id", "$$cluster_id"]
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
    fn create_notification_count_stage(query: &Vec<Document>) -> Document {
        doc! {
            "$lookup": {
                "from": "evidences",
                "let": { "cluster_id": "$id" },
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
                "let": { "cluster_id": "$id" },
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
                "name": "$name",
                "processor_count": {
                    "$cond": [
                        { "$first": "$processor" },
                        { "$first": "$processor.count" },
                        0
                    ]
                },
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
