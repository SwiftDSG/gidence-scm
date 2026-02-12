use actix_web::{HttpResponse, get};

pub mod camera;
pub mod cluster;
pub mod evidence;
pub mod processor;
pub mod subscriber;
pub mod user;

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("PONG")
}
