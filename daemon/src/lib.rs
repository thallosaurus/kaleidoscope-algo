use std::{
    env::current_dir,
    error::Error,
    sync::{Arc, OnceLock},
};

use clap::Parser;
use clap_derive::Parser;
use log::debug;
use sqlx::{Pool, Postgres, postgres::PgListener};
use tarascope::{
    Tarascope,
    shader::OutputArgs,
};
use tokio::sync::Mutex;

use crate::{database::init_database, queue::{RenderQueue, RenderQueueRequest}};

pub mod database;
mod queue;

pub type SharedDatabasePool = Arc<Mutex<Pool<Postgres>>>;
pub type SharedTarascope = Arc<Mutex<Tarascope>>;

/// Generate and store kaleidoscopes in postgres
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    out: OutputArgs,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    // parse cli args
    let args = Args::parse();

    let _ = dotenv::dotenv().ok();
    let pool = init_database().await.unwrap();

    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen("test").await?;
    listener.listen("test2").await?;
    listener.listen("generate_random").await?;
    listener.listen("queue_parameters").await?;
    
    let tarascopes = Arc::new(Mutex::new(Tarascope::new(String::from(
        args.out.output_dir,
    ))));
    
    let r_pool = Arc::new(Mutex::new(pool));
    let render_queue = RenderQueue::new(r_pool, tarascopes);

    // main event loop
    // listens for database notifications and acts upon them.
    // Spawns the tasks responsible for rendering
    loop {
        tokio::select! {
            Ok(Some(msg)) = listener.try_recv() => {
                // got notification

                let ch = msg.channel();
                let data = msg.payload();
                match ch {
                    "test" => debug!("test notif!"),
                    "test2" => debug!("test2 notif!"),

                    // database sent request for image generation, add to queue
                    "generate_random" => {
                        if let Err(e) = render_queue.push(RenderQueueRequest::RandomAnimated) {
                            continue;
                        }
                    },
                    "queue_parameters" =>  {
                        if let Err(e) = render_queue.push(RenderQueueRequest::ParameterizedAnimated(String::from(data))) {
                            continue;
                        }
                    },
                    _ => {
                        println!("unknown channel notification ({})", ch)
                    }
                }
            }
        }
    }
}
