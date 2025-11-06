use std::env::current_dir;

use tarascope::{run_kaleidoscope, shader::KaleidoArgs};
use tokio::{
    io::{self},
    sync::mpsc::unbounded_channel,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let dir = current_dir().unwrap();
    let s = format!("{}/output", dir.display());

    let (sender, receiver) = unbounded_channel();

    let a = KaleidoArgs::random(tarascope::shader::OutputArgs { output_dir: s });
    
    // status task
    tokio::spawn(async move {
        let mut receiver = receiver;
        loop {
            if let Some(msg) = receiver.recv().await {
                println!("{:?}", msg);
            } else {
                break;
            }
        }
    });

    let task = run_kaleidoscope(&a, sender).await.unwrap();
    println!("{:?}", task.exit_status);

    Ok(())
}
