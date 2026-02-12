use actix_web::{HttpMessage, HttpRequest, HttpResponse, delete, get, post, web};
use mongodb::Database;

use crate::{
    helper::error_handler,
    models::{
        cluster::{Cluster, ClusterQuery, ClusterRequest},
        user::{User, UserAuthentication, UserRole},
    },
    views::cluster::ViewCluster,
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
                    user.cluster_id.push(cluster.id.clone());
                    let _ = user.update(None, db.get_ref()).await;
                }
            }

            let query = ClusterQuery {
                cluster_id: Some(vec![cluster.id.clone()]),
                text: None,
                date_maximum: None,
                date_minimum: None,
            };

            HttpResponse::Created().json(ViewCluster::find_one(&query, db.get_ref()).await.unwrap())
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
        let user = match User::find_by_id(&issuer.id, db.get_ref()).await {
            Ok(v) => v,
            _ => return HttpResponse::Unauthorized().body("UNAUTHORIZED"),
        };

        query.cluster_id = Some(user.cluster_id);
    }

    match ViewCluster::find_many(&query, db.get_ref()).await {
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

    match ViewCluster::find_one(&query, db.get_ref()).await {
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
