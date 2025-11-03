use std::{
    env::var,
    process::{Child, Command},
};

use base64::{Engine, prelude::BASE64_STANDARD};
use uuid::Uuid;

use crate::shader::KaleidoArgs;

pub mod shader;

pub fn run_kaleidoscope(args: &KaleidoArgs) -> (Uuid, Child) {
    let encoded = BASE64_STANDARD.encode(args.json().to_string());
    //println!("{}", encoded);

    let blender_exec_path = var("BLENDER").unwrap_or(String::from("blender"));

    let output_filename = Uuid::new_v4();

    let child = match Command::new(blender_exec_path)
        .arg("-b")
        .arg("--log-level")
        .arg("-1")
        .arg("kaleido.blend")
        .arg("-o")
        .arg(format!("//output/{}_####", output_filename))
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
        };

        (output_filename, child)
}
