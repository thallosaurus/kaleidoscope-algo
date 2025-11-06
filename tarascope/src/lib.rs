use std::{
    cell::RefCell, fs::{File, create_dir}, io::{self, BufRead, BufReader, BufWriter, Write, pipe}, os::fd::AsRawFd, process::ExitStatus, rc::Rc, thread
};

use command_fds::{CommandFdExt, FdMapping};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use tokio::{process::Command, sync::mpsc::{Sender, UnboundedSender, unbounded_channel}};

use crate::{exec::run, shader::KaleidoArgs};
pub mod encoder;
pub mod shader;
mod exec;

static BLEND_FILE: &[u8] = include_bytes!("../kaleido.blend");
static PYTHON_LOADER: &[u8] = include_bytes!("../loader.py");

#[cfg(target_os = "macos")]
static BLENDER_PATH: &str = "/Applications/Blender.app/Contents/MacOS/Blender";

#[cfg(target_os = "linux")]
static BLENDER_PATH: &str = "blender";

#[cfg(target_os = "windows")]
static BLENDER_PATH: &str = "C:\\Program Files\\Blender Foundation\\Blender\\blender.exe";

#[derive(Serialize, Deserialize, Debug)]
pub struct RenderStatus {
    pub id: String,
    pub frame: i32,
}

fn extract_static_file(buffer: &[u8]) -> io::Result<Rc<RefCell<NamedTempFile>>> {
    let blend_tmp = Rc::new(RefCell::new(NamedTempFile::new()?));
    {
        let mut b = blend_tmp.borrow_mut();
        let mut writer = BufWriter::new(b.as_file_mut());
        writer.write(buffer)?;
    }
    Ok(blend_tmp)
}

pub async fn run_kaleidoscope(args: &KaleidoArgs, sender: UnboundedSender<String>) -> io::Result<KaleidoOutput> {
    // Encode the parameters to base64
    let encoded = args.base64();

    // extract Project File to a temporary location which gets dropped after the job is done
    let project_file = extract_static_file(BLEND_FILE)?;
    let project_borrow = project_file.borrow_mut();
    let tmp_project_path = project_borrow.path();

    // same with the loader file
    let loader_file = extract_static_file(PYTHON_LOADER)?;
    let loader_borrow = loader_file.borrow_mut();
    let tmp_loader_path = loader_borrow.path();

    // create the target project
    create_dir(args.project_folder_path()).expect("couldn't create project dir");

    // write the parameters before the render begins
    let json = serde_json::to_string(&args.json()).unwrap();
    let mut file = File::create(args.parameters_path())?;
    file.write_all(json.as_bytes())?;

    let mut cmd = Command::new(BLENDER_PATH);
    cmd
        //.arg("kaleido.blend")
        .arg(tmp_project_path.as_os_str())
        .arg("--factory-startup")
        .arg("--log-file")
        .arg(args.blender_native_log_path())
        .arg("-s")
        .arg(args.get_start_frame().to_string())
        .arg("-e")
        .arg(args.get_end_frame().to_string())
        .arg("-o")
        .arg(args.blender_frame_path())
        .arg("-Y")
        .arg("-P")
        .arg(tmp_loader_path.as_os_str())
        //.arg("loader.py")
        .arg("-f")
        .arg("0")
        .arg("-b")
        .arg("-a")
        .arg("--")
        .arg(encoded);
    // wait until the render has finished
    //let output = child.wait_with_output()?;

    let status = run(&mut cmd, args, sender).await?;
    Ok(KaleidoOutput::new(status, args.output_dir()))
}

pub struct KaleidoOutput {
    pub exit_status: ExitStatus,
    _output_directory: String,
}

impl KaleidoOutput {
    pub fn new(status: ExitStatus, directory: String) -> Self {
        KaleidoOutput {
            _output_directory: directory,
            exit_status: status,
        }
    }
}
