use std::error::Error;

use daemon::database::{init_database, insert_new_parameterized_job, insert_new_parameterized_still_job};
use tarascope::shader::KaleidoArgs;
/// Creates a random configuration and queues it in the database as a still-image job.

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenv::dotenv().ok();
    let kargs = KaleidoArgs::random();

    println!("{:#?}", kargs.json());
    
    let pool = init_database().await?;
    Ok(insert_new_parameterized_still_job(&pool, kargs).await?)
}