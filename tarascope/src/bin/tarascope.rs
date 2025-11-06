use std::io::Error;

use clap::{Parser, command};
use clap_derive::{Parser, Subcommand};
use tarascope::{encoder::stitch_video, run_kaleidoscope, shader::{KaleidoArgs, OutputArgs}};
use serde::Serialize;
use tokio::sync::mpsc::unbounded_channel;

/// Program to generate Kaleidoscopes using blender as a backend
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[structopt(subcommand)]
    mode: CliModes,


    //#[arg(short, long)]
    //output_dir: String,
}

#[derive(Debug, Subcommand, Clone, Serialize)]
enum CliModes {
    /// Randomized Kaleidoscope
    Random(OutputArgs),

    /// Create a parameterized kaleidoscope
    Custom(KaleidoArgs),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = CliArgs::parse();
    let (sender, receiver) = unbounded_channel();

    let kargs = match args.mode {
        CliModes::Random(output_args) => KaleidoArgs::random(output_args),
        CliModes::Custom(kaleido_args) => kaleido_args,
    };

    let cmd = run_kaleidoscope(&kargs, sender).await?;
    println!("{}", cmd.exit_status);

    stitch_video(&kargs).unwrap();
    Ok(())
}
