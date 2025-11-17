use std::{env::var, error::Error};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Serialize, Deserialize, Debug)]
struct MediaContainer {
    id: String
}

fn get_accesstoken() -> String {
    var("IG_ACCESSTOKEN").expect("no instagram token found")
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
async fn create_media_container(media_url: &String) -> Result<MediaContainer, reqwest::Error> {
    let token = String::new();
    let ig_id = String::new();
    
    let body = json!({
        "image_url": media_url
    });

    
    let client = Client::new();
    let url = format!("https://graph.instagram.com/v24.0/{}/media", ig_id);
    let res = client
        .post(url)
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .json(&body);

    let response = res.send().await?;

    let response_as_json: MediaContainer = response.json().await?;
    Ok(response_as_json)
}

async fn upload_media(filepath: String) -> Result<String, Box<dyn Error>> {
    //get catbox userhash
    let userhash = var("CATBOX_USERHASH").ok();
    catbox::file::from_file(filepath, userhash).await
}

async fn upload_to_container(container: MediaContainer, media_url: &String) -> Result<String, reqwest::Error> {
    let token = String::new();

    let client = Client::new();
    let url = format!("https://rupload.facebook.com/ig-api-upload/v24.0/{}", container.id);
    let res = client
        .post(url)
        .bearer_auth(token)
        .header("file_url", media_url);

    let response = res.send().await?;
    let json: Value = response.json().await?;
    Ok(json.to_string())
}

pub async fn create_instagram_post(filepath: String) {
    //upload file to catbox
    let url = upload_media(filepath).await.unwrap();

    //register new media container
    let container = create_media_container(&url).await.unwrap();

    //upload to instagram
    let ig_upload = upload_to_container(container, &url).await.unwrap();

    //link with data in database

}
