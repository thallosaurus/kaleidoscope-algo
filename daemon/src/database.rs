use std::{env::var, error::Error};

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{
    Pool, Postgres,
    postgres::{PgPoolOptions, PgRow, types},
    prelude::FromRow,
};
use tarascope::{RenderStatus, shader::KaleidoArgs};

use crate::publisher::PostPermalink;

pub async fn init_database() -> Result<Pool<Postgres>, Box<dyn Error>> {
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
    Ok(pool)
}

pub async fn all_kaleidoscopes(pool: &Pool<Postgres>) -> Result<Vec<Showcase>, Box<dyn Error>> {
    let d = sqlx::query_as::<_, Showcase>(
        "SELECT id::text, video, gif, thumbnail, ts::timestamp FROM showcase ORDER BY ts DESC",
    )
    .fetch_all(pool)
    .await?;

    Ok(d)
}

pub async fn single_kaleidoscopes(
    pool: &Pool<Postgres>,
    id: &String,
) -> Result<Vec<Showcase>, Box<dyn Error>> {
    let d = sqlx::query_as::<_, Showcase>("SELECT id::text, video, gif, thumbnail, ts::timestamp FROM showcase WHERE id = uuid($1) ORDER BY ts DESC")
    .bind(id)
    .fetch_all(pool)
    .await?;

    Ok(d)
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Showcase {
    // SELECT id, video, gif, thumbnail, ts FROM showcase ORDER BY ts DESC
    id: String,
    video: String,
    gif: String,
    thumbnail: String,
    ts: NaiveDateTime,
}

pub async fn trigger_generation(pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    sqlx::query("NOTIFY generate_random").execute(pool).await?;
    Ok(())
}

pub async fn register_new_kaleidoscope(
    pool: &Pool<Postgres>,
    id: &String,
    params: String,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("INSERT INTO public.tarascope (id, parameters) VALUES (uuid($1), json($2))")
        .bind(id)
        .bind(params)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_kaleidoscope_to_waiting(
    pool: &Pool<Postgres>,
    id: &String,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("UPDATE public.tarascope SET status=1 WHERE id = uuid($1)")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
async fn set_kaleidoscope_to_running(
    pool: &Pool<Postgres>,
    id: &String,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("UPDATE public.tarascope SET status=2 WHERE id = uuid($1)")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn set_kaleidoscope_to_failed(
    pool: &Pool<Postgres>,
    id: &String,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("UPDATE public.tarascope SET status=4 WHERE id = uuid($1)")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_kaleidoscope_to_done(
    pool: &Pool<Postgres>,
    id: &String,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("UPDATE public.tarascope SET status=3 WHERE id = uuid($1)")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn insert_frame(
    pool: &Pool<Postgres>,
    update: RenderStatus,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("INSERT INTO public.frames (kaleidoid, frame_count) VALUES (uuid($1), $2)")
        .bind(update.id)
        .bind(update.frame)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn insert_new_parameterized_job(
    pool: &Pool<Postgres>,
    kargs: KaleidoArgs,
) -> Result<(), Box<dyn Error>> {
    let id = kargs.get_id();
    register_new_kaleidoscope(pool, &id, kargs.json().to_string()).await?;

    sqlx::query("SELECT pg_notify('queue_parameters', $1)")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn insert_new_parameterized_still_job(
    pool: &Pool<Postgres>,
    kargs: KaleidoArgs,
) -> Result<(), Box<dyn Error>> {
    let id = kargs.get_id();
    register_new_kaleidoscope(pool, &id, kargs.json().to_string()).await?;

    sqlx::query("SELECT pg_notify('queue_still', $1)")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_specific_job_parameters(
    pool: &Pool<Postgres>,
    id: &String,
) -> Result<KaleidoArgs, Box<dyn Error>> {
    let q: (String,) = sqlx::query_as("SELECT parameters::text FROM tarascope WHERE id = uuid($1)")
        .bind(id)
        .fetch_one(pool)
        .await?;

    let vvvv: Value = serde_json::from_str(&q.0).unwrap();

    println!("db: {:?}", vvvv);

    Ok(KaleidoArgs::from_json(vvvv).unwrap())
}

pub async fn todays_done_jobs(
    pool: &Pool<Postgres>,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let q: Vec<(String, String)> = sqlx::query_as(
        "SELECT id::text, thumbnail from showcase where cast(ts as DATE) = cast(now() as DATE)",
    )
    .fetch_all(pool)
    .await?;
    Ok(q)
}

pub async fn insert_instagram_post(
    pool: &Pool<Postgres>,
    kaleido: &String,
    post: &PostPermalink
) -> Result<(), Box<dyn Error>> {
    sqlx::query("INSERT INTO public.instagram_posts (kaleidoid, permalink) VALUES (uuid($1), $2)")
        .bind(kaleido)
        .bind(&post.permalink)
        .execute(pool)
        .await?;
    Ok(())
}
