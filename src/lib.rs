use std::{
    cell::RefCell,
    env::{self, var},
    io::{self, BufWriter, Write},
    process::{Child, Command, ExitStatus},
    rc::Rc,
    sync::Arc,
};

use base64::{Engine, prelude::BASE64_STANDARD};
use tempfile::NamedTempFile;

use crate::shader::KaleidoArgs;
pub mod shader;

static BLEND_FILE: &[u8] = include_bytes!("../kaleido.blend");

fn extract_blend_file() -> io::Result<Rc<RefCell<NamedTempFile>>> {
    let tmp = Rc::new(RefCell::new(NamedTempFile::new()?));
    {
        let mut b = tmp.borrow_mut();
        let mut writer = BufWriter::new(b.as_file_mut());
        writer.write(BLEND_FILE)?;
    }
    Ok(tmp)
}

pub fn run_kaleidoscope(args: &KaleidoArgs) -> io::Result<ExitStatus> {
    let encoded = BASE64_STANDARD.encode(args.json().to_string());
    let blender_exec_path = var("BLENDER").unwrap_or(String::from("blender"));

    let project_file = extract_blend_file()?;
    let borrow = project_file.borrow_mut();
    let path = borrow.path();

    let child = match Command::new(blender_exec_path)
        //.arg("kaleido.blend")
        .arg(path.as_os_str())
        .arg("--factory-startup")
        .arg("--log-level")
        .arg("-1")
        .arg("--log-file")
        .arg(format!(
            "{}/{}",
            args.get_output_dir(),
            args.get_id() + ".json"
        ))
        .arg("-s")
        .arg(args.get_start_frame().to_string())
        .arg("-e")
        .arg(args.get_end_frame().to_string())
        .arg("-o")
        .arg(format!(
            "{}/{}",
            args.get_output_dir(),
            args.get_id() + "_#####"
        ))
        .arg("-Y")
        .arg("-P")
        .arg("loader.py")
        .arg("-f")
        .arg("0")
        .arg("-b")
        .arg("-a")
        .arg("--")
        .arg(encoded)
        .spawn()
    {
        Ok(child) => child,
        Err(err) => panic!("{}", err),
    };

    let output = child.wait_with_output()?;
    Ok(output.status)
}

pub fn stitch_video(kargs: &KaleidoArgs) -> std::io::Result<()> {
    let status = Command::new("ffmpeg")
        .args([
            "-framerate",
            "60",
            "-i",
            format!("{}/{}_%05d.png", kargs.get_output_dir(), kargs.get_id()).as_str(),
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            format!("{}/{}.mp4", kargs.get_output_dir(), kargs.get_id()).as_str(),
        ])
        .status()?;

    if status.success() {
        println!("video stitch sucessful");
    } else {
        eprintln!("error while stitching video");
    }

    Ok(())
}
