use base64::{Engine, prelude::BASE64_STANDARD};
use clap_derive::{Parser, Subcommand};
use core::panic;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    fmt::Display,
    ops::{RangeBounds, RangeInclusive},
};
use uuid::Uuid;

use crate::shader::{
    gabor::GaborArgs, magic::MagicArgs, noise::NoiseArgs, unoise::UnoiseArgs, voronoi::VoronoiArgs,
    wave::WaveArgs,
};

mod gabor;
mod magic;
mod noise;
mod unoise;
mod voronoi;
mod wave;

#[derive(Debug)]
pub enum ParseError {
    WrongType(String),
    WrongTextureIndex(u8),
    OutOfRangeError,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::WrongType(key) => {
                write!(f, "{} was not a number", key)
            }
            ParseError::WrongTextureIndex(index) => write!(f, "unknown index texture {}", index),
            ParseError::OutOfRangeError => write!(f, "value was out of range"),
        }
    }
}

#[derive(Debug, Parser, Clone, Serialize)]
pub struct KaleidoArgs {
    /// Texture to base the kaleidoscope on
    #[structopt(subcommand)]
    texture: TextureSelector,

    #[clap(flatten)]
    polar: PolarArgs,

    #[clap(flatten)]
    composite: CompositeArgs,

    #[clap(flatten)]
    frames: FrameArgs,

    #[clap(skip = Uuid::new_v4().to_string())]
    id: String,

    #[clap(flatten)]
    output: OutputArgs,

    #[arg(short, long)]
    animate: bool,
}

impl KaleidoArgs {
    pub fn random(output_dir: String) -> Self {
        println!("[DEBUG] {}", output_dir);
        Self {
            texture: TextureSelector::random(),
            polar: PolarArgs::random(),
            composite: CompositeArgs::random(),
            frames: FrameArgs::default(),
            id: Uuid::new_v4().to_string(),
            output: OutputArgs { output_dir },
            animate: false,
        }
    }

    pub fn json(&self) -> Value {
        json!({
            "id": self.get_id(),
            "output_directory": self.output_dir(),
            "texture_index": self.texture.get_index(),
            "repetition": self.polar.repetition,
            "scaling": self.polar.scaling,
            "rotation": self.polar.rotation,
            "pingpong": self.polar.pingpong,
            "texture": self.texture.json(),
            "composite": self.composite.json(),
            "frames": self.frames.json()
        })
    }

    pub fn from_json(v: Value) -> Result<Self, ParseError> {
        println!("{:?}", v.as_object());
        let repetition = parse_u64(&v, "repetition")? as u8;
        let scaling = parse_f64(&v, "scaling")? as f32;
        let rotation = parse_f64(&v, "rotation")? as f32;
        let pingpong = parse_f64(&v, "pingpong")? as f32;
        let id = parse_string(&v, "id")?;

        Ok(Self {
            id,
            texture: TextureSelector::from_json(&v)?,
            polar: PolarArgs {
                repetition,
                scaling,
                rotation,
                pingpong,
            },
            composite: CompositeArgs::from_json(&v["composite"])?,
            frames: FrameArgs::from_json(&v["frames"])?,
            output: OutputArgs {
                output_dir: String::new(),
            },
            animate: true,
        })
    }

    pub fn base64(&self) -> String {
        BASE64_STANDARD.encode(self.json().to_string())
    }

    pub fn get_start_frame(&self) -> u16 {
        self.frames.frame_start
    }
    pub fn get_end_frame(&self) -> u16 {
        self.frames.frame_end
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn output_dir(&self) -> String {
        /*let cwd = env::current_dir().expect("cannot access current working directory");
        let p = cwd.as_path().to_str().unwrap();*/
        self.output.output_dir.clone()
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

fn repetition_range() -> RangeInclusive<u8> {
    3..=12
}

fn scaling_range() -> RangeInclusive<f32> {
    2.5..=12.0
}
fn rotation_range() -> RangeInclusive<f32> {
    0.0..=360.0
}
fn pingpong_range() -> RangeInclusive<f32> {
    0.5..=4.5
}

#[derive(Debug, Parser, Clone, Serialize)]
struct PolarArgs {
    /// Specifies how many Repetitions the kaleidoscope has (3 - 12)
    #[arg(long)]
    repetition: u8,

    /// Specifies the scaling
    #[arg(long)]
    scaling: f32,

    /// Specifies the rotation offset
    #[arg(long)]
    rotation: f32,

    /// Specifies the Ping-Pong
    #[arg(long)]
    pingpong: f32,
}

impl PolarArgs {
    pub fn random() -> Self {
        Self {
            repetition: random_range(repetition_range()),
            scaling: random_range(scaling_range()),
            //rotation: random_range(0.0..=360.0),
            rotation: 0.0,
            pingpong: random_range(pingpong_range()),
        }
    }
}

#[derive(Debug, Subcommand, Clone, Serialize)]
enum TextureSelector {
    /// Gabor Texture
    Gabor(GaborArgs),

    /// Voronoi Texture
    Voronoi(VoronoiArgs),

    /// Wave Texture
    Wave(WaveArgs),

    /// Magic Texture
    Magic(MagicArgs),

    /// Noise Texture
    Noise(NoiseArgs),

    /// Unvectored Noise
    Unoise(UnoiseArgs),

    /// Image Texture
    Textured(TexturedArgs),
}

impl TextureSelector {
    pub fn random() -> Self {
        // 5 = without uNoise
        // 6 = with uNoise
        // 7 = with Textured
        let r = random_range(0..=4);
        //Self::from(r)
        match r {
            0 => TextureSelector::Gabor(GaborArgs::random()),
            1 => TextureSelector::Voronoi(VoronoiArgs::random()),
            2 => TextureSelector::Wave(WaveArgs::random()),
            3 => TextureSelector::Magic(MagicArgs::random()),
            4 => TextureSelector::Noise(NoiseArgs::random()),
            5 => TextureSelector::Unoise(UnoiseArgs::random()),
            6 => TextureSelector::Textured(TexturedArgs::random()),
            _ => panic!("invalid texture index"),
        }
    }

    fn get_index(&self) -> u8 {
        match self {
            TextureSelector::Gabor(_) => 0,
            TextureSelector::Voronoi(_) => 1,
            TextureSelector::Wave(_) => 2,
            TextureSelector::Magic(_) => 3,
            TextureSelector::Noise(_) => 4,
            TextureSelector::Unoise(_) => 5,
            TextureSelector::Textured(_) => 6,
        }
    }

    fn json(&self) -> Value {
        match self {
            TextureSelector::Gabor(gabor_args) => gabor_args.json(),
            TextureSelector::Voronoi(voronoi_args) => voronoi_args.json(),
            TextureSelector::Wave(wave_args) => wave_args.json(),
            TextureSelector::Magic(magic_args) => magic_args.json(),
            TextureSelector::Noise(noise_args) => noise_args.json(),
            TextureSelector::Unoise(unoise_args) => unoise_args.json(),
            TextureSelector::Textured(textured_args) => textured_args.json(),
        }
    }

    fn from_json(v: &Value) -> Result<Self, ParseError> {
        let index = parse_u64(v, "texture_index")? as u8;
        let texture = v["texture"].clone();

        match index {
            0 => {
                let args = GaborArgs::from_json(&texture)?;
                Ok(TextureSelector::Gabor(args))
            }
            1 => {
                let args = VoronoiArgs::from_json(&texture)?;
                Ok(TextureSelector::Voronoi(args))
            }
            2 => {
                let args = WaveArgs::from_json(&texture)?;
                Ok(TextureSelector::Wave(args))
            }
            3 => {
                let args = MagicArgs::from_json(&texture)?;
                Ok(TextureSelector::Magic(args))
            }
            4 => {
                let args = NoiseArgs::from_json(&texture)?;
                Ok(TextureSelector::Noise(args))
            }
            5 => {
                let args = UnoiseArgs::from_json(&texture)?;
                Ok(TextureSelector::Unoise(args))
            }
            6 => {
                let args = TexturedArgs::from_json(&texture)?;
                Ok(TextureSelector::Textured(args))
            }
            _ => Err(ParseError::WrongTextureIndex(index)),
        }
    }
}

/*impl From<u8> for TextureSelector {
    fn from(value: u8) -> Self {
        match value {
            0 => TextureSelector::Gabor(GaborArgs::random()),
            1 => TextureSelector::Voronoi(VoronoiArgs::random()),
            2 => TextureSelector::Wave(WaveArgs::random()),
            3 => TextureSelector::Magic(MagicArgs::random()),
            4 => TextureSelector::Noise(NoiseArgs::random()),
            5 => TextureSelector::Unoise(UnoiseArgs::random()),
            6 => TextureSelector::Textured(TexturedArgs::random()),
            _ => panic!("invalid texture index"),
        }
    }
}*/

#[derive(Debug, Parser, Clone, Serialize)]
struct CompositeArgs {
    #[arg(long)]
    lens_distortion: f32,

    #[arg(long)]
    lens_dispersion: f32,

    #[arg(long)]
    hue: f32,

    #[arg(long)]
    saturation: f32,
}

fn lens_distortion_range() -> RangeInclusive<f32> {
    -1.0..=-0.5
}
fn lens_dispersion_range() -> RangeInclusive<f32> {
    -1.0..=-0.5
}
fn hue_range() -> RangeInclusive<f32> {
    0.0..=1.0
}
fn saturation_range() -> RangeInclusive<f32> {
    1.0..=2.0
}

impl CompositeArgs {
    fn random() -> Self {
        Self {
            lens_distortion: random_range(lens_distortion_range()),
            lens_dispersion: random_range(lens_dispersion_range()),
            hue: random_range(hue_range()),
            saturation: random_range(saturation_range()),
        }
    }
    fn json(&self) -> Value {
        json!({
            "composite_lens_distortion": -0.1, //self.lens_distortion,
            "composite_lens_dispersion": -0.3, //self.lens_dispersion
            "composite_hue": self.hue,
            "composite_saturation": self.saturation
        })
    }

    fn from_json(json: &Value) -> Result<Self, ParseError> {
        let hue = validate_range(parse_f64(json, "composite_hue")? as f32, hue_range())?;
        let lens_dispersion = validate_range(
            parse_f64(json, "composite_lens_dispersion")? as f32,
            lens_dispersion_range(),
        )?;
        let lens_distortion = validate_range(
            parse_f64(json, "composite_lens_distortion")? as f32,
            lens_distortion_range(),
        )?;
        let saturation = validate_range(
            parse_f64(json, "composite_saturation")? as f32,
            saturation_range(),
        )?;
        Ok(Self {
            lens_distortion,
            lens_dispersion,
            hue,
            saturation,
        })
    }
}

pub fn validate_range<T>(value: T, range: std::ops::RangeInclusive<T>) -> Result<T, ParseError>
where
    T: PartialOrd + Copy + std::fmt::Debug,
{
    if range.contains(&value) {
        Ok(value)
    } else {
        Err(ParseError::OutOfRangeError)
    }
}

fn parse_u64(v: &Value, key: &'static str) -> Result<u64, ParseError> {
    let value = v[key].as_u64();
    println!("[DEBUG/u64]{}: {:?}", key, value);
    if let Some(value) = value {
        Ok(value)
    } else {
        Err(ParseError::WrongType(String::from(key)))
    }
}
fn parse_f64(v: &Value, key: &'static str) -> Result<f64, ParseError> {
    let value = v[key].as_f64();
    println!("[DEBUG/f64]{}: {:?}", key, value);
    if let Some(value) = value {
        Ok(value)
    } else {
        Err(ParseError::WrongType(String::from(key)))
    }
}
fn parse_string(v: &Value, key: &'static str) -> Result<String, ParseError> {
    let value = v[key].as_str();
    println!("[DEBUG/string]{}: {:?}", key, value);
    if let Some(value) = value {
        Ok(String::from(value))
    } else {
        Err(ParseError::WrongType(String::from(key)))
    }
}

#[derive(Parser, Debug, Clone, Serialize)]
struct TexturedArgs {
    file_path: String,
}

impl TexturedArgs {
    pub fn random() -> Self {
        Self {
            file_path: String::from("path goes here"),
        }
    }

    pub fn json(&self) -> Value {
        json!({
            "file_path": self.file_path
        })
    }

    pub fn from_json(json: &Value) -> Result<Self, ParseError> {
        let file_path = String::from(
            json["file_path"]
                .as_str()
                .expect("file_path was not a number"),
        );

        Ok(Self { file_path })
    }
}
#[derive(Parser, Debug, Clone, Serialize)]
struct FrameArgs {
    #[arg(long)]
    frame_start: u16,

    #[arg(long)]
    frame_end: u16,
}

impl FrameArgs {
    pub fn json(&self) -> Value {
        json!({
            "_frames_start": self.frame_start,
            "_frames_max": self.frame_end
        })
    }

    pub fn from_json(v: &Value) -> Result<Self, ParseError> {
        let frame_start = parse_u64(v, "_frames_start")? as u16;
        let frame_end = parse_u64(v, "_frames_max")? as u16;

        Ok(Self {
            frame_start,
            frame_end,
        })
    }
}

impl Default for FrameArgs {
    fn default() -> Self {
        Self {
            frame_start: 1,
            frame_end: 300,
        }
    }
}

#[derive(Debug, Parser, Clone, Serialize)]
pub struct OutputArgs {
    #[arg(short, long)]
    pub output_dir: String,
}
