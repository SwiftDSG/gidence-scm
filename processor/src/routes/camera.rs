use std::sync::Arc;

use actix_web::{HttpResponse, delete, get, post, put, web};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::{Device, camera::Camera};

#[post("")]
pub async fn create_camera(
    payload: web::Json<Camera>,
    device: web::Data<Arc<RwLock<Device>>>,
) -> HttpResponse {
    let mut new_camera = payload.into_inner();
    new_camera.id = Uuid::new_v4().to_string();

    let mut device = device.write().await;
    if device.camera.get(&new_camera.id).is_some() {
        drop(device);
        return HttpResponse::Conflict().finish();
    }

    new_camera.insert_one();
    device
        .camera
        .insert(new_camera.id.clone(), new_camera.clone());
    device.processor.update_version(); // Update processor version on change
    drop(device);
    HttpResponse::Ok().json(new_camera)
}

#[put("")]
pub async fn update_camera(
    payload: web::Json<Camera>,
    device: web::Data<Arc<RwLock<Device>>>,
) -> HttpResponse {
    let new_camera = payload.into_inner();

    let mut device = device.write().await;
    if let Some(camera) = device.camera.get_mut(&new_camera.id) {
        *camera = new_camera.clone();
        Camera::insert_many(
            &device
                .camera
                .values()
                .cloned()
                .into_iter()
                .collect::<Vec<Camera>>(),
        );
        device.processor.update_version(); // Update processor version on change
        HttpResponse::Ok().json(new_camera)
    } else {
        drop(device);
        HttpResponse::NotFound().finish()
    }
}

#[get("")]
pub async fn get_cameras(device: web::Data<Arc<RwLock<Device>>>) -> HttpResponse {
    let cameras = {
        let device = device.read().await;
        device.camera.values().cloned().collect::<Vec<Camera>>()
    };
    HttpResponse::Ok().json(cameras)
}

#[delete("/{id}")]
pub async fn delete_camera(
    id: web::Path<String>,
    device: web::Data<Arc<RwLock<Device>>>,
) -> HttpResponse {
    let id = id.into_inner();

    let mut device = device.write().await;
    if device.camera.remove(&id).is_some() {
        Camera::insert_many(
            &device
                .camera
                .values()
                .cloned()
                .into_iter()
                .collect::<Vec<Camera>>(),
        );
        device.processor.update_version(); // Update processor version on change
        HttpResponse::NoContent().finish()
    } else {
        drop(device);
        HttpResponse::NotFound().finish()
    }
}
