use std::{
    fs::{read_to_string, write},
    net::IpAddr,
    os::macos::raw::stat,
};

use chrono::Local;
use get_if_addrs::get_if_addrs;
use reqwest::{
    Client,
    multipart::{Form, Part},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Processor {
    pub id: String,
    pub name: String,
    pub model: String,
    pub address: ProcessorAddress,
    pub webhook: Option<ProcessorWebhook>,
    pub version: i64,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessorAddress {
    pub host: [u8; 4],
    pub port: u16,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessorWebhook {
    pub host: ProcessorWebhookHost,
    pub port: Option<u16>,
    pub secure: bool,
    pub path: ProcessorWebhookPath,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProcessorWebhookHost {
    Domain(String),
    IPv4([u8; 4]),
}
impl ProcessorWebhookHost {
    pub fn to_string(&self) -> String {
        match self {
            ProcessorWebhookHost::Domain(domain) => domain.clone(),
            ProcessorWebhookHost::IPv4(ip) => format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]),
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessorWebhookPath {
    pub evidence: String,
    pub update: String,
}

impl Processor {
    pub fn load() -> Self {
        let processor_json = match read_to_string("processor.json") {
            Ok(processor) => processor,
            Err(_) => {
                let id = Uuid::new_v4().to_string();

                let mut host = [127, 0, 0, 0];
                if let Ok(interfaces) = get_if_addrs() {
                    for iface in interfaces {
                        let ip = iface.ip();
                        if !ip.is_loopback() {
                            match ip {
                                IpAddr::V4(ip) => host = ip.octets(),
                                _ => (),
                            }
                        }
                    }
                }

                let processor = Self {
                    id: id.clone(),
                    name: id,
                    model: "yolov8n.hef".to_string(),
                    address: ProcessorAddress { host, port: 8000 },
                    webhook: None,
                    version: Local::now().timestamp_millis(),
                };

                write("processor.json", serde_json::to_string(&processor).unwrap()).unwrap();

                return processor;
            }
        };
        let processor: Self = serde_json::from_str(&processor_json).unwrap();
        processor
    }
    pub fn update(&self) {
        write("processor.json", serde_json::to_string(self).unwrap()).unwrap();
    }
    pub fn update_version(&mut self) {
        self.version = Local::now().timestamp_millis();
        self.update();
    }
}

impl ProcessorWebhook {
    // Send multipart/form-data with text and file
    pub async fn send_evidence(&self, text: String, file: Vec<u8>, evidence_id: &String) -> bool {
        let mut url = format!(
            "{}://{}",
            if self.secure { "https" } else { "http" },
            self.host.to_string()
        );

        if let Some(port) = self.port {
            url = format!("{}:{}", url, port);
        }

        let address = format!("{}/{}", url, self.path.evidence.trim_start_matches('/'));

        let file = Part::bytes(file).file_name(format!("{}.jpg", evidence_id));

        println!("[WEBHOOK] Sending evidence to {}", address);

        let client = Client::new();
        let form = Form::new().text("data", text).part("image", file);

        match client.post(&address).multipart(form).send().await {
            Ok(response) => {
                let status = response.status();
                println!(
                    "Webhook response status: {}",
                    response.text().await.unwrap_or_default()
                );
                if status.is_success() { true } else { false }
            }
            Err(e) => {
                println!("Failed to send evidence to webhook: {:?}", e);
                false
            }
        }
    }

    // Send processor's information to the webhook
    pub async fn send_update(&self, text: String) -> bool {
        let mut url = format!(
            "{}://{}",
            if self.secure { "https" } else { "http" },
            self.host.to_string()
        );

        if let Some(port) = self.port {
            url = format!("{}:{}", url, port);
        }

        let address = format!("{}/{}", url, self.path.update.trim_start_matches('/'));

        let client = Client::new();
        match client
            .post(&address)
            .header("Content-Type", "application/json")
            .body(text)
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();
                if status.is_success() { true } else { false }
            }
            Err(_) => {
                return false;
            }
        }
    }
}
