use std::{
    process::{ExitStatus, Stdio},
    sync::Arc,
};

use command_fds::{CommandFdExt, FdMapping};
use tokio::{
    fs::File,
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::unix::pipe::{self},
    process::Command,
    sync::{Mutex, mpsc::UnboundedSender},
};

use crate::shader::KaleidoArgs;

pub async fn run(
    cmd: &mut Command,
    kargs: &KaleidoArgs,
    sender: UnboundedSender<String>,
) -> io::Result<ExitStatus> {
    let (writer, reader) = pipe::pipe()?;

    let sender = Arc::new(Mutex::new(sender));

    //attach fd mappings
    cmd.fd_mappings(vec![FdMapping {
        parent_fd: writer.into_blocking_fd()?,
        child_fd: 7,
    }])
    .unwrap()
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

    let mut ccmd = cmd.spawn()?;

    let stdout = ccmd.stdout.take().unwrap();
    let stdout_path = kargs.blender_stdout_path();
    let stdout_task = tokio::spawn(async move {
        let mut log = File::create(stdout_path)
            .await
            .expect("failed to create output log");

        let mut stdout_bufreader = BufReader::new(stdout);
        loop {
            let mut stdout_buf = String::new();
            let read = stdout_bufreader.read_line(&mut stdout_buf).await;

            match read {
                // EOF
                Ok(0) => break,
                Ok(s) => {
                    println!("{}", stdout_buf.trim());
                    log.write_all(stdout_buf.as_bytes()).await.unwrap();
                    continue;
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    });

    let stderr = ccmd.stderr.take().unwrap();
    let stderr_path = kargs.blender_stderr_path();
    let stderr_task = tokio::spawn(async move {
        let mut log = File::create(stderr_path)
            .await
            .expect("failed to create output log");
        let mut stderr_bufreader = BufReader::new(stderr);

        loop {
            let mut stderr_buf = String::new();
            let read = stderr_bufreader.read_line(&mut stderr_buf).await;

            match read {
                // EOF
                Ok(0) => break,
                Ok(s) => {
                    println!("{}", stderr_buf.trim());
                    log.write_all(stderr_buf.as_bytes()).await.unwrap();
                    continue;
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    });

    let child = Arc::new(Mutex::new(ccmd));

    let mut status_reader = BufReader::new(reader);
    loop {
        let c = child.clone();
        let mut c = c.lock().await;

        let mut status_buf = String::new();

        tokio::select! {
            read = status_reader.read_line(&mut status_buf) => {

                match read {
                    // EOF
                    Ok(0) => {
                        continue
                    },
                    Ok(_) => {
                        //println!("status: {}", status_buf.trim());
                        let msg = String::from(status_buf.trim());
                        let s = sender.lock().await;
                        s.send(msg).unwrap();
                    },
                    Err(e) => eprintln!("{}", e)
                }
            }
            status = c.wait() => {
                println!("Child exited: {:?}", status);

                //stdout_task.await.unwrap();
                let _ = tokio::join!(stdout_task, stderr_task);
                return status
            }
        }
    }
}
