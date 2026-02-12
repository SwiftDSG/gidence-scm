use actix_web::{HttpResponse, get, web};
use mongodb::Database;

use crate::{helper::error_handler, models::camera::CameraQuery, views::camera::ViewCamera};

#[get("")]
pub async fn get_cameras(query: web::Query<CameraQuery>, db: web::Data<Database>) -> HttpResponse {
    match ViewCamera::find_many(&query, db.get_ref()).await {
        Ok(cameras) => HttpResponse::Ok().json(cameras),
        Err(e) => error_handler(e),
    }
}
