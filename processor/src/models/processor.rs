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
    pub name: String,
    pub model: String,
    pub address: ProcessorAddress,
    pub camera: Vec<ProcessorCamera>,
    pub webhook: Vec<ProcessorWebhook>,
    pub udp: Option<ProcessorUdp>,
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
pub struct ProcessorWebhook {
    pub host: ProcessorWebhookHost,
    pub port: Option<u16>,
    pub path: String,
    pub secure: bool,
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
                    name: id,
                    model: "yolov8n.hef".to_string(),
                    address: ProcessorAddress { host, port: 8000 },
                    camera: vec![],
                    webhook: vec![],
                    udp: None,
                    version: Uuid::new_v4().to_string(),
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

impl ProcessorWebhook {
    fn to_string(&self) -> String {
        let mut url = format!(
            "{}://{}",
            if self.secure { "https" } else { "http" },
            self.host.to_string()
        );

        if let Some(port) = self.port {
            url = format!("{}:{}", url, port);
        }

        format!("{}/{}", url, self.path.trim_start_matches('/'))
    }
    pub async fn ping(&self) -> bool {
        let address = self.to_string();

        let client = Client::new();
        let response = match client.get(&address).send().await {
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
