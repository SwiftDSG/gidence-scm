use models::processor::Processor;
use std::process::{Command, Stdio};
use tokio::time::{Duration, sleep};

mod models;


#[tokio::main]
async fn main() {
    println!("=== SCM Processor Starting ===\n");

    // Load processor configuration
    let processor = Processor::load();
    println!("Processor Configuration:");
    println!("  ID: {}", processor.id);
    println!("  Name: {}", processor.name);
    println!("  Model: {}", processor.model);
    println!("  Cameras: {}", processor.camera.len());
    if let Some(udp) = &processor.udp {
        println!(
            "  UDP: {}.{}.{}.{}:{}",
            udp.host[0], udp.host[1], udp.host[2], udp.host[3], udp.port
        );
    }
    println!();

    // Spawn inference engine thread with auto-restart capability
    let _ = tokio::spawn(async move {
        let mut count = 0;
        let delay = Duration::from_secs(5);
        loop {
            count += 1;
            println!("[Inference Engine] Starting... (attempt #{})", count);

            // Spawn the Python inference script with venv activation
            // Run: bash -c "source setup.sh && python3 inference/main.py"
            let mut child = match Command::new("bash")
                .arg("-c")
                .arg("source setup.sh && python3 inference/main.py")
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

    // TODO: Add UDP receiver here to listen for violations from Python
    // Example:
    // let udp_thread = thread::spawn(move || {
    //     listen_udp(&processor);
    // });

    // TODO: Register with server and start heartbeat
    // Example:
    // if let Some(server) = &processor.server {
    //     server.register(&processor.address).await;
    // }

    println!("\n=== SCM Processor Shutdown ===");
}
