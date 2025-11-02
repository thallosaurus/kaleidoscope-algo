use std::{env::{self, var}, process::Command};

use clap::Parser;
use base64::{Engine, prelude::BASE64_STANDARD};

use crate::cli::CliArgs;

mod cli;

fn main() {
    let args = CliArgs::parse();
    println!("{}", args.get_json());

    let encoded = BASE64_STANDARD.encode(args.get_json().to_string());
    println!("{}", encoded);

    let blender_exec_path = var("BLENDER").unwrap_or(String::from("blender"));

    let cmd = Command::new(blender_exec_path)
        .arg("-b")
        .arg("kaleido.blend")
        .arg("-o")
        .arg("//frame_####")
        .arg("-Y")
        .arg("-P")
        .arg("loader.py")
        .arg("-f")
        .arg("0")
        .arg("--")
        .arg(encoded)
        .spawn()
        .expect("should be executable");
    cmd.wait_with_output().unwrap();
    // blender -b kaleido.blend -o "//output/frame_####" -Y -P loader.py -f 0 -- 'aaa aaaaaaaa'
}
