use chrono::Local;
use models::processor::Processor;
use serde_json::from_slice;
use std::{
    collections::{HashMap, VecDeque},
    env,
    process::{Command, Stdio},
    sync::Arc,
};
use tokio::{
    fs,
    io::AsyncReadExt,
    net::UnixListener,
    sync::RwLock,
    time::{Duration, sleep},
};
use uuid::Uuid;

use crate::models::{Device, Reading, camera::Camera, evidence::Evidence};

mod models;

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let simulation_mode = args.iter().any(|arg| arg == "--simulation" || arg == "-s");

    if simulation_mode {
        println!("=== SCM Processor Starting (SIMULATION MODE) ===\n");
    } else {
        println!("=== SCM Processor Starting ===\n");
    }

    // Set up UDS listener
    let _ = fs::remove_file("/tmp/gidence-scm_uds.sock").await;
    let listener = UnixListener::bind("/tmp/gidence-scm_uds.sock").unwrap();

    let timestamp = Local::now().timestamp_millis();

    // Shared state across threads
    let violation = HashMap::<String, Evidence>::new(); // camera_id+person_id -> Evidence
    let queue = VecDeque::<Evidence>::new();
    let mut reading = Reading {
        camera: HashMap::new(),
    };
    let mut device = Device {
        processor: Processor::load(),
        camera: HashMap::new(),
    };

    let mut cameras_raw = Camera::load();
    let mut cameras = HashMap::new();
    for c in cameras_raw.drain(..) {
        reading.camera.insert(c.id.clone(), (None, timestamp));
        cameras.insert(c.id.clone(), c);
    }
    device.camera = cameras;

    let violation = Arc::new(RwLock::new(violation));
    let queue = Arc::new(RwLock::new(queue));
    let reading = Arc::new(RwLock::new(reading));
    let device = Arc::new(RwLock::new(device));

    // UDS THREAD: UDS listener for receiving Evidence structs
    let reading_clone = Arc::clone(&reading);
    let queue_clone = Arc::clone(&queue);
    let _ = tokio::spawn(async move {
        loop {
            let (mut stream, _) = match listener.accept().await {
                Ok(conn) => conn,
                Err(e) => {
                    eprintln!("[UDS] Failed to accept connection: {}", e);
                    continue;
                }
            };
            let mut buffer = vec![0; 65536];

            if let Err(e) = stream.read(&mut buffer).await {
                println!("Error reading from stream: {}", e);
                continue;
            }

            let filled_len = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());

            // Print received raw JSON for debugging
            let raw_json = String::from_utf8_lossy(&buffer[0..filled_len]);

            let mut evidence = match from_slice::<Evidence>(buffer[0..filled_len].as_ref()) {
                Ok(v) => v,
                Err(e) => {
                    println!("Error parsing JSON: {}", e);
                    continue;
                }
            };
            evidence.id = Uuid::new_v4().to_string();

            // Update the reading with the new evidence
            {
                let mut queue = queue_clone.write().await;
                queue.push_back(evidence.clone());
            }
            {
                let mut reading = reading_clone.write().await;
                if let Some(entry) = reading.camera.get_mut(&evidence.camera_id) {
                    entry.0 = Some(evidence);
                    entry.1 = Local::now().timestamp_millis();
                }
            }
        }
    });

    // QUEUE PROCESSOR THREAD: Process evidence from the queue
    let violation_clone = Arc::clone(&violation);
    let device_clone = Arc::clone(&device);
    let queue_clone = Arc::clone(&queue);
    let _ = tokio::spawn(async move {
        loop {
            let evidence = {
                let mut queue = queue_clone.write().await;
                queue.pop_front()
            };

            let evidence = match evidence {
                Some(e) => e,
                None => {
                    sleep(Duration::from_millis(100)).await;
                    continue;
                }
            };

            let mut new_violation = false;
            for person in evidence.person.iter() {
                let key = format!("{}-{}", evidence.camera_id, person.id);

                // Check if a violation by this person on this camera already exists
                if let Some(existing) = {
                    let violation = violation.read().await;
                    violation.get(&key).cloned()
                } {
                    // Check if the old evidence is older than 10 minutes than the new one
                    if evidence.timestamp - existing.timestamp < 10 * 60 * 1000 {
                        // Skip inserting this evidence as it's a duplicate within 10 minutes
                        continue;
                    }
                }

                // Insert or update the violation record
                new_violation = true;
                {
                    let mut violation = violation_clone.write().await;
                    violation.insert(key, evidence.clone());
                }
            }

            // If no new violation detected, skip processing
            if !new_violation {
                println!(
                    "[Queue] Duplicate evidence within 10 minutes, skipping: {:?}",
                    evidence
                );
                continue;
            }

            // Send evidence and captured image via webhook
            let image = match fs::read(format!("/tmp/{}.jpg", evidence.camera_id)).await {
                Ok(d) => d,
                Err(_) => continue,
            };

            let mut webhook = {
                let device = device_clone.read().await;
                device.processor.webhook.clone()
            };

            if webhook.is_empty() {
                // Log the evidence if no webhook is configured
                println!("[Webhook] No webhook configured. Evidence: {:?}", evidence);
                continue;
            }

            for wh in webhook.drain(..) {
                let payload = serde_json::to_string(&evidence).unwrap();
                let _ = wh.send(payload, image.clone()).await;
            }

            sleep(Duration::from_millis(100)).await;
        }
    });

    // INFERENCE ENGINE THREAD: Spawn inference engine thread with auto-restart capability
    let _ = tokio::spawn(async move {
        // In simulation mode, just wait indefinitely (run simulator manually in another terminal)
        if simulation_mode {
            println!("[Simulation] Waiting for simulator connection...");
            println!("[Simulation] Run the simulator manually in another terminal:");
            println!("[Simulation]   cd processor && python -m simulator.main");
            println!("[Simulation] Press Ctrl+C to stop\n");
            loop {
                sleep(Duration::from_secs(3600)).await;
            }
        }

        // Normal mode: spawn the real Inference Engine with auto-restart
        let mut count = 0;
        let delay = Duration::from_secs(5);

        loop {
            count += 1;
            println!("[Inference Engine] Starting... (attempt #{})", count);

            // Spawn the Python inference script with venv activation
            let mut child = match Command::new("bash")
                .arg("-c")
                .arg("source setup.sh && python3 -m inference.main")
                .stdout(Stdio::inherit()) // Show Python output
                .stderr(Stdio::inherit()) // Show Python errors
                .spawn()
            {
                Ok(child) => {
                    println!(
                        "[Inference Engine] Started successfully (PID: {})",
                        child.id()
                    );
                    child
                }
                Err(e) => {
                    eprintln!("[Inference Engine] Failed to start: {}", e);
                    eprintln!("  Make sure bash, setup.sh, and inference/main.py exist");
                    println!("[Inference Engine] Retrying in {:?}...", delay);
                    sleep(delay).await;
                    continue;
                }
            };

            // Wait for the process to exit
            match child.wait() {
                Ok(status) => {
                    if status.success() {
                        println!("[Inference Engine] Exited successfully (code 0)");
                        println!(
                            "[Inference Engine] Assuming intentional shutdown, not restarting"
                        );
                        break; // Clean exit - don't restart
                    } else {
                        eprintln!("[Inference Engine] Crashed with status: {}", status);
                    }
                }
                Err(e) => {
                    eprintln!("[Inference Engine] Error waiting for process: {}", e);
                }
            }

            // Wait before restarting to prevent rapid restart loops
            println!("[Inference Engine] Restarting in {:?}...", delay);
            sleep(delay).await;
        }

        println!("[Inference Engine] Thread exiting");
    })
    .await;

    println!("\n=== SCM Processor Shutdown ===");
}
