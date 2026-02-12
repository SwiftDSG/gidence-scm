use std::sync::Arc;

use actix_web::{HttpResponse, get, put, web};
use tokio::sync::RwLock;

use crate::models::{Device, processor::Processor};

#[put("")]
pub async fn update_processor(
    payload: web::Json<Processor>,
    device: web::Data<Arc<RwLock<Device>>>,
) -> HttpResponse {
    let new_processor = payload.into_inner();

    new_processor.update();

    let mut device = device.write().await;
    (*device).processor = new_processor.clone();
    device.processor.update_version(); // Update processor version on change

    drop(device);

    HttpResponse::Ok().json(new_processor)
}

#[get("")]
pub async fn get_processor(device: web::Data<Arc<RwLock<Device>>>) -> HttpResponse {
    let device = device.read().await;
    let processor = device.processor.clone();
    drop(device);
    HttpResponse::Ok().json(processor)
}
