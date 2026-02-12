use actix::{Actor, Addr, AsyncContext, Handler, Recipient, StreamHandler, prelude::Message};
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::views::evidence::ViewEvidence;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CentralWebSocketRequest {
    Connect(String),
    Disconnect,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CentralWebSocketResponse {
    Processor(HashMap<String, i64>),
    Evidence(ViewEvidence),
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct CentralWebSocketMessage(pub String);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct CentralWebSocket {
    processor: Arc<RwLock<HashMap<String, i64>>>, // Processor's last seen
    client:
        Arc<RwLock<HashMap<Recipient<CentralWebSocketMessage>, (String, Addr<CentralWebSocket>)>>>,
}

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    processor: web::Data<Arc<RwLock<HashMap<String, i64>>>>,
    client: web::Data<
        Arc<RwLock<HashMap<Recipient<CentralWebSocketMessage>, (String, Addr<CentralWebSocket>)>>>,
    >,
) -> Result<HttpResponse, Error> {
    let processor = processor.get_ref().clone();
    let client = client.get_ref().clone();

    ws::start(
        CentralWebSocket::new(processor.clone(), client.clone()),
        &req,
        stream,
    )
}

impl Actor for CentralWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let client = self.client.clone();
        let recipient = ctx.address().recipient();

        tokio::spawn(async move {
            let mut client = client.write().await;
            (*client).remove(&recipient);
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for CentralWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(msg) => match msg {
                // Handle Client/iOS websocket messages
                ws::Message::Text(msg) => {
                    let processor = self.processor.clone();
                    let client = self.client.clone();
                    let address = ctx.address();

                    tokio::spawn(async move {
                        let req = match serde_json::from_str::<CentralWebSocketRequest>(&msg) {
                            Ok(v) => v,
                            _ => {
                                if msg == "disconnect" {
                                    CentralWebSocketRequest::Disconnect
                                } else {
                                    return;
                                }
                            }
                        };

                        match req {
                            CentralWebSocketRequest::Connect(user_id) => {
                                println!("WS CONNECTED");
                                let processor = processor.read().await;
                                let mut data = HashMap::new();

                                for (k, v) in processor.iter() {
                                    data.insert(k.to_string(), v.clone());
                                }

                                drop(processor);

                                let payload = CentralWebSocketResponse::Processor(data);
                                address.do_send(CentralWebSocketMessage(
                                    serde_json::to_string(&payload).unwrap(),
                                ));

                                let mut client = client.write().await;
                                let recipient = address.clone().recipient();
                                (*client).insert(recipient, (user_id, address));
                            }
                            CentralWebSocketRequest::Disconnect => {
                                println!("WS DISCONNECTED");
                                let mut client = client.write().await;
                                let recipient = address.clone().recipient();
                                (*client).remove(&recipient);
                            }
                        }
                    });
                }
                ws::Message::Close(_) => ctx.close(None),
                ws::Message::Ping(msg) => ctx.pong(&msg),
                ws::Message::Pong(_) => (),
                _ => ctx.close(None),
            },
            Err(err) => {
                println!("WS ERROR: {:?}", err);
            }
        }
    }
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.address()
            .do_send(CentralWebSocketMessage(String::new()));
    }
}

impl CentralWebSocket {
    fn new(
        processor: Arc<RwLock<HashMap<String, i64>>>,
        client: Arc<
            RwLock<HashMap<Recipient<CentralWebSocketMessage>, (String, Addr<CentralWebSocket>)>>,
        >,
    ) -> Self {
        Self { processor, client }
    }
}

impl Handler<CentralWebSocketMessage> for CentralWebSocket {
    type Result = ();

    fn handle(&mut self, msg: CentralWebSocketMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
