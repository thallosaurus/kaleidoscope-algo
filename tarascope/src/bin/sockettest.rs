use std::{cell::RefCell, env::current_dir, iter::Rev, process::Stdio, rc::Rc, task::Context};

use command_fds::{CommandFdExt, FdMapping};
use crossbeam::channel::unbounded;
use tarascope::{run_kaleidoscope, shader::KaleidoArgs};
use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    net::unix::pipe::{self, Receiver},
    process::Command,
    sync::{Mutex, mpsc::unbounded_channel},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let dir = current_dir().unwrap();
    let s = format!("{}/output", dir.display());

    let (sender, mut receiver) = unbounded_channel();

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
