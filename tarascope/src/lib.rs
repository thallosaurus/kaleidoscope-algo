use std::{
    fs::{File, create_dir},
    io::{self, BufWriter, Write},
    path::Path,
    process::ExitStatus,
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use tokio::{
    process::Command,
    sync::{Mutex, mpsc::UnboundedSender},
};

use crate::{exec::run, shader::KaleidoArgs};
pub mod encoder;
mod exec;
pub mod shader;

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

pub struct RenderJobDirectories {
    _directory: String,
    id: String,
}

impl RenderJobDirectories {
    pub fn new(id: String, dir: String) -> Self {
        Self {
            id,
            _directory: dir,
        }
    }
    pub fn output_dir(&self) -> String {
        /*let cwd = env::current_dir().expect("cannot access current working directory");
        let p = cwd.as_path().to_str().unwrap();*/
        self._directory.clone()
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn project_folder_path(&self) -> String {
        format!("{}/{}", self.output_dir(), self.get_id())
    }

    pub fn blender_stdout_path(&self) -> String {
        format!("{}/blender.stdout.log", self.project_folder_path())
    }

    pub fn blender_stderr_path(&self) -> String {
        format!("{}/blender.stderr.log", self.project_folder_path())
    }

    pub fn parameters_path(&self) -> String {
        format!("{}/parameters.json", self.project_folder_path())
    }

    pub fn blender_project_path(&self) -> String {
        format!("{}/project.blend", self.project_folder_path())
    }

    pub fn blender_frame_path(&self) -> String {
        format!("{}/frame_#####", self.project_folder_path())
    }

    pub fn blender_native_log_path(&self) -> String {
        format!("{}/blender.log", self.project_folder_path())
    }
}

fn extract_static_file(buffer: &[u8]) -> io::Result<Arc<Mutex<NamedTempFile>>> {
    let blend_tmp = Arc::new(Mutex::new(NamedTempFile::new()?));
    {
        let mut b = blend_tmp.try_lock().unwrap();
        let mut writer = BufWriter::new(b.as_file_mut());
        writer.write(buffer)?;
    }
    Ok(blend_tmp)
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

/// MARK: - New Stuff

/// Represents the type of command the renderer can run
pub enum CommandType {
    /// Case that indicated the user wants an animated output(start_frame, end_frame)
    Animated(usize, usize, KaleidoArgs),

    /// Case that indicates the user wants only a still image. (frame)
    Still(usize, KaleidoArgs),
}

impl CommandType {
    pub fn get_job_id(&self) -> String {
        match self {
            CommandType::Animated(_, _, kaleido_args) => kaleido_args.get_id(),
            CommandType::Still(_, kaleido_args) => kaleido_args.get_id(),
        }
    }
    fn command(&self, project: &Path, loader: &Path, dirs: &RenderJobDirectories) -> Command {
        match self {
            CommandType::Animated(frame_start, frame_end, args) => {
                let mut cmd = Command::new(BLENDER_PATH);
                cmd
                    //.arg("kaleido.blend")
                    .arg(project.as_os_str())
                    .arg("--factory-startup")
                    .arg("--log-file")
                    .arg(dirs.blender_native_log_path())
                    .arg("-s")
                    .arg(frame_start.to_string())
                    .arg("-e")
                    .arg(frame_end.to_string())
                    .arg("-o")
                    .arg(dirs.blender_frame_path())
                    .arg("-Y")
                    .arg("-P")
                    .arg(loader.as_os_str())
                    //.arg("loader.py")
                    .arg("-f")
                    .arg("0")
                    .arg("-b")
                    .arg("-a")
                    .arg("--");
                //.arg(encoded);
                cmd
            }
            CommandType::Still(frame, args) => todo!(),
        }
    }
    fn project_args(&self) -> KaleidoArgs {
        match self {
            CommandType::Animated(_, _, kaleido_args) => kaleido_args.clone(),
            CommandType::Still(_, kaleido_args) => kaleido_args.clone(),
        }
    }
}

/// The main struct that controls the generation
pub struct Tarascope {
    /// path to the projects root (where all projects live)
    directory: String,
}

impl Tarascope {
    pub fn new(directory: String) -> Self {
        Self { directory }
    }
    pub fn paths_for_job(&self, job_id: &String) -> RenderJobDirectories {
        RenderJobDirectories {
            _directory: self.directory.clone(),
            id: job_id.clone(),
        }
    }
    pub async fn start_render(
        &self,
        c: CommandType,
        sender: UnboundedSender<String>,
    ) -> io::Result<KaleidoOutput> {
        let args = c.project_args();
        let id = args.get_id();
        let dirs = self.paths_for_job(&id);

        // Encode the parameters to base64
        let encoded = args.base64();

        // create the target project
        create_dir(dirs.project_folder_path()).expect("couldn't create project dir");
        // extract Project File to a temporary location which gets dropped after the job is done
        let project_file = extract_static_file(BLEND_FILE)?;
        let project_borrow = project_file.try_lock().unwrap();
        let tmp_project_path = project_borrow.path();

        // same with the loader file
        let loader_file = extract_static_file(PYTHON_LOADER)?;
        let loader_borrow = loader_file.try_lock().unwrap();
        let tmp_loader_path = loader_borrow.path();

        // write the parameters before the render begins
        let json = serde_json::to_string(&args.json()).unwrap();
        let mut file = File::create(dirs.parameters_path())?;
        file.write_all(json.as_bytes())?;

        let mut cmd = c.command(tmp_project_path, tmp_loader_path, &dirs);

        // Append projectdata
        cmd
            .arg(encoded);
        // wait until the render has finished
        //let output = child.wait_with_output()?;

        let status = run(&mut cmd, &dirs, sender).await?;
        Ok(KaleidoOutput::new(status, dirs.output_dir()))
    }
}
