use actix_web::{delete, get, post, web, HttpResponse};
use mongodb::{bson::oid::ObjectId, Database};

use crate::{
    helper::error_handler,
    models::{
        camera::{Camera, CameraQuery, CameraRequest, CameraResponse},
        cluster::Cluster,
        processor::Processor,
    },
};

#[post("")]
pub async fn create_camera(
    payload: web::Json<CameraRequest>,
    db: web::Data<Database>,
) -> HttpResponse {
    let request = payload.into_inner();

    if (Cluster::find_by_id(&request.cluster_id, db.get_ref()).await).is_err() {
        return HttpResponse::NotFound().body("NOT_FOUND");
    }
    let mut processor = match Processor::find_by_id(&request.processor_id, db.get_ref()).await {
        Ok(processor) => processor,
        Err(_) => return HttpResponse::NotFound().body("NOT_FOUND"),
    };

    let camera = Camera::from(request);

    match camera.save(db.get_ref()).await {
        Ok(()) => {
            processor.version = ObjectId::new().to_string();
            let _ = processor.save(db.get_ref()).await;
            HttpResponse::Created().json(CameraResponse::from(camera))
        }
        Err(e) => error_handler(e),
    }
}

#[delete("/{camera_id}")]
pub async fn delete_camera(camera_id: web::Path<String>, db: web::Data<Database>) -> HttpResponse {
    let camera_id = match camera_id.parse() {
        Ok(camera_id) => camera_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match Camera::find_by_id(&camera_id, db.get_ref()).await {
        Ok(camera) => {
            let _ = camera.delete(db.get_ref()).await;

            HttpResponse::NoContent().finish()
        }
        Err(e) => error_handler(e),
    }
}

#[get("")]
pub async fn get_cameras(query: web::Query<CameraQuery>, db: web::Data<Database>) -> HttpResponse {
    match Camera::find_many_minimal(&query, db.get_ref()).await {
        Ok(cameras) => HttpResponse::Ok().json(cameras),
        Err(e) => error_handler(e),
    }
}
