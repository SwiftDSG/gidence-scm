use std::{
    collections::{HashMap, VecDeque},
    fs::{self, File},
    io::Write,
    sync::Arc,
};

use actix::{Addr, Recipient};
use actix_multipart::Multipart;
use actix_web::{HttpResponse, get, post, web};
use futures::StreamExt;
use mongodb::Database;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    central::{CentralWebSocket, CentralWebSocketMessage, CentralWebSocketResponse},
    helper::error_handler,
    models::{
        evidence::{Evidence, EvidenceQuery, EvidenceRequest},
        processor::Processor,
    },
    views::evidence::ViewEvidence,
};

#[post("/{processor_id}")]
pub async fn create_evidence(
    processor_id: web::Path<String>,
    mut payload: Multipart,
    db: web::Data<Database>,

    // In-memory evidence queue for notification distribution
    queue: web::Data<Arc<RwLock<VecDeque<Evidence>>>>,

    // Websocket client
    client: web::Data<
        Arc<RwLock<HashMap<Recipient<CentralWebSocketMessage>, (String, Addr<CentralWebSocket>)>>>,
    >,
) -> HttpResponse {
    let processor_id: String = processor_id.into_inner();

    // Verify processor exists
    let processor = match Processor::find_by_id(&processor_id, db.get_ref()).await {
        Ok(v) => v,
        Err(e) => return error_handler(e),
    };

    // Collect multipart fields
    let mut image_data: Option<Vec<u8>> = None;
    let mut evidence_data: Option<EvidenceRequest> = None;

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(v) => v,
            Err(_) => continue,
        };

        let field_name = match field.content_disposition() {
            Some(cd) => cd.get_name().unwrap_or("").to_string(),
            None => continue,
        };

        if field_name == "image" {
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                match chunk {
                    Ok(bytes) => data.extend_from_slice(&bytes),
                    Err(_) => break,
                }
            }
            if !data.is_empty() {
                image_data = Some(data);
            }
        } else if field_name == "data" {
            let mut json_string = String::new();
            while let Some(chunk) = field.next().await {
                match chunk {
                    Ok(bytes) => json_string.push_str(&String::from_utf8_lossy(&bytes)),
                    Err(_) => break,
                }
            }
            evidence_data = serde_json::from_str::<EvidenceRequest>(&json_string).ok();
        }
    }

    // Verify both fields are present
    let image_data = match image_data {
        Some(data) => data,
        None => return HttpResponse::BadRequest().body("MISSING_IMAGE"),
    };
    let evidence_data = match evidence_data {
        Some(req) => req,
        None => return HttpResponse::BadRequest().body("MISSING_DATA"),
    };

    // Generate evidence ID
    let _ = fs::create_dir_all("./evidence");
    let evidence_id = Uuid::new_v4().to_string();
    let file_path = format!("./evidence/{}.jpg", evidence_id);

    // Save image to ./evidences/{EVIDENCE_ID}.jpg
    let file_path_clone = file_path.clone();
    if let Err(_) = web::block(move || {
        let mut file = File::create(&file_path_clone)?;
        file.write_all(&image_data)
    })
    .await
    .map_err(|_| ())
    .and_then(|r| r.map_err(|_| ()))
    {
        return HttpResponse::InternalServerError().body("FAILED_TO_SAVE_IMAGE");
    }

    // Create evidence record
    let evidence = Evidence {
        id: evidence_id,
        cluster_id: processor.cluster_id.clone(),
        processor_id,
        camera_id: evidence_data.camera_id,
        frame_id: evidence_data.frame_id,
        timestamp: evidence_data.timestamp,
        person: evidence_data.person,
    };

    // Save to database
    match evidence.save(db.get_ref()).await {
        Ok(_) => {
            // Notify all connected clients about new evidence
            let payload = serde_json::to_string(&CentralWebSocketResponse::Evidence(
                ViewEvidence::from(evidence.clone(), db.get_ref()).await,
            ))
            .unwrap();

            let client = client.read().await;
            for (_, (_, client)) in (*client).iter() {
                client.do_send(CentralWebSocketMessage(payload.clone()));
            }

            {
                let mut evidence_queue = queue.write().await;
                evidence_queue.push_back(evidence);
            }

            HttpResponse::Created().finish()
        }
        Err(e) => {
            // Delete the image if database save fails
            let _ = fs::remove_file(&file_path);
            error_handler(e)
        }
    }
}

#[get("")]
pub async fn get_evidences(
    query: web::Query<EvidenceQuery>,
    db: web::Data<Database>,
) -> HttpResponse {
    match ViewEvidence::find_many(&query, db.get_ref()).await {
        Ok(evidences) => HttpResponse::Ok().json(evidences),
        Err(_) => HttpResponse::NotFound().body("NOT_FOUND"),
    }
}

#[get("/{evidence_id}")]
pub async fn get_evidence(evidence_id: web::Path<String>, db: web::Data<Database>) -> HttpResponse {
    let evidence_id = match evidence_id.parse() {
        Ok(v) => v,
        _ => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match ViewEvidence::find_by_id(&evidence_id, db.get_ref()).await {
        Ok(evidence) => HttpResponse::Ok().json(evidence),
        Err(_) => HttpResponse::NotFound().body("NOT_FOUND"),
    }
}
