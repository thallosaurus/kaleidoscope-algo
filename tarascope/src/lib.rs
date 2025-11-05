use std::{
    cell::RefCell, fs::{File, create_dir}, io::{self, BufRead, BufReader, BufWriter, Write, pipe}, os::fd::AsRawFd, process::{Command, ExitStatus}, rc::Rc, thread
};

use base64::{Engine, prelude::BASE64_STANDARD};
use tempfile::NamedTempFile;

use crate::shader::KaleidoArgs;
pub mod shader;
pub mod encoder;

static BLEND_FILE: &[u8] = include_bytes!("../kaleido.blend");
static PYTHON_LOADER: &[u8] = include_bytes!("../loader.py");

#[cfg(target_os = "macos")]
static BLENDER_PATH: &str = "/Applications/Blender.app/Contents/MacOS/Blender";

#[cfg(target_os = "linux")]
static BLENDER_PATH: &str = "blender";

#[cfg(target_os = "windows")]
static BLENDER_PATH: &str = "C:\\Program Files\\Blender Foundation\\Blender\\blender.exe";

fn extract_static_file(buffer: &[u8]) -> io::Result<Rc<RefCell<NamedTempFile>>> {
    let blend_tmp = Rc::new(RefCell::new(NamedTempFile::new()?));
    {
        let mut b = blend_tmp.borrow_mut();
        let mut writer = BufWriter::new(b.as_file_mut());
        writer.write(buffer)?;
    }
    Ok(blend_tmp)
}

pub fn run_kaleidoscope(args: &KaleidoArgs) -> io::Result<KaleidoOutput> {
    // Encode the parameters to base64
    let encoded = BASE64_STANDARD.encode(args.json().to_string());

    // extract Project File to a temporary location which gets dropped after the job is done
    let project_file = extract_static_file(BLEND_FILE)?;
    let project_borrow = project_file.borrow_mut();
    let project_path = project_borrow.path();

    // same with the loader file
    let loader_file = extract_static_file(PYTHON_LOADER)?;
    let loader_borrow = loader_file.borrow_mut();
    let loader_path = loader_borrow.path();

    // create the target project
    create_dir(format!("{}/{}",args.get_output_dir(),args.get_id())).expect("couldn't create project dir");

    // write the parameters before the render begins
    let json = serde_json::to_string(&args.json()).unwrap();
    let mut file = File::create(format!("{}/{}/parameters.json", args.get_output_dir(), args.get_id()))?;
    file.write_all(json.as_bytes())?;

    let (reader, writer) = pipe().unwrap();
    let writer_fd = writer.as_raw_fd();

    let mut child = match Command::new(BLENDER_PATH)
        //.arg("kaleido.blend")
        .arg(project_path.as_os_str())
        .arg("--factory-startup")
        .arg("--log-file")
        .arg(format!(
            "{}/{}/blender.log",
            args.get_output_dir(),
            args.get_id()
        ))
        .arg("-s")
        .arg(args.get_start_frame().to_string())
        .arg("-e")
        .arg(args.get_end_frame().to_string())
        .arg("-o")
        .arg(format!(
            "{}/{}/frame_#####",
            args.get_output_dir(),
            args.get_id()
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
        .arg(format!("{}", writer_fd))
        .arg(encoded)
        .spawn()
    {
        Ok(child) => child,
        Err(err) => panic!("error while spawning subprocess: {}", err),
    };

    // wait until the render has finished
    //let output = child.wait_with_output()?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let output_dir = format!("{}/{}", args.get_output_dir(), args.get_id());
    
    let stdout_reader = thread::spawn(move || {
        let mut log = File::create(format!("{}/blender.stdout.log", output_dir)).expect("failed to create output log");
        for line in BufReader::new(stdout).lines() {
            let l = line.unwrap();
            println!("{}", l);
            log.write_all(l.as_bytes()).expect("error writing to output log");
        }
    });
    
    let output_dir = format!("{}/{}", args.get_output_dir(), args.get_id());
    let stderr_reader = thread::spawn(move || {
        let mut log_err = File::create(format!("{}/blender.stderr.log", output_dir)).expect("failed to create output error log");
        for line in BufReader::new(stderr).lines() {
            let l = line.unwrap();
            eprintln!("[stderr] {}", l);
            log_err.write_all(l.as_bytes()).expect("error writing to output log");
        }
    });

    stdout_reader.join().unwrap();
    stderr_reader.join().unwrap();

    let output = child.wait()?;

    // TODO write stdout
    /*let mut log = File::create(format!("{}/{}/blender.stdout.log", args.get_output_dir(), args.get_id()))?;
    log.write_all(&output.stdout)?;
    log.flush()?;

    // TODO write stderr
    let mut log_err = File::create(format!("{}/{}/blender.stderr.log", args.get_output_dir(), args.get_id()))?;
    log_err.write_all(&output.stderr)?;
    log_err.flush()?;*/

    //Ok(output.status)
    Ok(KaleidoOutput::new(output, args.get_output_dir()))
}

pub struct KaleidoOutput {
    pub exit_status: ExitStatus,
    _output_directory: String
}

impl KaleidoOutput {
    pub fn new(status: ExitStatus, directory: String) -> Self {
        KaleidoOutput {
            _output_directory: directory,
            exit_status: status
        }
    }
}