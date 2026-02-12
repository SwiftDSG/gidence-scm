use std::{collections::HashMap, fs::read_to_string, io, sync::Arc, time::Duration};

use actix::{Addr, Recipient};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    App, HttpServer,
    web::{self, scope},
};
use chrono::Utc;
use tokio::{sync::RwLock, time::sleep};

use central::{CentralWebSocket, CentralWebSocketMessage};
use models::user::{User, UserAuthenticationMiddlewareFactory, UserRole, load_keys};
use uuid::Uuid;

mod central;
mod database;
mod helper;
mod models;
mod routes;
mod views;

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

    let processor = Arc::new(RwLock::new(HashMap::<String, i64>::new()));
    let client = Arc::new(RwLock::new(HashMap::<
        Recipient<CentralWebSocketMessage>,
        (String, Addr<CentralWebSocket>),
    >::new()));

    match User::find_all(&database).await {
        Ok(users) => {
            if users.is_empty() {
                let mut user = User {
                    id: Uuid::new_v4().to_string(),
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
                id: Uuid::new_v4().to_string(),
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

            let payload = {
                let mut processor = processor_clone.write().await;
                processor.retain(|_, exp| timestamp <= *exp);
                central::CentralWebSocketResponse::Processor((*processor).clone())
            };

            let client = client_clone.read().await;
            for (_, (_, client)) in (*client).iter() {
                client.do_send(central::CentralWebSocketMessage(
                    serde_json::to_string(&payload).unwrap(),
                ));
            }

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
            .app_data(web::Data::new(processor.clone()))
            .app_data(web::Data::new(client.clone()))
            .service(
                web::scope(&std::env::var("BASE_PATH").unwrap())
                    .service(web::resource("/ws").to(central::ws_index))
                    .service(routes::ping)
                    .service(Files::new("/static", "./evidence").show_files_listing())
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
                            .service(routes::processor::update_processor)
                            .service(routes::processor::delete_processor)
                            .service(routes::processor::get_processors)
                            .service(routes::processor::get_processor)
                            .service(routes::processor::sync_processor),
                    )
                    .service(
                        scope("/evidences")
                            .service(routes::evidence::create_evidence)
                            .service(routes::evidence::get_evidences),
                    )
                    .service(scope("/cameras").service(routes::camera::get_cameras))
                    .service(
                        scope("/subscribers")
                            .service(routes::subscriber::refresh)
                            .service(routes::subscriber::subscribe)
                            .service(routes::subscriber::unsubscribe),
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
