use actix_web::HttpResponse;

use crate::models::event::EventKind;

pub fn error_handler(e: EventKind) -> HttpResponse {
    match e {
        EventKind::NotFound => HttpResponse::NotFound().body(e.to_string()),
        _ => HttpResponse::BadRequest().body(e.to_string()),
    }
}
