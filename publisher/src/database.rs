use std::{env::var, error::Error};

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

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
