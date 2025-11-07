use std::{error::Error, rc::Rc, sync::Arc};

use clap::Parser;
use clap_derive::Parser;
use sqlx::{Pool, Postgres, postgres::PgListener};
use tarascope::{
    RenderStatus,
    encoder::stitch_video,
    run_kaleidoscope,
    shader::{KaleidoArgs, OutputArgs},
};
use tokio::sync::{Mutex, mpsc::{self, unbounded_channel}};

use crate::database::{init_database, insert_frame, register_new_kaleidoscope, set_kaleidoscope_to_done};

pub mod database;

enum RenderQueueRequest {
    Random
}

/// Generate and store kaleidoscopes in postgres
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    out: OutputArgs,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let _ = dotenv::dotenv().ok();
    let pool = init_database().await.unwrap();

    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen("test").await?;
    listener.listen("test2").await?;
    listener.listen("generate_random").await?;

    let r_pool = Arc::new(Mutex::new(pool));
    let r_args = Arc::new(Mutex::new(args));

    // for the start allocate a size 2 render
    let (tx, mut rx) = mpsc::channel::<RenderQueueRequest>(1);

    // render queue task
    tokio::spawn(async move {
        let pool = r_pool.clone();
        let args = r_args.clone();
        loop {
            while let Some(req) = rx.recv().await {
                match req {
                    RenderQueueRequest::Random => {
                        println!("Starting new random job");
                        let pool = pool.lock().await;
                        let args = args.lock().await;
                        render(&pool, &args).await.unwrap();
                    },
                }
            }
        }
    });

    // main event loop

    loop {
        //let r_pool = r_pool.clone();
        //let r_args = r_args.clone();
        tokio::select! {
            Ok(Some(msg)) = listener.try_recv() => {
                // got notification

                let ch = msg.channel();
                match ch {
                    "test" => println!("test notif!"),
                    "test2" => println!("test2 notif!"),

                    // database sent request for image generation, add to queue
                    "generate_random" => {
                        println!("queued new random generation");

                        tx.send(RenderQueueRequest::Random).await.expect("render queue is full");
                    },
                    _ => {
                        println!("unknown channel notification ({})", ch)
                    }
                }
            }
        }
    }
}



async fn render(pool: &Pool<Postgres>, args: &Args) -> Result<(), Box<dyn Error>> {
    //let pool = init_database().await.unwrap();
    let (sender, receiver) = unbounded_channel::<String>();

    //let out = args.out.clone();

    let kaleidoargs = KaleidoArgs::random(args.out.clone());

    // status task
    let p = pool.clone();
    tokio::spawn(async move {
        let mut receiver = receiver;
        loop {
            if let Some(msg) = receiver.recv().await {
                println!("{:?}", msg);

                let data: RenderStatus = serde_json::from_str(msg.as_str()).unwrap();
                insert_frame(&p, data).await.unwrap();
                continue
            } else {
                break;
            }
        }
    });

    let j = kaleidoargs.json();
    register_new_kaleidoscope(&pool, kaleidoargs.get_id(), j.to_string()).await?;

    let output = run_kaleidoscope(&kaleidoargs, sender).await?;

    if output.exit_status.success() {
        stitch_video(&kaleidoargs).unwrap();
        set_kaleidoscope_to_done(&pool, kaleidoargs.get_id()).await?;
    }

    Ok(())
}
