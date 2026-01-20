use actix_web::{get, HttpResponse};

pub mod camera;
pub mod cluster;
pub mod processor;
pub mod subscriber;
pub mod uniform;
pub mod user;
pub mod violation;

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("PONG")
}
