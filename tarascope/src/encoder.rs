use std::{io, process::Command};

use crate::{RenderJobDirectories, shader::KaleidoArgs};

pub fn stitch_video(dirs: &RenderJobDirectories) -> io::Result<()> {
    stitch_video_mp4(dirs)?;
    stitch_video_gif(dirs)?;
    Ok(())
}

pub fn stitch_video_gif(dirs: &RenderJobDirectories) -> io::Result<()> {
    println!("Stitching Video");
    let status = Command::new("ffmpeg")
        .args([
            "-framerate",
            "60",
            "-i",
            format!("{}/frame_%05d.png", dirs.project_folder_path()).as_str(),
            format!("{}/video.gif", dirs.project_folder_path()).as_str(),
        ])
        .status()?;

    if status.success() {
        println!("gif stitch sucessful");
    } else {
        eprintln!("error while stitching gif");
    }

    Ok(())
}

pub fn stitch_video_mp4(dirs: &RenderJobDirectories) -> io::Result<()> {
    println!("Stitching Video");
    let status = Command::new("ffmpeg")
        .args([
            "-framerate",
            "60",
            "-i",
            format!("{}/frame_%05d.png", dirs.project_folder_path()).as_str(),
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            format!("{}/video.mp4", dirs.project_folder_path()).as_str(),
        ])
        .status()?;

    if status.success() {
        println!("video stitch sucessful");
    } else {
        eprintln!("error while stitching video");
    }

    Ok(())
}
