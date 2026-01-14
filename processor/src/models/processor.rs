use std::{
    fs::{read_to_string, write},
    net::IpAddr,
};

use get_if_addrs::get_if_addrs;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Processor {
    pub id: String,
    pub cluster_id: Option<String>,
    pub name: String,
    pub address: ProcessorAddress,
    pub camera: Vec<ProcessorCamera>,
    pub model: String,
    pub udp: Option<ProcessorUdp>,
    pub server: Option<ProcessorServer>,
    pub version: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessorAddress {
    pub host: [u8; 4],
    pub port: u16,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessorUdp {
    pub host: [u8; 4],
    pub port: u16,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessorServer {
    pub host: [u8; 4],
    pub port: u16,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessorCamera {
    pub id: String,
    pub address: ProcessorCameraAddress,
    pub name: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessorCameraAddress {
    pub host: [u8; 4],
    pub port: u16,
    pub path: Option<String>,
    pub authentication: Option<(String, String)>,
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
                    cluster_id: None,
                    name: id,
                    address: ProcessorAddress { host, port: 8000 },
                    camera: vec![],
                    model: "yolov8n".to_string(),
                    version: Uuid::new_v4().to_string(),
                    udp: None,
                    server: None,
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
        self.version = Uuid::new_v4().to_string();
        self.update();
    }
}

impl ProcessorServer {
    pub async fn register(&self, processor_address: &ProcessorAddress) -> bool {
        let address = format!(
            "http://{}.{}.{}.{}:{}",
            self.host[0], self.host[1], self.host[2], self.host[3], self.port
        );

        let client = Client::new();
        let payload = serde_json::to_string(processor_address).unwrap();
        let response = match client
            .post(&format!("{}/register", address))
            .header("Content-Type", "application/json")
            .body(payload)
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                println!("Failed to reach server {}: {}", address, e);
                return false;
            }
        };

        if response.status().is_success() {
            true
        } else {
            false
        }
    }
    pub async fn ping(&self, processor_id: &str, processor_version: &str) -> bool {
        let address = format!(
            "http://{}.{}.{}.{}:{}",
            self.host[0], self.host[1], self.host[2], self.host[3], self.port
        );

        let client = Client::new();
        let response = match client
            .get(&format!(
                "{}/{}/{}",
                address, processor_id, processor_version
            ))
            .send()
            .await
        {
            Ok(response) => response,
            Err(_) => {
                return false;
            }
        };

        if response.status().is_success() {
            true
        } else {
            false
        }
    }
}
