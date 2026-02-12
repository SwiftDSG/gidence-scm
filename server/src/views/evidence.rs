use futures::StreamExt;
use mongodb::{
    Database,
    bson::{Document, doc, from_document},
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        camera::Camera,
        cluster::Cluster,
        event::EventKind,
        evidence::{Evidence, EvidencePerson, EvidenceQuery},
        processor::Processor,
    },
    views::{camera::CameraRef, cluster::ClusterRef, processor::ProcessorRef},
};

const COLLECTION: &str = "evidences";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewEvidence {
    pub id: String,
    pub cluster: ClusterRef,
    pub processor: ProcessorRef,
    pub camera: CameraRef,
    pub timestamp: i64,
    pub person: Vec<EvidencePerson>,
}

impl ViewEvidence {
    pub async fn from(evidence: Evidence, db: &Database) -> Self {
        let cluster = match Cluster::find_by_id(&evidence.cluster_id, db).await {
            Ok(v) => ClusterRef {
                id: v.id,
                name: v.name,
            },
            Err(_) => ClusterRef {
                id: evidence.cluster_id.clone(),
                name: evidence.cluster_id.clone(),
            },
        };
        let processor = match Processor::find_by_id(&evidence.processor_id, db).await {
            Ok(v) => ProcessorRef {
                id: v.id,
                name: v.name,
            },
            Err(_) => ProcessorRef {
                id: evidence.processor_id.clone(),
                name: evidence.processor_id.clone(),
            },
        };
        let camera = match Camera::find_by_id(&evidence.camera_id, db).await {
            Ok(v) => CameraRef {
                id: v.id,
                name: v.name,
            },
            Err(_) => CameraRef {
                id: evidence.camera_id.clone(),
                name: evidence.camera_id.clone(),
            },
        };

        ViewEvidence {
            id: evidence.id,
            cluster,
            processor,
            camera,
            timestamp: evidence.timestamp,
            person: evidence.person,
        }
    }
    pub async fn find_many(query: &EvidenceQuery, db: &Database) -> Result<Vec<Self>, EventKind> {
        let collection = db.collection::<Self>(COLLECTION);

        let mut evidence_query = Vec::new();

        if let Some(cluster_id) = &query.cluster_id {
            evidence_query.push(doc! {
                "$eq": ["$cluster_id", &cluster_id]
            });
        }
        if let Some(processor_id) = &query.processor_id {
            evidence_query.push(doc! {
                "$eq": ["$processor_id", &processor_id]
            });
        }
        if let Some(camera_id) = &query.camera_id {
            evidence_query.push(doc! {
                "$eq": ["$camera_id", &camera_id]
            });
        }
        if let Some(date) = &query.date_minimum {
            evidence_query.push(doc! {
                "$gte": ["$timestamp", date]
            });
        }
        if let Some(date) = &query.date_maximum {
            evidence_query.push(doc! {
                "$lte": ["$timestamp", date]
            });
        }

        let pipeline = vec![
            Self::create_match_stage(&evidence_query),
            Self::create_cluster_lookup_stage(),
            Self::create_processor_lookup_stage(),
            Self::create_camera_lookup_stage(),
            Self::create_project_stage(),
        ];

        match collection.aggregate(pipeline, None).await {
            Ok(mut cursor) => {
                let mut evidences = Vec::new();
                while let Some(Ok(doc)) = cursor.next().await {
                    let evidence = from_document::<ViewEvidence>(doc).unwrap();
                    evidences.push(evidence);
                }
                if !evidences.is_empty() {
                    Ok(evidences)
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
                    }
                ]
            }
        }
    }
    fn create_camera_lookup_stage() -> Document {
        doc! {
            "$lookup": {
                "from": "cameras",
                "let": { "camera_id": "$camera_id" },
                "as": "camera",
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$eq": ["$id", "$$camera_id"]
                            }
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
                "camera": {
                    "$cond": [
                        { "$first": "$camera" },
                        { "$first": "$camera" },
                        {
                            "id": "$camera_id",
                            "name": "$camera_id"
                        }
                    ]
                },
                "timestamp": "$timestamp",
                "person": "$person",
            }
        }
    }
}
