use std::error::Error;

use clap::Parser;

use daemon::database::{init_database, insert_new_parameterized_job};
use tarascope::shader::KaleidoArgs;
/// Reads render parameters from the command line and queues the parameterized job.

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenv::dotenv().ok();
    let kargs = KaleidoArgs::parse();

    println!("{:#?}", kargs.json());
    
    let pool = init_database().await?;
    Ok(insert_new_parameterized_job(&pool, kargs).await?)
}