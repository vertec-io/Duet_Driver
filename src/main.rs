pub mod duet_driver;
use duet_driver::DuetDriver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // FIXME: this needs to come from a .env file 
    let driver = DuetDriver::new("10.21.0.151".to_string()).await;

    driver.send_position(150.0).await?;
    // Give the async loop a moment to process the command.
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(())
}

