use std::sync::Arc;

use daemon::database::{
    all_kaleidoscopes, init_database, insert_new_parameterized_job, single_kaleidoscopes,
};
use handlebars::Handlebars;
use rocket::{State, fs::NamedFile, get, launch, put, response::content::RawHtml, routes, serde::json::Json, tokio::sync::Mutex};
use serde::Deserialize;
use serde_json::{Map, json};
use sqlx::{Pool, Postgres};
use tarascope::shader::KaleidoArgs;

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

#[launch]
async fn rocket() -> _ {
    let _ = dotenv::dotenv().ok();

    let pool = init_database().await.unwrap();

    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("main", "./index.hbs").unwrap();

    rocket::build()
        .manage(ApiState {
            pool: Arc::new(Mutex::new(pool)),
            handlebars
        })
        .mount("/", routes![frontpage])
        .mount("/api", routes![full, single, new, random])
}
