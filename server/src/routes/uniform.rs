use actix_web::{delete, get, post, put, web, HttpResponse};
use mongodb::Database;

use crate::{
    helper::error_handler,
    models::uniform::{Uniform, UniformQuery, UniformRequest, UniformResponse},
};

#[post("")]
pub async fn create_uniform(
    payload: web::Json<UniformRequest>,
    db: web::Data<Database>,
) -> HttpResponse {
    let request = payload.into_inner();

    let uniform = Uniform::from(request);

    match uniform.save(db.get_ref()).await {
        Ok(()) => HttpResponse::Created().json(UniformResponse::from(uniform)),
        Err(e) => error_handler(e),
    }
}

#[get("")]
pub async fn get_uniforms(
    query: web::Query<UniformQuery>,
    db: web::Data<Database>,
) -> HttpResponse {
    let query = query.into_inner();
    match Uniform::find_many(&query, db.get_ref()).await {
        Ok(uniforms) => HttpResponse::Ok().json(uniforms),
        Err(e) => error_handler(e),
    }
}

#[put("/{uniform_id}")]
pub async fn update_uniform(
    uniform_id: web::Path<String>,
    payload: web::Json<UniformRequest>,
    db: web::Data<Database>,
) -> HttpResponse {
    let uniform_id = match uniform_id.parse() {
        Ok(uniform_id) => uniform_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match Uniform::find_by_id(&uniform_id, db.get_ref()).await {
        Ok(mut uniform) => match uniform.update(payload.into_inner(), db.get_ref()).await {
            Ok(_) => HttpResponse::Ok().json(UniformResponse::from(uniform)),
            _ => HttpResponse::InternalServerError().body("UNIFORM_UPDATING_FAILED"),
        },
        Err(e) => error_handler(e),
    }
}
#[delete("/{uniform_id}")]
pub async fn delete_uniform(
    uniform_id: web::Path<String>,
    db: web::Data<Database>,
) -> HttpResponse {
    let uniform_id = match uniform_id.parse() {
        Ok(uniform_id) => uniform_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match Uniform::find_by_id(&uniform_id, db.get_ref()).await {
        Ok(uniform) => {
            let _ = uniform.delete(db.get_ref()).await;

            HttpResponse::NoContent().finish()
        }
        Err(e) => error_handler(e),
    }
}
