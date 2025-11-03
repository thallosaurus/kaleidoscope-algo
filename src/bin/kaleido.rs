use std::{
    fs::File,
    io::{Error, Write},
};

use clap::Parser;
use clap_derive::{Parser, Subcommand};
use kaleidosynth::{run_kaleidoscope, shader::KaleidoArgs, stitch_video};
use serde::Serialize;

/// Program to generate Kaleidoscopes using blender as a backend
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[structopt(subcommand)]
    mode: CliModes,

    #[arg(short)]
    output_dir: Option<String>,
}

#[derive(Debug, Subcommand, Clone, Serialize)]
enum CliModes {
    /// Randomized Kaleidoscope
    Random,

    /// Create a parameterized kaleidoscope
    Custom(KaleidoArgs),
}

fn main() -> Result<(), Error> {
    let args = CliArgs::parse();

    let kargs = match args.mode {
        CliModes::Random => KaleidoArgs::random(args.output_dir),
        CliModes::Custom(kaleido_args) => kaleido_args,
    };

    let cmd = run_kaleidoscope(&kargs);

    let json = serde_json::to_string(&kargs.json()).unwrap();

    let mut file = File::create(format!("output/{}.json", kargs.get_id()))?;
    file.write_all(json.as_bytes())?;

    stitch_video(&kargs).unwrap();
    Ok(())
}
