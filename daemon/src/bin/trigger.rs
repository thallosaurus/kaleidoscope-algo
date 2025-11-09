use std::error::Error;

use daemon::database::{init_database, trigger_generation};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenv::dotenv().ok();

    simple_logger::init()?;
    
    let pool = init_database().await?;
    Ok(trigger_generation(&pool).await?)
}