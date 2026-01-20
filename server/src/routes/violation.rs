use std::{fs::File, io::Write, sync::Arc};

use actix_multipart::Multipart;
use actix_web::{get, post, put, web, HttpMessage, HttpRequest, HttpResponse};
use futures::StreamExt;
use mongodb::{bson::oid::ObjectId, Database};
use tokio::sync::Mutex;

use crate::{
    helper::error_handler,
    models::{
        processor::Processor,
        user::{User, UserAuthentication, UserRole},
        violation::{
            Violation, ViolationMinimalResponse, ViolationQuery, ViolationRequest, ViolationUniform,
        },
    },
};

#[post("/{processor_id}")]
pub async fn create_violation(
    processor_id: web::Path<String>,
    violations: web::Data<Arc<Mutex<Vec<Violation>>>>,
    mut payload: Multipart,
    db: web::Data<Database>,
) -> HttpResponse {
    let mut request = None;
    let mut path = None;

    let processor_id = match processor_id.parse() {
        Ok(v) => v,
        _ => {
            return HttpResponse::BadRequest().body("INVALID_ID");
        }
    };

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(v) => v,
            Err(e) => {
                continue;
            }
        };
        let content_disposition = match field.content_disposition() {
            Some(v) => v,
            None => {
                continue;
            }
        };

        let field_name = content_disposition.get_name().unwrap();

        if field_name == "file" {
            let file_name = format!("{}.jpg", ObjectId::new().to_string());
            let file_path = format!("./violations/{}", file_name);

            let mut f = match web::block(|| File::create(file_path)).await {
                Ok(Ok(v)) => v,
                Ok(Err(e)) => {
                    break;
                }
                Err(e) => {
                    break;
                }
            };

            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(v) => v,
                    Err(e) => {
                        break;
                    }
                };

                f = match web::block(move || f.write_all(&data).map(|_| f)).await {
                    Ok(Ok(v)) => v,
                    _ => {
                        break;
                    }
                }
            }

            path = Some(file_name);
        } else if field_name == "data" {
            let mut json_string = String::new();
            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(v) => v,
                    Err(e) => {
                        break;
                    }
                };
                json_string.push_str(&String::from_utf8_lossy(&data));
            }
            request = match serde_json::from_str::<Vec<ViolationRequest>>(&json_string) {
                Ok(v) => Some(v),
                Err(e) => {
                    break;
                }
            }
        }
    }

    let path = match path {
        Some(v) => v,
        _ => {
            return HttpResponse::BadRequest().finish();
        }
    };
    let mut request = match request {
        Some(v) => v,
        _ => {
            return HttpResponse::BadRequest().finish();
        }
    };

    let processor = match Processor::find_by_id(&processor_id, db.get_ref()).await {
        Ok(v) => v,
        Err(e) => return error_handler(e),
    };

    for request in request.drain(..) {
        let violation = Violation {
            _id: ObjectId::new(),
            processor_id,
            cluster_id: processor.cluster_id.clone(),
            camera_id: ObjectId::parse_str(request.camera_id).unwrap(),
            uniform: request
                .uniform
                .into_iter()
                .map(|u| ViolationUniform::from(u))
                .collect(),
            path: path.clone(),
            resolved: false,
            timestamp: request.timestamp,
        };
        match violation.save(db.get_ref()).await {
            Ok(_) => {
                let mut violations = violations.lock().await;
                (*violations).push(violation);
                drop(violations);
            }
            _ => continue,
        }
    }

    HttpResponse::Created().finish()
}
#[put("/{violation_id}")]
pub async fn update_violation(
    req: HttpRequest,
    violation_id: web::Path<String>,
    violations: web::Data<Arc<Mutex<Vec<Violation>>>>,
    db: web::Data<Database>,
) -> HttpResponse {
    let issuer = match req.extensions().get::<UserAuthentication>() {
        Some(issuer) => issuer.clone(),
        None => return HttpResponse::Unauthorized().body("UNAUTHORIZED"),
    };

    let violation_id = match violation_id.parse() {
        Ok(v) => v,
        _ => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    let mut violation = match Violation::find_by_id(&violation_id, db.get_ref()).await {
        Ok(v) => v,
        Err(e) => return error_handler(e),
    };

    if issuer.role != UserRole::SuperAdmin && issuer.role != UserRole::Manager {
        let user = match User::find_by_id(&issuer._id, db.get_ref()).await {
            Ok(v) => v,
            _ => return HttpResponse::Unauthorized().body("UNAUTHORIZED"),
        };

        if !user.cluster_id.contains(&violation.cluster_id) {
            return HttpResponse::Unauthorized().body("UNAUTHORIZED");
        }
    }

    violation.resolved = true;
    match violation.update(db.get_ref()).await {
        Ok(_) => {
            let payload = ViolationMinimalResponse::from(&violation, db.get_ref()).await;

            let mut violations = violations.lock().await;
            if let Some(index) = violations.iter_mut().position(|a| a._id == violation_id) {
                (*violations)[index] = violation
            }
            drop(violations);

            HttpResponse::Ok().json(payload)
        }
        Err(e) => error_handler(e),
    }
}
#[get("")]
pub async fn get_violations(
    query: web::Query<ViolationQuery>,
    db: web::Data<Database>,
) -> HttpResponse {
    match Violation::find_many_minimal(&query, db.get_ref()).await {
        Ok(violations) => HttpResponse::Ok().json(violations),
        Err(_) => HttpResponse::NotFound().body("NOT_FOUND"),
    }
}
