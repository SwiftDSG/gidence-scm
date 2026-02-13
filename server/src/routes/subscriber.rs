use actix_web::{HttpResponse, delete, post, put, web};
use mongodb::Database;

use crate::{
    helper::error_handler,
    models::subscriber::{
        Subscriber, SubscriberKind, SubscriberQuery, SubscriberQueryKind, SubscriberRequest,
    },
};

#[post("")]
pub async fn subscribe(
    payload: web::Json<SubscriberRequest>,
    db: web::Data<Database>,
) -> HttpResponse {
    let request = payload.into_inner();

    if (Subscriber::find_by_kind(&request.kind, db.get_ref()).await).is_ok() {
        return HttpResponse::Conflict().finish();
    }

    let mut subscriber = Subscriber::from(request);

    match subscriber.save(db.get_ref()).await {
        Ok(()) => HttpResponse::Created().json(subscriber),
        Err(e) => error_handler(e),
    }
}

#[put("/{subscriber_id}")]
pub async fn refresh(
    subscriber_id: web::Path<String>,
    payload: web::Json<SubscriberRequest>,
    db: web::Data<Database>,
) -> HttpResponse {
    let subscriber_id = match subscriber_id.parse() {
        Ok(v) => v,
        _ => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    let request = payload.into_inner();

    let mut subscriber = match Subscriber::find_by_id(&subscriber_id, db.get_ref()).await {
        Ok(v) => v,
        Err(e) => return error_handler(e),
    };

    subscriber.user_id = request.user_id;
    subscriber.kind = request.kind;

    match subscriber.update(db.get_ref()).await {
        Ok(()) => HttpResponse::Ok().json(subscriber),
        Err(e) => error_handler(e),
    }
}

#[delete("")]
pub async fn unsubscribe(
    query: web::Query<SubscriberQuery>,
    db: web::Data<Database>,
) -> HttpResponse {
    let kind = match (&query.kind, &query.token) {
        (Some(kind), Some(token)) => match kind {
            SubscriberQueryKind::Apple => SubscriberKind::Apple(token.clone()),
        },
        _ => return HttpResponse::BadRequest().finish(),
    };

    match Subscriber::find_by_kind(&kind, db.get_ref()).await {
        Ok(subscriber) => {
            let _ = subscriber.delete(db.get_ref()).await;

            HttpResponse::NoContent().finish()
        }
        Err(e) => error_handler(e),
    }
}
