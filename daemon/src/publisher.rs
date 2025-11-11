use std::{env::var, error::Error};

use reqwest::Client;

/// Example Request:
/// ```sh
/// curl -X POST "https://<HOST_URL>/<LATEST_API_VERSION>/<IG_ID>/media"
///     -H "Content-Type: application/json" 
///     -H "Authorization: Bearer <ACCESS_TOKEN>" 
///     -d '{
///           "image_url":"https://www.example.com/images/bronz-fonz.jpg"
///         }'
/// ```
pub async fn create_media_container(ig_id: String) {
    let url = format!("https://graph.instagram.com/v24.0/{}/media", ig_id);
    let client = Client::new();
    let res = client.post(url);
}

async fn upload_media(filepath: String) -> Result<String, Box<dyn Error>> {
    //get catbox userhash
    let userhash = var("CATBOX_USERHASH").ok();
    catbox::file::from_file(filepath, userhash).await
}

pub async fn create_instagram_post(filepath: String) {
    //upload file to catbox
    let url = upload_media(filepath).await.unwrap();

        //register new media container
    
        //upload to instagram
    
        //link with data in database

}