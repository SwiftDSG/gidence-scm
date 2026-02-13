use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse, get, web};
use tokio::sync::RwLock;

use crate::models::{Device, Reading};

pub mod camera;
pub mod processor;

#[get("/reading")]
pub async fn get_reading(reading: web::Data<Arc<RwLock<Reading>>>) -> HttpResponse {
    let reading = reading.read().await;
    HttpResponse::Ok().json(&*reading)
}
#[get("/device")]
pub async fn get_device(device: web::Data<Arc<RwLock<Device>>>) -> HttpResponse {
    let device = {
        let device = device.read().await;
        device.clone()
    };

    let processor = device.processor;
    let camera = device.camera.values().collect::<Vec<_>>();
    // FORM A JSON OBJECT
    let device_json = serde_json::json!({
        "processor": processor,
        "camera": camera,
    });
    HttpResponse::Ok().json(device_json)
}
#[get("/frame/{camera_id}")]
pub async fn get_frame(camera_id: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let camera_id = camera_id.into_inner();

    NamedFile::open(&format!("/tmp/{}.jpg", camera_id))
        .map(|file| file.into_response(&req))
        .unwrap_or_else(|_| HttpResponse::NotFound().finish())
}

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ping)
        .service(get_reading)
        .service(get_device)
        .service(get_frame)
        .service(
            web::scope("/processor")
                .service(processor::get_processor)
                .service(processor::update_processor),
        )
        .service(
            web::scope("/camera")
                .service(camera::get_cameras)
                .service(camera::create_camera)
                .service(camera::update_camera)
                .service(camera::delete_camera),
        );
}
