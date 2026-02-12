use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use chrono::Local;
use models::processor::Processor;
use serde_json::from_slice;
use std::{
    collections::{HashMap, VecDeque},
    env,
    net::SocketAddr,
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
mod routes;

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
    let processor = Processor::load();

    let mut reading = Reading {
        camera: HashMap::new(),
    };
    let mut device = Device {
        processor: processor.clone(),
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
                    println!("[UDS] Failed to accept connection: {}", e);
                    continue;
                }
            };
            let mut buffer = vec![];

            if let Err(e) = stream.read_to_end(&mut buffer).await {
                println!("Error reading from stream: {}", e);
                continue;
            }

            let mut evidence = match from_slice::<Evidence>(&buffer) {
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
    let violation_queue = Arc::clone(&violation);
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
                    let violation = violation_queue.read().await;
                    violation.get(&key).cloned()
                } {
                    // Check if the old evidence is older than 10 minutes than the new one
                    if evidence.timestamp - existing.timestamp < 10 * 60 * 1000 {
                        // Skip inserting this evidence as it's a duplicate within 10 minutes
                        continue;
                    }
                }

                // Insert or update the violation record
                if person.violation.len() > 0 {
                    new_violation = true;
                    {
                        let mut violation = violation_queue.write().await;
                        violation.insert(key, evidence.clone());
                    }
                }
            }

            // If no new violation detected, skip processing
            if new_violation {
                let image = match fs::read(format!("/tmp/{}.jpg", evidence.camera_id)).await {
                    Ok(d) => d,
                    Err(_) => continue,
                };

                // Save the evidence and the image to ./evidence
                // Create ./evidence directory if it doesn't exist
                let _ = fs::create_dir_all("./evidence").await;
                fs::write(&format!("./evidence/{}.jpg", evidence.id), &image)
                    .await
                    .unwrap();
                fs::write(
                    &format!("./evidence/{}.json", evidence.id),
                    &serde_json::to_string(&evidence).unwrap(),
                )
                .await
                .unwrap();
            }

            sleep(Duration::from_millis(100)).await;
        }
    });

    // WEBHOOK SENDER THREAD: Send the saved evidence to configured webhooks
    let device_clone = Arc::clone(&device);
    let _ = tokio::spawn(async move {
        loop {
            // Load all evidences from ./evidence/{}.json (NOT ./evidence/uploaded.{}.json) that have not been sent yet
            let mut evidences = {
                let mut evidences = Vec::new();
                let entries = match fs::read_dir("./evidence").await {
                    Ok(entries) => entries,
                    Err(_) => {
                        sleep(Duration::from_millis(100)).await;
                        continue;
                    }
                };

                let mut dir = entries;
                while let Ok(Some(entry)) = dir.next_entry().await {
                    let file_name = entry.file_name().into_string().unwrap();
                    if file_name.ends_with(".json") && !file_name.starts_with("uploaded.") {
                        let data = match fs::read_to_string(entry.path()).await {
                            Ok(d) => d,
                            Err(_) => continue,
                        };
                        let evidence: Evidence = match serde_json::from_str(&data) {
                            Ok(e) => e,
                            Err(_) => continue,
                        };
                        evidences.push(evidence);
                    }
                }
                evidences
            };

            for evidence in evidences.drain(..) {
                let webhook = {
                    let device = device_clone.read().await;
                    match device.processor.webhook.clone() {
                        Some(v) => v,
                        None => continue,
                    }
                };

                // Send evidence and captured image via webhook
                let image = match fs::read(format!("./evidence/{}.jpg", evidence.id)).await {
                    Ok(d) => d,
                    Err(_) => continue,
                };
                let payload = serde_json::to_string(&evidence).unwrap();

                if webhook
                    .send_evidence(payload, image.clone(), &evidence.id)
                    .await
                {
                    // Rename the evidence files to mark them as uploaded
                    let _ = fs::rename(
                        format!("./evidence/{}.json", evidence.id),
                        format!("./evidence/uploaded.{}.json", evidence.id),
                    )
                    .await;
                    let _ = fs::rename(
                        format!("./evidence/{}.jpg", evidence.id),
                        format!("./evidence/uploaded.{}.jpg", evidence.id),
                    )
                    .await;
                }
            }

            sleep(Duration::from_millis(100)).await;
        }
    });

    // WEBHOOK UPDATER THREAD: Periodically update webhook info from Device
    let device_clone = Arc::clone(&device);
    let _ = tokio::spawn(async move {
        loop {
            let (processor, camera, webhook) = {
                let device = device_clone.read().await;
                let webhook = match device.processor.webhook.clone() {
                    Some(v) => v,
                    None => {
                        sleep(Duration::from_secs(10)).await;
                        continue;
                    }
                };

                let camera = device.camera.values().cloned().collect::<Vec<Camera>>();

                (device.processor.clone(), camera, webhook)
            };

            // Send a heartbeat or info update to the webhook
            let payload = serde_json::json!({
                "processor": processor,
                "camera": camera,
            });

            let _ = webhook.send_update(payload.to_string()).await;

            sleep(Duration::from_secs(10)).await;
        }
    });

    // INFERENCE ENGINE THREAD: Spawn inference engine thread with auto-restart capability
    let device_clone = Arc::clone(&device);
    let _ = tokio::spawn(async move {
        // In simulation mode, just wait indefinitely (run simulator manually in another terminal)
        if simulation_mode {
            println!("[Inference Engine] Simulation mode active, not starting real engine");
            loop {
                sleep(Duration::from_secs(60)).await;
            }
        }

        let command = if simulation_mode {
            println!("[Inference Engine] Running in SIMULATION MODE");
            "python3 -m simulator.main"
        } else {
            "source setup.sh && python3 -m inference.main"
        };

        // Normal mode: spawn the real Inference Engine with auto-restart
        let mut count = 0;
        let delay = Duration::from_secs(5);

        let mut version = {
            let device = device_clone.read().await;
            device.processor.version
        };

        loop {
            count += 1;
            println!("[Inference Engine] Starting... (attempt #{})", count);

            // Spawn the Python inference script with venv activation
            let mut child = match Command::new("bash")
                .arg("-c")
                .arg(command)
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

            // Monitor loop: check for process exit OR version change
            let exit_status = loop {
                // Check if the child has exited
                match child.try_wait() {
                    Ok(Some(status)) => break Some(status),
                    Ok(None) => {} // still running
                    Err(e) => {
                        eprintln!("[Inference Engine] Error checking process: {}", e);
                        break None;
                    }
                }

                // Check if version changed
                let new_version = {
                    let device = device_clone.read().await;
                    device.processor.version
                };

                if new_version != version {
                    println!(
                        "[Inference Engine] Version changed ({} -> {}), restarting...",
                        version, new_version
                    );
                    version = new_version;
                    let _ = child.kill();
                    let _ = child.wait(); // reap the process
                    break None;
                }

                sleep(Duration::from_secs(1)).await;
            };

            // Handle clean exit (don't restart)
            if let Some(status) = exit_status {
                if status.success() {
                    println!("[Inference Engine] Exited successfully (code 0)");
                    println!("[Inference Engine] Assuming intentional shutdown, not restarting");
                    break;
                } else {
                    eprintln!("[Inference Engine] Crashed with status: {}", status);
                }
            }

            // Wait before restarting to prevent rapid restart loops
            println!("[Inference Engine] Restarting in {:?}...", delay);
            sleep(delay).await;
        }

        println!("[Inference Engine] Thread exiting");
    });

    // HTTP SERVER THREAD: Serve web interface API
    let port = processor.address.port;
    let addr = SocketAddr::from((processor.address.host, port));
    let _ = HttpServer::new(move || {
        let cors = Cors::permissive();

        println!("[HTTP Server] Listening on http://{}", addr);

        App::new()
            .wrap(cors)
            .app_data(Data::new(device.clone()))
            .app_data(Data::new(reading.clone()))
            .app_data(Data::new(violation.clone()))
            .wrap(Logger::default())
            .configure(routes::configure_routes)
            .service(Files::new("/evidence", "./evidence").show_files_listing())
    })
    .bind(addr)
    .unwrap()
    .run()
    .await;

    println!("\n=== SCM Processor Shutdown ===");
}
