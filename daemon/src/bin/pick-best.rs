use std::error::Error;

use daemon::database::{init_database, insert_new_parameterized_job, todays_done_jobs};
use tarascope::shader::KaleidoArgs;
/// Loads and prints today’s completed showcase entries.

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenv::dotenv().ok();
    
    let pool = init_database().await?;
    let best = todays_done_jobs(&pool).await?;
    println!("{:?}", best);
    Ok(())
    //Ok(insert_new_parameterized_job(&pool, kargs).await?)
}