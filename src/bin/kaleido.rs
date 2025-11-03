use clap::Parser;
use clap_derive::{Parser, Subcommand};
use kaleidosynth::{run_kaleidoscope, shader::KaleidoArgs};
use serde::Serialize;

/// Program to generate Kaleidoscopes using blender as a backend
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[structopt(subcommand)]
    mode: CliModes,
}

#[derive(Debug, Subcommand, Clone, Serialize)]
enum CliModes {
    /// Randomized Kaleidoscope
    Random,

    /// Create a parameterized kaleidoscope
    Custom(KaleidoArgs)
}


fn main() {
    let args = CliArgs::parse();

    let (id, cmd) = match args.mode {
        CliModes::Random => run_kaleidoscope(KaleidoArgs::random()),
        CliModes::Custom(kaleido_args) => run_kaleidoscope(kaleido_args),
    };

    cmd.wait_with_output().unwrap();
}
