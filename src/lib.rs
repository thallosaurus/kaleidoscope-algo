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
static PYTHON_LOADER: &[u8] = include_bytes!("../loader.py");
#[cfg(target_os = "macos")]
static BLENDER_PATH: &str = "/Applications/Blender.app/Contents/MacOS/Blender";

#[cfg(target_os = "linux")]
static BLENDER_PATH: &str = "blender";

fn extract_static_file(buffer: &[u8]) -> io::Result<Rc<RefCell<NamedTempFile>>> {
    let blend_tmp = Rc::new(RefCell::new(NamedTempFile::new()?));
    {
        let mut b = blend_tmp.borrow_mut();
        let mut writer = BufWriter::new(b.as_file_mut());
        writer.write(buffer)?;
    }
    Ok(blend_tmp)
}

pub fn run_kaleidoscope(args: &KaleidoArgs) -> io::Result<ExitStatus> {
    let encoded = BASE64_STANDARD.encode(args.json().to_string());
    //let blender_exec_path = ").unwrap_or(String::from("blender"));

    let project_file = extract_static_file(BLEND_FILE)?;
    let project_borrow = project_file.borrow_mut();
    let project_path = project_borrow.path();

    let loader_file = extract_static_file(PYTHON_LOADER)?;
    let loader_borrow = loader_file.borrow_mut();
    let loader_path = loader_borrow.path();

    let child = match Command::new(BLENDER_PATH)
        //.arg("kaleido.blend")
        .arg(project_path.as_os_str())
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
        .arg(loader_path.as_os_str())
        //.arg("loader.py")
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
    println!("Stitching Video");
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
