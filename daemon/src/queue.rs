use std::error::Error;

use log::debug;
use tarascope::{CommandType, RenderStatus, shader::KaleidoArgs};
use tokio::{sync::mpsc::{Receiver, Sender, UnboundedReceiver, UnboundedSender, channel, error::TrySendError, unbounded_channel}, task::JoinHandle};

use crate::{SharedDatabasePool, SharedTarascope, database::{get_specific_job_parameters, insert_frame, register_new_kaleidoscope, set_kaleidoscope_to_done, set_kaleidoscope_to_waiting}};

pub enum RenderQueueRequest {
    RandomAnimated,
    ParameterizedAnimated(String),
}

pub enum RenderQueueError {
    QueuePushError
}

pub struct RenderQueue {
    pool: SharedDatabasePool,
    queue_sender: UnboundedSender<RenderQueueRequest>,
    _handle: JoinHandle<()>,
}

static MAX_QUEUE_ITEMS: usize = 1;

impl RenderQueue {
    pub fn new(pool: SharedDatabasePool, executor: SharedTarascope) -> Self {
        // for the start allocate a size 2 render
        let (queue_sender, mut rx) = unbounded_channel::<RenderQueueRequest>();

        Self {
            pool: pool.clone(),
            queue_sender,
            _handle: Self::task(pool, rx, executor),
        }
    }

    fn task(
        pool: SharedDatabasePool,
        mut rx: UnboundedReceiver<RenderQueueRequest>,
        executor: SharedTarascope,
    ) -> JoinHandle<()> {
        // render queue task
        tokio::spawn(async move {
            //let args = r_args.clone();
            loop {
                if let Some(req) = rx.recv().await {
                    //let lock = pool.lock().await;
                    //let args = args.lock().await;
                    match req {
                        RenderQueueRequest::RandomAnimated => {
                            println!("Starting new random job");
                            let job = KaleidoArgs::random();

                            let j = job.json();
                            let id = job.get_id();

                            let lock = pool.lock().await;
                            register_new_kaleidoscope(&lock, &id, j.to_string())
                                .await
                                .unwrap();
                            drop(lock);

                            //render_tasks(&pool, &job).await.unwrap();
                            Self::render(
                                pool.clone(),
                                CommandType::Animated(1, 300, job),
                                executor.clone(),
                            )
                            .await
                            .unwrap();
                            //tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                            println!("Finished Render Job");
                        }
                        RenderQueueRequest::ParameterizedAnimated(id) => {
                            let lock = pool.lock().await;
                            let job = get_specific_job_parameters(&lock, &id).await.unwrap();
                            drop(lock);
                            println!("Starting new parameterized job {}", id);
                            //render_tasks(&pool, &job).await.unwrap();
                            Self::render(
                                pool.clone(),
                                CommandType::Animated(1, 300, job),
                                executor.clone(),
                            )
                            .await
                            .unwrap();
                            println!("Finished Render Job");
                        }
                    }
                } else {
                    println!("queue closed");
                    break;
                }
            }
        })
    }

    //let pool = init_database().await.unwrap();
    async fn render(
        pool: SharedDatabasePool,
        job: CommandType,
        executor: SharedTarascope,
    ) -> Result<(), Box<dyn Error>> {
        let id = job.get_job_id();
        let (sender, receiver) = unbounded_channel::<String>();

        // status collector task
        let pool_task = pool.clone();
        tokio::spawn(async move {
            let mut receiver = receiver;
            loop {
                if let Some(msg) = receiver.recv().await {
                    debug!("{:?}", msg);

                    //let p = self.pool.clone();
                    let pool = pool_task.lock().await;

                    let data: RenderStatus = serde_json::from_str(msg.as_str()).unwrap();
                    insert_frame(&pool, data).await.unwrap();
                    continue;
                } else {
                    break;
                }
            }
        });

        let pool = pool.clone();

        let lock = pool.lock().await;
        if let Err(e) = set_kaleidoscope_to_waiting(&lock, &id).await {
            eprintln!("{}", e);
        }
        drop(lock);

        let t_lock = executor.lock().await;
        let output = t_lock.start_render(job, sender).await?;

        if output.exit_status.success() {
            //stitch_video(&kaleidoargs).unwrap();
            let lock = pool.lock().await;
            set_kaleidoscope_to_done(&lock, &id).await?;
            drop(lock);
        }

        Ok(())
    }

    pub fn push(&self, request: RenderQueueRequest) -> Result<(), RenderQueueError> {
        //debug!("queue capacity: {}", self.queue_sender.capacity());
        if let Err(e) = self.queue_sender.send(request) {
            eprintln!("error while adding task to render queue: {}", e);
            Err(RenderQueueError::QueuePushError)
            //Err(e)
        } else {
            Ok(())
        }
    }
}