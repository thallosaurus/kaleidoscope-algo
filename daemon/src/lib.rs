use std::{
    env::current_dir,
    error::Error,
    sync::{Arc, OnceLock},
};

use clap::Parser;
use log::{debug, info};
use sqlx::{Pool, Postgres, postgres::PgListener};
use tarascope::{
    Tarascope,
    shader::OutputArgs,
};
use tokio::sync::Mutex;

use crate::{api::init_api, database::init_database, queue::{RenderQueue, RenderQueueRequest}};

pub mod database;
mod queue;
pub mod publisher;
mod api;

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
    let master_pool = init_database().await.unwrap();

    let mut listener = PgListener::connect_with(&master_pool.clone()).await?;
    listener.listen("test").await?;
    listener.listen("test2").await?;
    listener.listen("generate_random").await?;
    listener.listen("queue_parameters").await?;
    listener.listen("queue_still").await?;

    let out = args.out.output_dir;
    
    let tarascopes = Arc::new(Mutex::new(Tarascope::new(String::from(
        out.clone(),
    ))));
    
    let r_pool = Arc::new(Mutex::new(master_pool));
    let r_clone = r_pool.clone();

    let render_queue = RenderQueue::new(r_pool, tarascopes);

    let api = init_api(r_clone, out.clone());

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
                    "queue_still" =>  {
                        if let Err(e) = render_queue.push(RenderQueueRequest::ParameterizedStill(String::from(data))) {
                            continue;
                        }
                    },
                    _ => {
                        println!("unknown channel notification ({})", ch)
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                api.send(()).expect("error while stopping api");
                break
            }
        }
    }
    info!("database listener closed");
    Ok(())
}
