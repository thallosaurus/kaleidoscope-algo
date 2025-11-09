use std::sync::Arc;

use daemon::database::{
    all_kaleidoscopes, init_database, insert_new_parameterized_job, single_kaleidoscopes,
};
use rocket::{State, get, launch, put, routes, serde::json::Json, tokio::sync::Mutex};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tarascope::shader::KaleidoArgs;

struct ApiState {
    pool: Arc<Mutex<Pool<Postgres>>>,
}

#[get("/")]
async fn full(state: &State<ApiState>) -> String {
    let lock = state.pool.lock().await;
    //let p = lock.acquire().await;
    let res = all_kaleidoscopes(&lock).await.unwrap();

    serde_json::to_string(&res).unwrap()
}

#[put("/", data = "<data>")]
async fn new(state: &State<ApiState>, data: Json<KaleidoArgs>) -> String {
    println!("{:?}", data);

    let lock = state.pool.lock().await;

    insert_new_parameterized_job(&lock, data.0).await.unwrap();
    String::from("ok")
}

#[put("/random")]
async fn random(state: &State<ApiState>) -> String {
    let data = KaleidoArgs::random();
    println!("{:?}", data);

    let lock = state.pool.lock().await;

    insert_new_parameterized_job(&lock, data).await.unwrap();
    String::from("ok")
}

#[get("/<id>")]
async fn single(state: &State<ApiState>, id: &str) -> String {
    let lock = state.pool.lock().await;
    //let p = lock.acquire().await;
    let res = single_kaleidoscopes(&lock, &String::from(id))
        .await
        .unwrap();

    serde_json::to_string(&res).unwrap()
}

#[launch]
async fn rocket() -> _ {
    let _ = dotenv::dotenv().ok();

    let pool = init_database().await.unwrap();

    rocket::build()
        .manage(ApiState {
            pool: Arc::new(Mutex::new(pool)),
        })
        .mount("/api", routes![full, single, new, random])
}
