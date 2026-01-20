use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io,
    sync::Arc,
    time::{Duration, Instant},
};

use a2::{
    Client, ClientConfig, DefaultNotificationBuilder, NotificationBuilder, NotificationOptions,
};
use actix::{Addr, Recipient};
use actix_cors::Cors;
use actix_web::{
    web::{self, scope},
    App, HttpServer,
};
use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use tokio::{
    sync::{Mutex, RwLock},
    time::sleep,
};

use central::{CentralWebSocket, CentralWebSocketMessage, CentralWebSocketResponse};
use models::{
    cluster::Cluster,
    processor::Processor,
    subscriber::{Subscriber, SubscriberKind},
    user::{load_keys, User, UserAuthenticationMiddlewareFactory, UserQuery, UserRole},
    violation::{Violation, ViolationMinimalResponse},
};

mod central;
mod database;
mod helper;
mod models;
mod routes;

fn load_env() {
    unsafe {
        if let Ok(env) = read_to_string(".env") {
            let lines: Vec<(&str, &str)> = env
                .lines()
                .map(|a| {
                    let b: Vec<&str> = a.split('=').collect();
                    (
                        <&str>::clone(b.first().expect("INVALID_ENVIRONMENT_VARIABLES")),
                        <&str>::clone(b.last().expect("INVALID_ENVIRONMENT_VARIABLES")),
                    )
                })
                .collect();

            for (key, value) in lines {
                std::env::set_var(key, value);
            }
        }

        if std::env::var("APNS_KEY").is_err() {
            std::env::set_var("APNS_KEY", "mongodb://localhost:27017");
        }
        if std::env::var("APNS_TEAM").is_err() {
            std::env::set_var("APNS_TEAM", "mongodb://localhost:27017");
        }
        if std::env::var("DATABASE_URI").is_err() {
            std::env::set_var("DATABASE_URI", "mongodb://localhost:27017");
        }
        if std::env::var("CLIENT_ORIGIN").is_err() {
            std::env::set_var("CLIENT_ORIGIN", "http://localhost:3000");
        }
        if std::env::var("CLIENT_URL").is_err() {
            std::env::set_var("CLIENT_URL", "http://localhost:3000");
        }
        if std::env::var("ADMIN_ORIGIN").is_err() {
            std::env::set_var("ADMIN_ORIGIN", "http://localhost:4000");
        }
        if std::env::var("ADMIN_URL").is_err() {
            std::env::set_var("ADMIN_URL", "http://localhost:4000");
        }
        if std::env::var("BASE_ORIGIN").is_err() {
            std::env::set_var("BASE_ORIGIN", "http://localhost:8000");
        }
        if std::env::var("BASE_URL").is_err() {
            std::env::set_var("BASE_URL", "http://localhost:8000");
        }
        if std::env::var("BASE_PATH").is_err() {
            std::env::set_var("BASE_PATH", "");
        }
        if std::env::var("HOST").is_err() {
            std::env::set_var("HOST", "127.0.0.1");
        }
        if std::env::var("PORT").is_err() {
            std::env::set_var("PORT", "8000");
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    load_env();
    load_keys();

    let port = std::env::var("PORT")
        .unwrap()
        .parse::<u16>()
        .expect("INVALID_PORT");
    let host = std::env::var("HOST").expect("INVALID_HOST");

    let database = database::connect()
        .await
        .expect("Failed to connect to database");

    let violation = Arc::new(Mutex::new(Vec::<Violation>::new()));
    let processor = Arc::new(RwLock::new(HashMap::<ObjectId, i64>::new()));
    let client = Arc::new(RwLock::new(HashMap::<
        Recipient<CentralWebSocketMessage>,
        (ObjectId, Addr<CentralWebSocket>),
    >::new()));

    // NOTIFICATION DISTRIBUTOR THREAD
    let database_clone = database.clone();
    let violation_clone = violation.clone();
    let client_clone = client.clone();
    let _ = tokio::spawn(async move {
        let mut file = File::open("keys/apns.p8").expect("APNS_NOT_FOUND");

        let key_id = std::env::var("APNS_KEY").expect("APNS_KEY_NOT_FOUND");
        let team_id = std::env::var("APNS_TEAM").expect("APNS_TEAM_NOT_FOUND");

        let mut apns = Client::token(
            &mut file,
            key_id.clone(),
            team_id.clone(),
            ClientConfig::new(a2::Endpoint::Sandbox),
        )
        .expect("TOKEN_CREATION_FAILED");

        let mut user_map = HashMap::<ObjectId, (Vec<ObjectId>, Instant)>::new();
        let mut counter = 0;
        loop {
            let mut violation = violation_clone.lock().await;
            if violation.is_empty() {
                drop(violation);
                sleep(Duration::from_secs(5)).await;
                continue;
            }

            let client = client_clone.read().await;

            for (_, (_id, client)) in (*client).iter() {
                let now = Instant::now();
                let (cluster_id, should_send) = match user_map.get(&_id) {
                    Some((cluster_id, last_sent)) => {
                        let should_send = now.duration_since(*last_sent) > Duration::from_secs(60); // 1 minute cooldown
                        (cluster_id.clone(), should_send)
                    },
                    _ => {
                        let cluster_id = match User::find_by_id(&_id, &database_clone).await {
                            Ok(v) => v.cluster_id,
                            _ => continue,
                        };

                        user_map.insert(_id.clone(), (cluster_id.clone(), now));
                        (cluster_id, true)
                    }
                };

                if !should_send {
                    continue;
                }
                let mut payload = Vec::new();
                for violation in (*violation).iter() {
                    if cluster_id.contains(&violation.cluster_id) {
                        payload
                            .push(ViolationMinimalResponse::from(violation, &database_clone).await);
                    }
                }

                client.do_send(CentralWebSocketMessage(
                    serde_json::to_string(&CentralWebSocketResponse::Violation(payload)).unwrap(),
                ));

                // Update the last notification time
                user_map.insert(_id.clone(), (cluster_id.clone(), now));
            }

            for violation in (*violation).drain(..) {
                let cluster =
                    match Cluster::find_by_id(&violation.cluster_id, &database_clone).await {
                        Ok(v) => v,
                        _ => continue,
                    };

                let mut users =
                    match User::find_many_by_cluster_id(&cluster._id, &database_clone).await {
                        Ok(v) => v,
                        _ => continue,
                    };

                for user in users.drain(..) {
                    let mut subscribers =
                        match Subscriber::find_many_by_user_id(&user._id, &database_clone).await {
                            Ok(v) => v,
                            _ => continue,
                        };

                    for subscriber in subscribers.drain(..) {
                        match &subscriber.kind {
                            SubscriberKind::Apple(token) => {
                                let options = NotificationOptions {
                                    apns_topic: Some("com.gidence.scm"),
                                    ..Default::default()
                                };

                                let title = format!(
                                    "Terjadi {} Pelanggaran Baru!",
                                    violation.uniform.len()
                                );
                                let subtitle = match Processor::find_by_id(
                                    &violation.processor_id,
                                    &database_clone,
                                )
                                .await
                                {
                                    Ok(v) => format!("Tertangkap kamera {}", v.name),
                                    _ => String::from("Cek sekarang!"),
                                };

                                let builder = DefaultNotificationBuilder::new()
                                    .set_title(&title)
                                    .set_subtitle(&subtitle)
                                    .set_sound("ping.flac");

                                let payload = builder.build(token, options);

                                if let Err(err) = apns.send(payload).await {
                                    println!("SENDING FAILED: {:#?}", err);
                                    let response = match err {
                                        a2::Error::ResponseError(v) => v,
                                        _ => continue,
                                    };

                                    if response.code == 403 || response.code == 410 {
                                        let _ = subscriber.delete(&database_clone).await;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            drop(violation);

            counter += 1;
            sleep(Duration::from_secs(5)).await;

            if counter >= 240 {
                apns = Client::token(
                    &mut file,
                    key_id.clone(),
                    team_id.clone(),
                    ClientConfig::new(a2::Endpoint::Sandbox),
                )
                .expect("TOKEN_CREATION_FAILED");
                counter = 0;
            }
        }
    });

    match User::find_many_minimal(
        &UserQuery {
            cluster_id: None,
            cluster_eid: None,
            text: None,
            limit: None,
            skip: None,
        },
        &database,
    )
    .await
    {
        Ok(users) => {
            if users.is_empty() {
                let mut user = User {
                    _id: ObjectId::new(),
                    cluster_id: Vec::new(),
                    number: String::from("111"),
                    name: String::from("Super Admin"),
                    password: String::from("1234abcd"),
                    role: UserRole::SuperAdmin,
                };
                let _ = user.save(&database).await;
            }
        }
        _ => {
            let mut user = User {
                _id: ObjectId::new(),
                cluster_id: Vec::new(),
                number: String::from("111"),
                name: String::from("Super Admin"),
                password: String::from("1234abcd"),
                role: UserRole::SuperAdmin,
            };
            let _ = user.save(&database).await;
        }
    };

    // STATE MANAGER THREAD
    let processor_clone = processor.clone();
    let client_clone = client.clone();
    let _ = tokio::spawn(async move {
        loop {
            let timestamp = Utc::now().timestamp_millis();

            let mut processor = processor_clone.write().await;

            let mut payload = Vec::<ObjectId>::new();

            for (_id, exp) in (*processor).iter() {
                if timestamp > *exp {
                    payload.push(_id.clone());
                }
            }

            if !payload.is_empty() {
                let client = client_clone.read().await;
                for _id in payload.drain(..) {
                    (*processor).remove(&_id);
                    for (_, (_, client)) in (*client).iter() {
                        client.do_send(central::CentralWebSocketMessage(
                            serde_json::to_string(&central::CentralWebSocketResponse::Left(
                                _id.to_string(),
                            ))
                            .unwrap(),
                        ));
                    }
                }
            }

            drop(processor);

            sleep(Duration::from_millis(30000)).await;
        }
    });

    println!("Running on: http://{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(UserAuthenticationMiddlewareFactory)
            .wrap(cors)
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(violation.clone()))
            .app_data(web::Data::new(processor.clone()))
            .app_data(web::Data::new(client.clone()))
            .service(
                web::scope(&std::env::var("BASE_PATH").unwrap())
                    .service(web::resource("/ws").to(central::ws_index))
                    .service(routes::ping)
                    .service(
                        actix_files::Files::new("/static", "./violations").show_files_listing(),
                    )
                    .service(
                        scope("/violations")
                            .service(routes::violation::create_violation)
                            .service(routes::violation::update_violation)
                            .service(routes::violation::get_violations),
                    )
                    .service(
                        scope("/users")
                            .service(routes::user::create_user)
                            .service(routes::user::update_user)
                            .service(routes::user::delete_user)
                            .service(routes::user::login)
                            .service(routes::user::refresh)
                            .service(routes::user::get_users)
                            .service(routes::user::get_user),
                    )
                    .service(
                        scope("/clusters")
                            .service(routes::cluster::create_cluster)
                            .service(routes::cluster::delete_cluster)
                            .service(routes::cluster::get_clusters)
                            .service(routes::cluster::get_cluster),
                    )
                    .service(
                        scope("/processors")
                            .service(routes::processor::scan_processors)
                            .service(routes::processor::create_processor)
                            .service(routes::processor::delete_processor)
                            .service(routes::processor::get_processors)
                            .service(routes::processor::get_processor)
                            .service(routes::processor::sync_processor),
                    )
                    .service(
                        scope("/cameras")
                            .service(routes::camera::create_camera)
                            .service(routes::camera::delete_camera)
                            .service(routes::camera::get_cameras),
                    )
                    .service(
                        scope("/subscribers")
                            .service(routes::subscriber::refresh)
                            .service(routes::subscriber::subscribe)
                            .service(routes::subscriber::unsubscribe),
                    )
                    .service(
                        scope("/uniforms")
                            .service(routes::uniform::get_uniforms)
                            .service(routes::uniform::create_uniform)
                            .service(routes::uniform::update_uniform)
                            .service(routes::uniform::delete_uniform),
                    ),
            )
    })
    .keep_alive(Duration::from_secs(75)) // WebSocket idle timeout
    .client_request_timeout(Duration::from_secs(30))
    .client_disconnect_timeout(Duration::from_secs(5))
    .bind((host, port))
    .expect("Unable to bind host and port")
    .workers(8)
    .run()
    .await
}
