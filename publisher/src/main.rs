use std::{env::{self, current_dir, var}, error::Error};

use clap::Parser;
use clap_derive::Parser;
use sqlx::postgres::PgPoolOptions;
use tarascope::{
    encoder::stitch_video, run_kaleidoscope, shader::{KaleidoArgs, OutputArgs}
};


/// Generate and store kaleidoscopes in postgres
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    out: OutputArgs
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let _ = dotenv::dotenv().ok();
    let host = var("PG_HOST").unwrap_or("localhost".to_string());
    let username = var("PG_USER").unwrap_or("postgres".to_string());
    let password = var("PG_PASS").unwrap_or("password".to_string());
    let database = var("PG_DB").unwrap_or("postgres".to_string());
    let connection_uri = format!("postgres://{}:{}@{}/{}", username, password, host, database);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_uri.as_str())
        .await?;

    println!("Connection to Database successful");

    println!("{}", args.out.output_dir);
    let kaleidoargs = KaleidoArgs::random(args.out);
    let output = run_kaleidoscope(&kaleidoargs).unwrap();

    let j = kaleidoargs.json();

    if output.exit_status.success() {
        stitch_video(&kaleidoargs).unwrap();
        sqlx::query("INSERT INTO public.tarascope (id, parameters) VALUES (uuid($1), $2)")
            .bind(kaleidoargs.get_id())
            .bind(j)
            .execute(&pool)
            .await?;
    }

    Ok(())
}
