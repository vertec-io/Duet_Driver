use reqwest;
use tokio::sync::mpsc;
use std::error::Error;

// A driver to send position commands to a Duet board.
pub struct DuetDriver {
    duet_ip: String,
    tx: mpsc::Sender<String>,
}

impl DuetDriver {
    // Initializes the driver and spawns an async loop to process position commands.
    pub fn new(duet_ip: String) -> Self {
        let (tx, mut rx) = mpsc::channel::<String>(32);
        let ip_clone = duet_ip.clone();

        // Async loop to process G-code commands.
        tokio::spawn(async move {
            while let Some(gcode) = rx.recv().await {
                let url = format!(
                    "http://{}/rr_gcode?gcode={}",
                    ip_clone,
                    urlencoding::encode(&gcode)
                );
                match reqwest::get(&url).await {
                    Ok(resp) => {
                        if let Ok(body) = resp.text().await {
                            println!("Sent: {} -> Response: {}", gcode, body);
                        } else {
                            println!("Sent: {} -> Failed to read response", gcode);
                        }
                    },
                    Err(e) => eprintln!("Error sending {}: {}", gcode, e),
                }
            }
        });

        DuetDriver { duet_ip, tx }
    }

    // Sends a position command by constructing a G-code command.
    // Only the Y position is needed; the feed rate is fixed to 3000.
    pub async fn send_position(&self, position: f64) -> Result<(), Box<dyn Error>> {
        let gcode = format!("G1 Y{} F3000", position);
        self.tx.send(gcode).await.map_err(|e| e.into())
    }
}
