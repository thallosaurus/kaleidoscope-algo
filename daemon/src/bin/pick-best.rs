use std::error::Error;

use daemon::database::{init_database, insert_new_parameterized_job, todays_done_jobs};
use tarascope::shader::KaleidoArgs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenv::dotenv().ok();
    let kargs = KaleidoArgs::random();

    println!("{:#?}", kargs.json());
    
    let pool = init_database().await?;
    let best = todays_done_jobs(&pool).await?;
    println!("{:?}", best);
    Ok(())
    //Ok(insert_new_parameterized_job(&pool, kargs).await?)
}