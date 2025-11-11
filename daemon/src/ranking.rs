use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use opencv::{core, imgproc, prelude::*};
use std::error::Error;

pub fn score_image(path: &str) -> Result<f32, Box<dyn Error>> {
    // Bild laden
    let img = ImageReader::open(path)?.decode()?.to_rgb8();
    let (width, height) = img.dimensions();

    // In Mat für OpenCV umwandeln
    let mat = Mat::from_slice(img.as_raw())?
        .reshape(3, height as i32)?
        .t()?;

    // Graustufen
    let mut gray = Mat::default();
    imgproc::cvt_color(&mat, &mut gray, imgproc::COLOR_RGB2GRAY, 0)?;

    // 1. Kontrast (Standardabweichung)
    let mut mean = core::Scalar::default();
    let mut stddev = core::Scalar::default();
    core::mean_std_dev(&gray, &mut mean, &mut stddev, &core::no_array()?)?;
    let contrast = stddev[0] as f32;

    // 2. Sättigung (aus HSV)
    let mut hsv = Mat::default();
    imgproc::cvt_color(&mat, &mut hsv, imgproc::COLOR_RGB2HSV, 0)?;
    let channels = core::split(&hsv)?;
    let saturation = core::mean(&channels[1], &core::no_array()?)?.0[0] as f32;

    // 3. Kantenenergie (Canny)
    let mut edges = Mat::default();
    imgproc::canny(&gray, &mut edges, 100.0, 200.0, 3, false)?;
    let edge_energy = core::sum_elems(&edges)?.0[0] as f32 / (width * height) as f32;

    // 4. Symmetrie
    let left = Mat::roi(&mat, core::Rect::new(0, 0, (width/2) as i32, height as i32))?;
    let right = Mat::roi(&mat, core::Rect::new((width/2) as i32, 0, (width/2) as i32, height as i32))?;
    let mut right_flipped = Mat::default();
    core::flip(&right, &mut right_flipped, 1)?;
    let diff = core::absdiff(&left, &right_flipped)?;
    let mean_diff = core::mean(&diff, &core::no_array()?)?.0[0] as f32 / 255.0;
    let symmetry = 1.0 - mean_diff;

    // Gewichtete Mischung
    Ok(0.4*contrast + 0.2*saturation + 0.3*edge_energy + 0.1*symmetry)
}