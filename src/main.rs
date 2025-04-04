use duet_driver::DuetDriver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // FIXME: this needs to come from a .env file 
    let driver = DuetDriver::new("10.21.0.151".to_string());

    driver.send_position(50.0, 50.0).await?;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    driver.send_position(150.0, 50.0).await?;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    driver.send_position(50.0, 50.0).await?;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(())
}

