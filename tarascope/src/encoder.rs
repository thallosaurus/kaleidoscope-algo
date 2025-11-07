use std::{io, process::Command};

use crate::shader::KaleidoArgs;

pub fn stitch_video(kargs: &KaleidoArgs) -> io::Result<()> {
    stitch_video_mp4(kargs)?;
    stitch_video_gif(kargs)?;
    Ok(())
}

pub fn stitch_video_gif(kargs: &KaleidoArgs) -> io::Result<()> {
    println!("Stitching Video");
    let status = Command::new("ffmpeg")
        .args([
            "-framerate",
            "60",
            "-i",
            format!("{}/frame_%05d.png", kargs.project_folder_path()).as_str(),
            format!("{}/video.gif", kargs.project_folder_path()).as_str(),
        ])
        .status()?;

    if status.success() {
        println!("gif stitch sucessful");
    } else {
        eprintln!("error while stitching gif");
    }

    Ok(())
}

pub fn stitch_video_mp4(kargs: &KaleidoArgs) -> io::Result<()> {
    println!("Stitching Video");
    let status = Command::new("ffmpeg")
        .args([
            "-framerate",
            "60",
            "-i",
            format!("{}/frame_%05d.png", kargs.project_folder_path()).as_str(),
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            format!("{}/video.mp4", kargs.project_folder_path()).as_str(),
        ])
        .status()?;

    if status.success() {
        println!("video stitch sucessful");
    } else {
        eprintln!("error while stitching video");
    }

    Ok(())
}
