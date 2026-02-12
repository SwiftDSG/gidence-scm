use std::fs::{read_to_string, write};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Camera {
    pub id: String,
    pub address: CameraAddress,
    pub name: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CameraAddress {
    pub host: [u8; 4],
    pub port: u16,
    pub path: Option<String>,
    pub authentication: Option<(String, String)>,
}

impl Camera {
    pub fn insert_many(camera: &Vec<Self>) {
        let camera_json = serde_json::to_string(&camera).unwrap();
        write("camera.json", camera_json).unwrap();
    }

    pub fn insert_one(&self) {
        let mut camera = Self::load();
        camera.push(self.clone());
        Self::insert_many(&camera);
    }

    pub fn load() -> Vec<Self> {
        let camera_json = match read_to_string("camera.json") {
            Ok(camera) => camera,
            Err(_) => {
                write("camera.json", "[]").unwrap();

                return Vec::new();
            }
        };
        let camera: Vec<Self> = serde_json::from_str(&camera_json).unwrap();
        camera
    }
}
