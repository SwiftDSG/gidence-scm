use actix_web::{HttpMessage, HttpRequest, HttpResponse, delete, get, post, put, web};
use mongodb::Database;

use crate::{
    helper::error_handler,
    models::{
        cluster::Cluster,
        user::{
            User, UserAuthentication, UserCredential, UserQuery, UserRefreshRequest, UserRequest,
            UserRole,
        },
    },
    views::user::ViewUser,
};

#[get("")]
pub async fn get_users(query: web::Query<UserQuery>, db: web::Data<Database>) -> HttpResponse {
    match ViewUser::find_many(&query, db.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        _ => HttpResponse::NotFound().body("USER_NOT_FOUND"),
    }
}
#[get("/{user_id}")]
pub async fn get_user(user_id: web::Path<String>, db: web::Data<Database>) -> HttpResponse {
    let user_id = match user_id.parse() {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match User::find_by_id(&user_id, db.get_ref()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::NotFound().body("USER_NOT_FOUND"),
    }
}
#[put("/{user_id}")]
pub async fn update_user(
    req: HttpRequest,
    user_id: web::Path<String>,
    payload: web::Json<UserRequest>,
    db: web::Data<Database>,
) -> HttpResponse {
    let user_id = match user_id.parse() {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    let issuer = match req.extensions().get::<UserAuthentication>() {
        Some(issuer) => issuer.clone(),
        None => return HttpResponse::Unauthorized().body("UNAUTHORIZED"),
    };
    if issuer.id != user_id && issuer.role == UserRole::Officer {
        return HttpResponse::Unauthorized().body("UNAUTHORIZED");
    }

    match User::find_by_id(&user_id, db.get_ref()).await {
        Ok(mut user) => {
            let payload = payload.into_inner();

            let mut password = None;
            if payload.role == UserRole::SuperAdmin && user.role != UserRole::SuperAdmin {
                return HttpResponse::BadRequest().body("USER_ROLE_INVALID");
            }
            if payload.password.len() >= 8 {
                password = Some(payload.password);
            }
            user.role = payload.role;
            user.name = payload.name;
            user.number = payload.number;
            user.cluster_id = payload.cluster_id;

            match user.update(password, db.get_ref()).await {
                Ok(_) => HttpResponse::Ok().json(ViewUser::from(user, db.get_ref()).await),
                _ => HttpResponse::InternalServerError().body("USER_UPDATING_FAILED"),
            }
        }
        _ => HttpResponse::NotFound().body("USER_NOT_FOUND"),
    }
}
#[delete("/{user_id}")]
pub async fn delete_user(user_id: web::Path<String>, db: web::Data<Database>) -> HttpResponse {
    let user_id = match user_id.parse() {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match User::find_by_id(&user_id, db.get_ref()).await {
        Ok(user) => {
            if user.role == UserRole::SuperAdmin {
                return HttpResponse::BadRequest().body("USER_OWNER_CANNOT_BE_DELETED");
            }

            user.delete(db.get_ref()).await;

            HttpResponse::NoContent().finish()
        }
        _ => HttpResponse::NotFound().body("ID MEMBER TIDAK TERDAFTAR"),
    }
}
#[post("")]
pub async fn create_user(payload: web::Json<UserRequest>, db: web::Data<Database>) -> HttpResponse {
    let payload: UserRequest = payload.into_inner();

    if payload.password.len() < 8 {
        return HttpResponse::BadRequest().body("USER_MUST_HAVE_VALID_PASSWORD");
    }

    let mut user = User::from(payload);

    if User::super_admin_available(db.get_ref()).await && user.role == UserRole::SuperAdmin {
        return HttpResponse::BadRequest().body("USER_ROLE_INVALID");
    }

    if User::find_by_number(&user.number, db.get_ref())
        .await
        .is_ok()
    {
        return HttpResponse::BadRequest().body("USER_ALREADY_EXIST");
    }

    if let Ok(mut clusters) = Cluster::find_all(db.get_ref()).await {
        for cluster in clusters.drain(..) {
            user.cluster_id.push(cluster.id);
        }
    }

    match user.save(db.get_ref()).await {
        Ok(_) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::InternalServerError().body("USER_SAVING_FAILED"),
    }
}

#[post("/login")]
pub async fn login(payload: web::Json<UserCredential>, db: web::Data<Database>) -> HttpResponse {
    let payload = payload.into_inner();

    match payload.authenticate(db.get_ref()).await {
        Ok(((atk, rtk), user)) => HttpResponse::Ok().json(serde_json::json!({
            "atk": atk,
            "rtk": rtk,
            "user": user
        })),
        Err(e) => error_handler(e),
    }
}

#[post("/refresh")]
pub async fn refresh(
    payload: web::Json<UserRefreshRequest>,
    db: web::Data<Database>,
) -> HttpResponse {
    let payload = payload.into_inner();

    match UserCredential::refresh(&payload.rtk, db.get_ref()).await {
        Ok((atk, rtk, user)) => HttpResponse::Ok().json(serde_json::json!({
            "atk": atk,
            "rtk": rtk,
            "user": user
        })),
        Err(e) => error_handler(e),
    }
}
