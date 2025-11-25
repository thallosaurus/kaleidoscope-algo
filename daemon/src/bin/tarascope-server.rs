use std::error::Error;

use daemon::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    simple_logger::init_with_env()?;
    Ok(run().await?)
}