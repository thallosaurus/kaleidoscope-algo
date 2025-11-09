use std::io::Error;

use clap::{Parser, command};
use clap_derive::{Parser, Subcommand};
use tarascope::{CommandType, Tarascope, shader::KaleidoArgs};
use serde::Serialize;
use tokio::sync::mpsc::unbounded_channel;

/// Program to generate Kaleidoscopes using blender as a backend
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[structopt(subcommand)]
    mode: CliModes,


    #[arg(short, long)]
    output_dir: String,
}

#[derive(Debug, Subcommand, Clone, Serialize)]
enum CliModes {
    /// Randomized Kaleidoscope
    Random,

    /// Create a parameterized kaleidoscope
    Custom(KaleidoArgs),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = CliArgs::parse();
    let (sender, receiver) = unbounded_channel();

    let tarascopes = Tarascope::new(String::from(args.output_dir));

    let kargs = match args.mode {
        CliModes::Random => KaleidoArgs::random(),
        CliModes::Custom(kaleido_args) => kaleido_args,
    };

    let c = CommandType::Animated(1, 10, kargs);

    let output = tarascopes.start_render(c, sender).await?;

    //let cmd = run_kaleidoscope(output_args.output_dir, &kargs, sender).await?;
    println!("{}", output.exit_status);

    //stitch_video(&kargs).unwrap();
    Ok(())
}
