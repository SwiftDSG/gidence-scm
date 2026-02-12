use std::{collections::HashMap, sync::Arc};

use actix_web::{HttpResponse, delete, get, post, put, web};
use chrono::Local;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    helper::error_handler,
    models::{
        camera::{Camera, CameraQuery, CameraRequest},
        cluster::Cluster,
        processor::{Processor, ProcessorQuery, ProcessorRequest},
    },
    views::processor::ViewProcessor,
};

#[derive(Debug, Deserialize, Serialize)]
struct ProcessorSynchronization {
    processor: ProcessorRequest,
    camera: Vec<CameraRequest>,
}

#[delete("/{processor_id}")]
pub async fn delete_processor(
    processor_id: web::Path<String>,
    db: web::Data<Database>,
) -> HttpResponse {
    let processor_id = match processor_id.parse() {
        Ok(processor_id) => processor_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match Processor::find_by_id(&processor_id, db.get_ref()).await {
        Ok(processor) => {
            let _ = processor.delete(db.get_ref()).await;

            HttpResponse::NoContent().finish()
        }
        Err(e) => error_handler(e),
    }
}

#[get("")]
pub async fn get_processors(
    query: web::Query<ProcessorQuery>,
    db: web::Data<Database>,
) -> HttpResponse {
    match ViewProcessor::find_many(&query, db.get_ref()).await {
        Ok(processors) => HttpResponse::Ok().json(processors),
        Err(e) => error_handler(e),
    }
}

#[get("/{processor_id}")]
pub async fn get_processor(
    processor_id: web::Path<String>,
    query: web::Query<ProcessorQuery>,
    db: web::Data<Database>,
) -> HttpResponse {
    let processor_id = match processor_id.parse() {
        Ok(v) => v,
        _ => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    let mut query = query.into_inner();
    query.processor_id = Some(processor_id);

    match ViewProcessor::find_one(&query, db.get_ref()).await {
        Ok(processor) => HttpResponse::Ok().json(processor),
        Err(e) => error_handler(e),
    }
}

#[post("/{cluster_id}")]
pub async fn sync_processor(
    cluster_id: web::Path<String>,
    payload: web::Json<ProcessorSynchronization>,
    processor_online: web::Data<Arc<RwLock<HashMap<String, i64>>>>,
    db: web::Data<Database>,
) -> HttpResponse {
    let cluster_id = match cluster_id.parse() {
        Ok(v) => v,
        _ => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    if (Cluster::find_by_id(&cluster_id, db.get_ref()).await).is_err() {
        return HttpResponse::NotFound().body("NOT_FOUND");
    }

    let mut processor = match Processor::find_by_id(&payload.processor.id, db.get_ref()).await {
        Ok(v) => {
            // Saved version is newer or equal, no need to update
            if v.version >= payload.processor.version {
                {
                    // Update processor online timestamp
                    let mut processor_map = processor_online.write().await;
                    processor_map.insert(
                        payload.processor.id.clone(),
                        Local::now().timestamp_millis() + 30000,
                    );
                }
                return HttpResponse::NoContent().finish();
            }
            v
        }
        Err(_) => {
            let processor = Processor {
                id: payload.processor.id.clone(),
                cluster_id: cluster_id.clone(),
                name: payload.processor.name.clone(),
                model: payload.processor.model.clone(),
                address: payload.processor.address.clone(),
                version: payload.processor.version.clone(),
            };

            match processor.save(db.get_ref()).await {
                Ok(()) => processor,
                Err(e) => return error_handler(e),
            }
        }
    };

    // Update cameras
    let camera = match Camera::find_many(
        &CameraQuery {
            cluster_id: None,
            processor_id: Some(payload.processor.id.clone()),
            date_minimum: None,
            date_maximum: None,
            text: None,
            limit: None,
            skip: None,
        },
        db.get_ref(),
    )
    .await
    {
        Ok(v) => v,
        Err(_) => Vec::new(),
    };

    // Filter which cameras to delete by comparing existing cameras with payload cameras, delete those not present in payload
    let mut cameras_to_delete = camera
        .into_iter()
        .filter(|c| !payload.camera.iter().any(|pc| pc.id == c.id))
        .collect::<Vec<Camera>>();

    for camera in cameras_to_delete.drain(..) {
        let _ = camera.delete(db.get_ref()).await;
    }
    for camera in &payload.camera {
        let camera = Camera {
            id: camera.id.clone(),
            cluster_id: cluster_id.clone(),
            processor_id: payload.processor.id.clone(),
            name: camera.name.clone(),
            address: camera.address.clone(),
        };
        let _ = camera.save(db.get_ref()).await;
    }

    processor.name = payload.processor.name.clone();
    processor.model = payload.processor.model.clone();
    processor.address = payload.processor.address.clone();
    processor.version = payload.processor.version.clone();

    match processor.update(db.get_ref()).await {
        Ok(()) => {
            {
                // Update processor online timestamp
                let mut processor_map = processor_online.write().await;
                processor_map.insert(
                    payload.processor.id.clone(),
                    Local::now().timestamp_millis() + 30000,
                );
            }
            HttpResponse::Ok().json(
                ViewProcessor::find_one(
                    &ProcessorQuery {
                        processor_id: Some(payload.processor.id.clone()),
                        cluster_id: None,
                        date_minimum: None,
                        date_maximum: None,
                        text: None,
                        limit: None,
                        skip: None,
                    },
                    db.get_ref(),
                )
                .await
                .unwrap(),
            )
        }
        Err(e) => error_handler(e),
    }
}

#[put("/{processor_id}")]
pub async fn update_processor(
    processor_id: web::Path<String>,
    payload: web::Json<ProcessorRequest>,
    db: web::Data<Database>,
) -> HttpResponse {
    let processor_id = match processor_id.parse() {
        Ok(processor_id) => processor_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };
    let mut processor = match Processor::find_by_id(&processor_id, db.get_ref()).await {
        Ok(v) => v,
        Err(e) => return error_handler(e),
    };

    let request = payload.into_inner();

    processor.name = request.name;
    processor.model = request.model;
    processor.address = request.address;
    processor.version = Local::now().timestamp_millis();

    match processor.update(db.get_ref()).await {
        Ok(()) => HttpResponse::Created().json(
            ViewProcessor::find_one(
                &ProcessorQuery {
                    processor_id: Some(processor_id.clone()),
                    cluster_id: None,
                    date_minimum: None,
                    date_maximum: None,
                    text: None,
                    limit: None,
                    skip: None,
                },
                db.get_ref(),
            )
            .await
            .unwrap(),
        ),
        Err(e) => error_handler(e),
    }
}
