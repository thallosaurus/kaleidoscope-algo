use std::{env::var, error::Error, time::Duration};

use log::{debug, error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    task::JoinHandle,
};

use crate::{SharedDatabasePool, database::get_specific_job_parameters};

#[derive(Serialize, Deserialize, Debug)]
struct MediaContainer {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct InstagramPost {
    id: String,
}

#[derive(Deserialize, Debug)]
struct StatusResponse {
    status_code: Option<String>,
}

enum AwaitError {
    ReqwestError(reqwest::Error),
    ApiError,
}

#[derive(Deserialize, Debug)]
struct UploadResponse {
    id: String,
}

#[derive(Deserialize, Debug)]
pub struct PostPermalink {
    id: String,
    pub permalink: String,
}

#[derive(Deserialize, Debug)]
pub enum PostQueueError {
    QueuePushError,
}

static GRAPH_BASE: &str = "https://graph.facebook.com";

fn get_accesstoken() -> String {
    var("IG_ACCESSTOKEN").expect("no instagram token found")
}

fn get_accountid() -> String {
    var("IG_USERID").expect("no instagram id found")
}

/// Example Request:
/// ```sh
/// curl -X POST "https://<HOST_URL>/<LATEST_API_VERSION>/<IG_ID>/media"
///     -H "Content-Type: application/json"
///     -H "Authorization: Bearer <ACCESS_TOKEN>"
///     -d '{
///           "image_url":"https://www.example.com/images/bronz-fonz.jpg"
///         }'
/// ```
async fn create_media_container(
    client: &Client,
    media_url: &String,
) -> Result<MediaContainer, reqwest::Error> {
    let token = get_accesstoken();
    let ig_id = get_accountid();

    let body = json!({
        "media_type": "REELS",
        "video_url": media_url,
        "caption": "a post, made by an api. who would've thought i find out how this stuff works?"
    });

    let url = format!("{}/{}/media", GRAPH_BASE, ig_id);
    let res = client
        .post(url)
        .query(&[("access_token", &token)])
        .json(&body);

    let response = res.send().await?;
    println!("{:?}", response);

    let response_as_json: MediaContainer = response.json().await?;
    Ok(response_as_json)
}

async fn upload_media(filepath: String) -> Result<String, Box<dyn Error>> {
    //get catbox userhash
    let userhash = var("CATBOX_USERHASH").ok();
    catbox::file::from_file(filepath, userhash).await
}

async fn upload_to_container(
    client: &Client,
    container: MediaContainer,
) -> Result<UploadResponse, reqwest::Error> {
    let token = get_accesstoken();
    let ig_id = get_accountid();

    let body = json!({
        "creation_id": container.id
    });

    let url = format!("{}/{}/media_publish", GRAPH_BASE, ig_id);
    let res = client
        .post(url)
        .query(&[("access_token", &token)])
        .json(&body);

    let response = res.send().await?;
    let json: UploadResponse = response.json().await?;
    Ok(json)
}

async fn wait_until_finished(client: &Client, container: &MediaContainer) -> anyhow::Result<()> {
    let url = format!("{}/{}", GRAPH_BASE, container.id);
    let token = get_accesstoken();
    loop {
        let resp = client
            .get(&url)
            .query(&[("fields", "status_code"), ("access_token", &token)])
            .send()
            .await?;

        let json: StatusResponse = resp.json().await?;

        match json.status_code.as_deref() {
            Some("FINISHED") => {
                println!("Media done");
                return Ok(());
            }
            Some("ERROR") => {
                anyhow::bail!("api returned error")
            }
            _ => {
                println!("Not done yet");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    }
}

async fn get_permalink(
    client: &Client,
    response: UploadResponse,
) -> Result<PostPermalink, reqwest::Error> {
    let url = format!("{}/{}?fields=permalink", GRAPH_BASE, response.id);
    let token = get_accesstoken();

    let resp = client
        .get(&url)
        .query(&[("fields", "status_code"), ("access_token", &token)])
        .send()
        .await?;

    Ok(resp.json().await?)
}

pub async fn create_instagram_post(filepath: String) {
    //upload file to catbox
    let url = upload_media(filepath).await.unwrap();
    //let url = String::from("https://files.catbox.moe/byvuxj.mp4");
    println!("catbox url: {}", url);

    //register new media container
    let client = Client::new();
    let container = create_media_container(&client, &url).await.unwrap();
    println!("{:?}", container);

    wait_until_finished(&client, &container).await.unwrap();
    //upload to instagram
    let ig_upload = upload_to_container(&client, container).await.unwrap();
    println!("{:?}", ig_upload);

    //link with data in database
    let post = get_permalink(&client, ig_upload).await.unwrap();
}

#[derive(Debug)]
pub enum PostQueueRequest {
    Instagram(String),
}

pub struct PostQueue {
    pool: SharedDatabasePool,
    queue_sender: UnboundedSender<PostQueueRequest>,
    _handle: JoinHandle<()>,
}

impl PostQueue {
    pub fn new(pool: SharedDatabasePool) -> Self {
        let (queue_sender, rx) = unbounded_channel::<PostQueueRequest>();

        Self {
            pool: pool.clone(),
            queue_sender,
            _handle: Self::task(pool, rx),
        }
    }

    fn task(
        pool: SharedDatabasePool,
        mut rx: UnboundedReceiver<PostQueueRequest>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                if let Some(req) = rx.recv().await {
                    match req {
                        PostQueueRequest::Instagram(object_id) => {
                            info!("New Post request");
                            let lock = pool.lock().await;
                            let json = get_specific_job_parameters(&lock, &object_id)
                                .await
                                .unwrap();
                            drop(lock);

                            //create_instagram_post(String::from(fpath)).await;
                        }
                    }
                }
            }
        })
    }

    pub fn push(&self, request: PostQueueRequest) -> Result<(), PostQueueError> {
        //debug!("queue capacity: {}", self.queue_sender.capacity());
        debug!("Adding {:?} to the queue", request);
        if let Err(e) = self.queue_sender.send(request) {
            error!("error while adding task to render queue: {}", e);
            Err(PostQueueError::QueuePushError)
            //Err(e)
        } else {
            Ok(())
        }
    }
}
