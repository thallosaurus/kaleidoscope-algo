use daemon::publisher::create_instagram_post;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    let _ = dotenv().unwrap();
    let fpath = "/Users/rillo/Desktop/testvideo.mp4";
    create_instagram_post(String::from(fpath)).await;
}