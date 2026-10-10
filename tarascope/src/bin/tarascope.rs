use std::{io::Error, sync::Arc};

use clap::Parser as _;
use clap_derive::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tarascope::{CommandType, Tarascope, encoder::stitch_video, exec::ExecutorUpdate, shader::KaleidoArgs};
use tokio::{fs, sync::{Mutex, mpsc::unbounded_channel}};

/// Program to generate Kaleidoscopes using blender as a backend
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[structopt(subcommand)]
    mode: CliModes,

    #[arg(short, long, default_value = "output")]
    output_dir: String,
}

#[derive(Debug, Subcommand, Clone, Serialize)]
enum CliModes {
    /// Randomized Kaleidoscope
    Random,

    /// Create a parameterized kaleidoscope
    Custom(KaleidoArgs),

    /// Loads parameters from JSON file
    Preset(FromFileArguments),
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
struct FromFileArguments {
    path: String,
}

/// Reads the render mode and output directory, starts rendering, and stitches the frames into videos.

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = CliArgs::parse();
    let (sender, mut receiver) = unbounded_channel();

    let tarascopes = Tarascope::new(String::from(args.output_dir));

    let kargs = match args.mode {
        CliModes::Random => KaleidoArgs::random(),
        CliModes::Custom(kaleido_args) => kaleido_args,
        CliModes::Preset(path) => {
            let contents = fs::read_to_string(path.path)
                .await
                .expect("cant read preset file");
            let v: Value = serde_json::from_str(&contents).expect("invalid data");
            KaleidoArgs::from(v)
        }
    };

    let id = kargs.get_id();

    let c = CommandType::Animated(
        kargs.frames.frame_start.into(),
        kargs.frames.frame_end.into(),
        kargs,
    );

    //let output = tarascopes.start_render(c, sender).await?;

    let render = tarascopes.start_render(c, sender);
    tokio::pin!(render);

    loop {
        tokio::select! {
            Ok(output) = &mut render => {
                println!("{}", output.exit_status);
                stitch_video(&tarascopes.paths_for_job(&id)).unwrap();
                break
            }

            Some(msg) = receiver.recv() => {
                let m: ExecutorUpdate = serde_json::from_str(&msg).unwrap();
                println!("{:?}", m)
            }
        }
    }

    Ok(())
}
