use std::collections::HashMap;

use serde::Serialize;

use crate::models::{camera::Camera, evidence::Evidence, processor::Processor};

pub mod camera;
pub mod evidence;
pub mod processor;

#[derive(Clone, Serialize)]
pub struct Device {
    pub processor: Processor,
    pub camera: HashMap<String, Camera>,
}

// Reading struct to hold the state of evidence per camera
#[derive(Clone, Serialize)]
pub struct Reading {
    pub camera: HashMap<String, (Option<Evidence>, i64, f64)>, // camera_id -> (evidence, timestamp, fps)
}
