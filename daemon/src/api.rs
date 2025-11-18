///Web API Code
use std::sync::Arc;

use crate::database::{
    all_kaleidoscopes, insert_new_parameterized_job, single_kaleidoscopes,
};
use handlebars::Handlebars;
use rocket::{State, fs::FileServer, get, put, response::content::RawHtml, routes, serde::json::Json, tokio::sync::Mutex};
use serde_json::{Map, json};
use sqlx::{Pool, Postgres};
use tarascope::shader::KaleidoArgs;
use tokio::sync::oneshot;

struct ApiState<'a> {
    pool: Arc<Mutex<Pool<Postgres>>>,
    handlebars: Handlebars<'a>
}

#[get("/")]
async fn full(state: &State<ApiState<'_>>) -> String {
    let lock = state.pool.lock().await;
    //let p = lock.acquire().await;
    let res = all_kaleidoscopes(&lock).await.unwrap();

    serde_json::to_string(&res).unwrap()
}

#[put("/", data = "<data>")]
async fn new(state: &State<ApiState<'_>>, data: Json<KaleidoArgs>) -> String {
    println!("{:?}", data);

    let lock = state.pool.lock().await;

    insert_new_parameterized_job(&lock, data.0).await.unwrap();
    String::from("ok")
}

#[put("/random")]
async fn random(state: &State<ApiState<'_>>) -> String {
    let data = KaleidoArgs::random();
    println!("{:?}", data);

    let lock = state.pool.lock().await;

    insert_new_parameterized_job(&lock, data).await.unwrap();
    String::from("ok")
}

#[get("/<id>")]
async fn single(state: &State<ApiState<'_>>, id: &str) -> String {
    let lock = state.pool.lock().await;
    //let p = lock.acquire().await;
    let res = single_kaleidoscopes(&lock, &String::from(id))
        .await
        .unwrap();

    serde_json::to_string(&res).unwrap()
}

#[get("/")]
async fn frontpage(state: &State<ApiState<'_>>) -> Result<RawHtml<String>, std::io::Error> {
    let lock = state.pool.lock().await;
    let data = all_kaleidoscopes(&lock).await.unwrap();

    let mut content = Map::new();
    content.insert("content".to_string(), json!(data));
    let res = state.handlebars.render("main", &content).unwrap();

    Ok(RawHtml(res))
}

pub fn init_api(pool: Arc<Mutex<Pool<Postgres>>>, static_path: String) -> oneshot::Sender<()> {
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("main", "./daemon/index.hbs").unwrap();

    let r = rocket::build()
        .manage(ApiState {
            pool,
            handlebars
        })
        .mount("/", routes![frontpage])
        .mount("/api", routes![full, single, new, random])
        .mount("/assets", FileServer::from(static_path));


    let _ = tokio::spawn(async move {
        let orbit = r.launch().await.expect("rocket failed");
            println!("Rocket launched");

            let _ = shutdown_rx.await;
            println!("Stopping webserver");

            orbit.shutdown().notify();

            println!("API is down");
        });

    shutdown_tx
        //.launch().await
}
