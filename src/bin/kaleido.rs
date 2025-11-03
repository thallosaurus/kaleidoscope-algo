use std::{env::{self, var}, process::Command};

use clap::Parser;
use base64::{Engine, prelude::BASE64_STANDARD};
use clap_derive::{Parser, Subcommand};
use kaleidoscope_algo::{run_kaleidoscope, shader::KaleidoArgs};
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
    Random,
    Custom(KaleidoArgs)
}


fn main() {
    let args = CliArgs::parse();

    let cmd = match args.mode {
        CliModes::Random => run_kaleidoscope(KaleidoArgs::random()),
        CliModes::Custom(kaleido_args) => run_kaleidoscope(kaleido_args),
    };
    //println!("{}", args.get_json());

    cmd.wait_with_output().unwrap();

    // blender -b kaleido.blend -o "//output/frame_####" -Y -P loader.py -f 0 -- 'aaa aaaaaaaa'
}
