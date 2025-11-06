use std::{env::var, error::Error};

use clap::Parser;
use clap_derive::Parser;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tarascope::{
    RenderStatus,
    encoder::stitch_video,
    run_kaleidoscope,
    shader::{KaleidoArgs, OutputArgs},
};
use tokio::sync::mpsc::unbounded_channel;

use crate::database::init_database;

pub mod database;

/// Generate and store kaleidoscopes in postgres
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    out: OutputArgs,
}

async fn register_new_kaleidoscope(
    pool: &Pool<Postgres>,
    id: String,
    params: String,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("INSERT INTO public.tarascope (id, parameters) VALUES (uuid($1), json($2))")
        .bind(id)
        .bind(params)
        .execute(pool)
        .await?;
    Ok(())
}

async fn set_kaleidoscope_to_waiting(pool: &Pool<Postgres>, id: String) -> Result<(), Box<dyn Error>> {
    sqlx::query("UPDATE public.tarascope SET status=1 WHERE id = uuid($1)")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
async fn set_kaleidoscope_to_running(pool: &Pool<Postgres>, id: String) -> Result<(), Box<dyn Error>> {
    sqlx::query("UPDATE public.tarascope SET status=2 WHERE id = uuid($1)")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn set_kaleidoscope_to_failed(pool: &Pool<Postgres>, id: String) -> Result<(), Box<dyn Error>> {
    sqlx::query("UPDATE public.tarascope SET status=4 WHERE id = uuid($1)")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn set_kaleidoscope_to_done(pool: &Pool<Postgres>, id: String) -> Result<(), Box<dyn Error>> {
    sqlx::query("UPDATE public.tarascope SET status=3 WHERE id = uuid($1)")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn insert_frame(pool: &Pool<Postgres>, update: RenderStatus) -> Result<(), Box<dyn Error>> {
    sqlx::query("INSERT INTO public.frames (kaleidoid, frame_count) VALUES (uuid($1), $2)")
        .bind(update.id)
        .bind(update.frame)
        .execute(pool)
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let _ = dotenv::dotenv().ok();
    let pool = init_database().await.unwrap();

    let (sender, receiver) = unbounded_channel::<String>();

    let kaleidoargs = KaleidoArgs::random(args.out);

    // status task
    let r_pool = pool.clone();
    tokio::spawn(async move {
        let mut receiver = receiver;
        loop {
            if let Some(msg) = receiver.recv().await {
                println!("{:?}", msg);

                let data: RenderStatus = serde_json::from_str(msg.as_str()).unwrap();
                insert_frame(&r_pool, data).await.unwrap();
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
