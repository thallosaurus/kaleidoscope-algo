use std::{
    cell::LazyCell,
    env::current_dir,
    error::Error,
    rc::Rc,
    sync::{Arc, LazyLock, OnceLock},
};

use clap::Parser;
use clap_derive::Parser;
use serde_json::to_string;
use sqlx::{Pool, Postgres, postgres::PgListener};
use tarascope::{
    RenderStatus,
    encoder::stitch_video,
    run_kaleidoscope,
    shader::{KaleidoArgs, OutputArgs},
};
use tokio::sync::{
    Mutex,
    mpsc::{self, unbounded_channel},
};

use crate::database::{
    get_specific_job_parameters, init_database, insert_frame, register_new_kaleidoscope,
    set_kaleidoscope_to_done, set_kaleidoscope_to_waiting,
};

pub mod database;

static MAX_QUEUE_ITEMS: usize = 1;
static OUTPUT_DIR: OnceLock<String> = OnceLock::new();
fn set_cwd() {
    println!("Setting CWD");
    let cwd = current_dir().expect("no current working directory");
    let path = cwd.to_string_lossy().to_string();
    OUTPUT_DIR.set(path).unwrap_or_else(|_| {
        eprintln!("CWD is already set");
    });
}

fn get_cwd() -> &'static str {
    OUTPUT_DIR.get().expect("CWD is not set")
}

enum RenderQueueRequest {
    Random,
    Parameterized(String),
}

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
    listener.listen("queue_parameter").await?;

    let r_pool = Arc::new(Mutex::new(pool));
    let r_args = Arc::new(Mutex::new(args));

    // for the start allocate a size 2 render
    let (tx, mut rx) = mpsc::channel::<RenderQueueRequest>(MAX_QUEUE_ITEMS);

    // render queue task
    let _queue_task = tokio::spawn(async move {
        let pool = r_pool.clone();
        let args = r_args.clone();
        loop {
            if let Some(req) = rx.recv().await {
                let pool = pool.lock().await;
                let args = args.lock().await;
                match req {
                    RenderQueueRequest::Random => {
                        println!("Starting new random job");
                        let job = KaleidoArgs::random(String::from(get_cwd()));

                        let j = job.json();
                        let id = job.get_id();
                        register_new_kaleidoscope(&pool, &id, j.to_string())
                            .await
                            .unwrap();
                        render(&pool, &job).await.unwrap();
                        //tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                        println!("Finished Render Job");
                    }
                    RenderQueueRequest::Parameterized(id) => {
                        let job = get_specific_job_parameters(&pool, &id).await.unwrap();
                        println!("Starting new parameterized job {}", id);
                        render(&pool, &job).await.unwrap();
                        println!("Finished Render Job");
                    }
                }
            } else {
                println!("queue closed");
                break;
            }
        }
    });

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
                    "test" => println!("test notif!"),
                    "test2" => println!("test2 notif!"),

                    // database sent request for image generation, add to queue
                    "generate_random" => {
                        println!("queue capacity: {}", tx.capacity());
                        if let Err(e) = tx.try_send(RenderQueueRequest::Random) {
                            eprintln!("error while adding task to render queue: {}", e);
                            continue;
                        } else {
                            println!("queued next random generation");
                        }
                    },
                    "queue_parameter" =>  {
                        if let Err(e) = tx.try_send(RenderQueueRequest::Parameterized(String::from(data))) {
                            eprintln!("error while adding task to render queue: {}", e);
                            continue;
                        } else {
                            println!("queued next parameterized generation");
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

async fn render(pool: &Pool<Postgres>, kaleidoargs: &KaleidoArgs) -> Result<(), Box<dyn Error>> {
    //let pool = init_database().await.unwrap();
    let (sender, receiver) = unbounded_channel::<String>();

    // status task
    let p = pool.clone();
    tokio::spawn(async move {
        let mut receiver = receiver;
        loop {
            if let Some(msg) = receiver.recv().await {
                println!("{:?}", msg);

                let data: RenderStatus = serde_json::from_str(msg.as_str()).unwrap();
                insert_frame(&p, data).await.unwrap();
                continue;
            } else {
                break;
            }
        }
    });

    /*     let j = kaleidoargs.json();
    register_new_kaleidoscope(&pool, &id, j.to_string()).await?; */
    let id = kaleidoargs.get_id();

    set_kaleidoscope_to_waiting(pool, id).await?;

    let output = run_kaleidoscope(&kaleidoargs, sender).await?;

    if output.exit_status.success() {
        stitch_video(&kaleidoargs).unwrap();
        set_kaleidoscope_to_done(&pool, kaleidoargs.get_id()).await?;
    }

    Ok(())
}
