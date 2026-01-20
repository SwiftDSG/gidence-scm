use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse};
use mongodb::Database;

use crate::{
    helper::error_handler,
    models::{
        cluster::{Cluster, ClusterQuery, ClusterRequest, ClusterResponse},
        user::{User, UserAuthentication, UserRole},
    },
};

#[post("")]
pub async fn create_cluster(
    payload: web::Json<ClusterRequest>,
    db: web::Data<Database>,
) -> HttpResponse {
    let request = payload.into_inner();

    let cluster = Cluster::from(request);

    match cluster.save(db.get_ref()).await {
        Ok(()) => {
            if let Ok(mut users) = User::find_all(db.get_ref()).await {
                for mut user in users.drain(..) {
                    user.cluster_id.push(cluster._id);
                    let _ = user.update(None, db.get_ref()).await;
                }
            }
            HttpResponse::Created().json(ClusterResponse::from(cluster))
        }
        Err(e) => error_handler(e),
    }
}

#[get("")]
pub async fn get_clusters(
    req: HttpRequest,
    query: web::Query<ClusterQuery>,
    db: web::Data<Database>,
) -> HttpResponse {
    let issuer = match req.extensions().get::<UserAuthentication>() {
        Some(issuer) => issuer.clone(),
        None => return HttpResponse::Unauthorized().body("UNAUTHORIZED"),
    };

    let mut query = query.into_inner();
    if issuer.role != UserRole::SuperAdmin {
        let user = match User::find_by_id(&issuer._id, db.get_ref()).await {
            Ok(v) => v,
            _ => return HttpResponse::Unauthorized().body("UNAUTHORIZED"),
        };

        query.cluster_id = Some(user.cluster_id);
    }

    match Cluster::find_many_minimal(&query, db.get_ref()).await {
        Ok(clusters) => HttpResponse::Ok().json(clusters),
        Err(e) => error_handler(e),
    }
}

#[get("/{cluster_id}")]
pub async fn get_cluster(
    cluster_id: web::Path<String>,
    query: web::Query<ClusterQuery>,
    db: web::Data<Database>,
) -> HttpResponse {
    let cluster_id = match cluster_id.parse() {
        Ok(v) => v,
        _ => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    let mut query = query.into_inner();
    query.cluster_id = Some(vec![cluster_id]);

    match Cluster::find_one_minimal(&query, db.get_ref()).await {
        Ok(cluster) => HttpResponse::Ok().json(cluster),
        Err(e) => error_handler(e),
    }
}

#[delete("/{cluster_id}")]
pub async fn delete_cluster(
    cluster_id: web::Path<String>,
    db: web::Data<Database>,
) -> HttpResponse {
    let cluster_id = match cluster_id.parse() {
        Ok(v) => v,
        _ => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match Cluster::find_by_id(&cluster_id, db.get_ref()).await {
        Ok(cluster) => {
            let _ = cluster.delete(db.get_ref()).await;
            HttpResponse::NoContent().finish()
        }
        Err(e) => error_handler(e),
    }
}
