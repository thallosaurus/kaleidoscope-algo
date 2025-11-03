use std::{
    env::var,
    io::Error,
    process::{Child, Command},
};

use base64::{Engine, prelude::BASE64_STANDARD};

use crate::shader::KaleidoArgs;

pub mod shader;

pub fn run_kaleidoscope(args: KaleidoArgs) -> Child {
    let encoded = BASE64_STANDARD.encode(args.get_json().to_string());
    println!("{}", encoded);

    let blender_exec_path = var("BLENDER").unwrap_or(String::from("blender"));

    match Command::new(blender_exec_path)
        .arg("-b")
        .arg("--log-level")
        .arg("-1")
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
        .spawn() {
            Ok(child) => child,
            Err(err) => panic!("{}", err),
        }
}
