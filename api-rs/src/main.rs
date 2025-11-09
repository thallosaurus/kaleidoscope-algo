use std::sync::Arc;

use daemon::database::{all_kaleidoscopes, init_database, single_kaleidoscopes};
use rocket::{State, get, launch, routes, tokio::sync::Mutex};
use sqlx::{Pool, Postgres};

struct ApiState {
    pool: Arc<Mutex<Pool<Postgres>>>
}

#[get("/")]
async fn full(state: &State<ApiState>) -> String {
    let lock = state.pool.lock().await;
    //let p = lock.acquire().await;
    let res = all_kaleidoscopes(&lock).await.unwrap();

    serde_json::to_string(&res).unwrap()
}

#[get("/<id>")]
async fn single(state: &State<ApiState>, id: String) -> String {
    let lock = state.pool.lock().await;
    //let p = lock.acquire().await;
    let res = single_kaleidoscopes(&lock, &id).await.unwrap();

    serde_json::to_string(&res).unwrap()
}


#[launch]
async fn rocket() -> _ {
    let _ = dotenv::dotenv().ok();
    
    let pool = init_database().await.unwrap();

    rocket::build().manage(ApiState {
        pool: Arc::new(Mutex::new(pool))
    }).mount("/api", routes![full, single])
}