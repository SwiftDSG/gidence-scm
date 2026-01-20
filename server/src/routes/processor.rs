use std::{collections::HashMap, net::SocketAddr, str::FromStr, sync::Arc, time::Duration};

use actix::{Addr, Recipient};
use actix_web::{delete, get, post, web, HttpResponse};
use chrono::Utc;
use mongodb::{bson::oid::ObjectId, Database};
use tokio::{net::UdpSocket, sync::RwLock, time::timeout};

use crate::{
    central::{CentralWebSocket, CentralWebSocketMessage, CentralWebSocketResponse},
    helper::error_handler,
    models::{
        camera::{Camera, CameraQuery},
        cluster::Cluster,
        processor::{
            Processor, ProcessorData, ProcessorMinimalResponse, ProcessorQuery, ProcessorRequest,
            ProcessorResponse,
        },
        uniform::{Uniform, UniformQuery},
    },
};

#[post("")]
pub async fn create_processor(
    payload: web::Json<ProcessorRequest>,
    db: web::Data<Database>,
) -> HttpResponse {
    let request = payload.into_inner();

    if (Cluster::find_by_id(&request.cluster_id, db.get_ref()).await).is_err() {
        return HttpResponse::NotFound().body("NOT_FOUND");
    }

    let processor = Processor::from(request);

    match processor.save(db.get_ref()).await {
        Ok(()) => HttpResponse::Created().json(ProcessorResponse::from(processor)),
        Err(e) => error_handler(e),
    }
}

#[delete("/{processor_id}")]
pub async fn delete_processor(
    processor_id: web::Path<String>,
    db: web::Data<Database>,
) -> HttpResponse {
    let processor_id = match processor_id.parse() {
        Ok(processor_id) => processor_id,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match Processor::find_by_id(&processor_id, db.get_ref()).await {
        Ok(processor) => {
            let _ = processor.delete(db.get_ref()).await;

            HttpResponse::NoContent().finish()
        }
        Err(e) => error_handler(e),
    }
}

#[get("")]
pub async fn get_processors(
    query: web::Query<ProcessorQuery>,
    db: web::Data<Database>,
) -> HttpResponse {
    match Processor::find_many_minimal(&query, db.get_ref()).await {
        Ok(processors) => HttpResponse::Ok().json(processors),
        Err(e) => error_handler(e),
    }
}

#[get("/{processor_id}")]
pub async fn get_processor(
    processor_id: web::Path<String>,
    db: web::Data<Database>,
) -> HttpResponse {
    let processor_id = match processor_id.parse::<ObjectId>() {
        Ok(v) => v,
        _ => return HttpResponse::BadRequest().body("INVALID_ID"),
    };

    match Processor::find_by_id(&processor_id, db.get_ref()).await {
        Ok(processor) => {
            HttpResponse::Ok().json(ProcessorMinimalResponse::from(processor, db.get_ref()).await)
        }
        Err(e) => error_handler(e),
    }
}

// For processors to update themselves
#[get("/{processor_id}/{version}")]
pub async fn sync_processor(
    param: web::Path<(String, String)>,
    db: web::Data<Database>,
    processor: web::Data<Arc<RwLock<HashMap<ObjectId, i64>>>>,
    client: web::Data<
        Arc<
            RwLock<HashMap<Recipient<CentralWebSocketMessage>, (ObjectId, Addr<CentralWebSocket>)>>,
        >,
    >,
) -> HttpResponse {
    let param = param.into_inner();
    let processor_id = match param.0.parse::<ObjectId>() {
        Ok(v) => v,
        _ => return HttpResponse::BadRequest().body("INVALID_ID"),
    };
    let version = param.1;

    let mut response = ProcessorData {
        processor: match Processor::find_by_id(&processor_id, db.get_ref()).await {
            Ok(mut processor) => {
                if processor.version == version {
                    return HttpResponse::NoContent().finish();
                }

                processor.version = version;
                let _ = processor.save(db.get_ref()).await;
                ProcessorResponse::from(processor)
            }
            Err(_) => return HttpResponse::NotFound().finish(),
        },
        camera: Vec::new(),
        uniform: Vec::new(),
    };

    let mut processor = processor.write().await;
    let client = client.read().await;

    (*processor).insert(processor_id.clone(), Utc::now().timestamp_millis() + 60000);

    let mut data = HashMap::new();

    for (k, v) in processor.iter() {
        data.insert(k.to_string(), v.clone());
    }

    let payload = CentralWebSocketResponse::Data(data);

    for (_, (_, client)) in (*client).iter() {
        let payload = serde_json::to_string(&payload).unwrap();
        println!("BROADCASTING WS: {}", payload);
        client.do_send(CentralWebSocketMessage(payload));
    }

    let cluster = match Cluster::find_by_id(
        &ObjectId::from_str(&response.processor.cluster_id).unwrap(),
        db.get_ref(),
    )
    .await
    {
        Ok(v) => v,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    response.camera = Camera::find_many(
        &CameraQuery {
            cluster_id: None,
            processor_id: Some(processor_id.clone()),
            date_minimum: None,
            date_maximum: None,
            text: None,
            limit: None,
            skip: None,
        },
        db.get_ref(),
    )
    .await
    .unwrap_or_default();

    response.uniform = Uniform::find_many(
        &UniformQuery {
            uniform_id: Some(cluster.uniform_id),
            text: None,
        },
        db.get_ref(),
    )
    .await
    .unwrap_or_default();

    HttpResponse::Ok().json(response)
}

#[get("/scan-processors")]
pub async fn scan_processors() -> HttpResponse {
    let socket = match UdpSocket::bind("0.0.0.0:34254").await {
        Ok(v) => v,
        _ => return HttpResponse::InternalServerError().finish(),
    };

    let mut hosts = HashMap::new();
    let mut now = Utc::now().timestamp_millis();
    let mut buf;

    let target = now + 1000;

    while now < target {
        now = Utc::now().timestamp_millis();
        buf = [0; 1024];

        let (bytes, addr) = match timeout(Duration::from_secs(1), socket.recv_from(&mut buf)).await
        {
            Ok(Ok(v)) => v,
            _ => continue,
        };
        let ip = match addr {
            SocketAddr::V4(v) => v.ip().octets(),
            _ => continue,
        };
        let _id = String::from_utf8_lossy(&buf[..bytes]).to_string();
        if hosts.get(&_id).is_none() {
            hosts.insert(_id.to_string().clone(), ip);
        }
    }

    HttpResponse::Ok().json(hosts)
}
