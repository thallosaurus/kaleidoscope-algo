use core::panic;
use clap_derive::{Parser, Subcommand};
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::shader::{
    gabor::GaborArgs, magic::MagicArgs, noise::NoiseArgs, unoise::UnoiseArgs, voronoi::VoronoiArgs, wave::WaveArgs
};

mod gabor;
mod magic;
mod noise;
mod voronoi;
mod wave;
mod unoise;

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
    animate: bool
}

impl KaleidoArgs {
    pub fn random(output_dir: OutputArgs) -> Self {
        Self {
            texture: TextureSelector::random(),
            polar: PolarArgs::random(),
            composite: CompositeArgs::random(),
            frames: FrameArgs::default(),
            id: Uuid::new_v4().to_string(),
            output: output_dir,
            animate: false
        }
    }

    pub fn json(&self) -> Value {
        json!({
            "id": self.get_id(),
            "output_directory": self.get_output_dir(),
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

    pub fn get_start_frame(&self) -> u16 {
        self.frames.frame_start
    }
    pub fn get_end_frame(&self) -> u16 {
        self.frames.frame_end
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_output_dir(&self) -> String {
        /*let cwd = env::current_dir().expect("cannot access current working directory");
        let p = cwd.as_path().to_str().unwrap();*/
        self.output.output_dir.clone()
    }
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
            repetition: random_range(3..=12),
            scaling: random_range(2.5..=12.0),
            //rotation: random_range(0.0..=360.0),
            rotation: 0.0,
            pingpong: random_range(0.5..=4.5),
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
        Self::from(r)
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
}

impl From<u8> for TextureSelector {
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
}

#[derive(Debug, Parser, Clone, Serialize)]
struct CompositeArgs {
    #[arg(long)]
    lens_distortion: f32,

    #[arg(long)]
    lens_dispersion: f32,
    
    #[arg(long)]
    hue: f32,

    #[arg(long)]
    saturation: f32
}

impl CompositeArgs {
    fn random() -> Self {
        Self {
            lens_distortion: random_range(-1.0..=-0.5),
            lens_dispersion: random_range(-1.0..=-0.5),
            hue: random_range(0.0..=1.0),
            saturation: random_range(1.0..=2.0),
            
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
}

#[derive(Parser, Debug, Clone, Serialize)]
struct TexturedArgs {
    file_path: String
}

impl TexturedArgs {
    pub fn random() -> Self {
        Self {
            file_path: String::from("path goes here")
        }
    }

    pub fn json(&self) -> Value {
        json!({
            "file_path": self.file_path
        })
    }
}
#[derive(Parser, Debug, Clone, Serialize)]
struct FrameArgs {
    #[arg(long)]
    frame_start: u16,
    
    #[arg(long)]
    frame_end: u16
}

impl FrameArgs {
    pub fn json(&self) -> Value {
        json!({
            "_frames_start": self.frame_start,
            "_frames_max": self.frame_end
        })
    }
}

impl Default for FrameArgs {
    fn default() -> Self {
        Self { frame_start: 1, frame_end: 10 }
    }
}

#[derive(Debug, Parser, Clone, Serialize)]
pub struct OutputArgs {
    #[arg(short, long)]
    output_dir: String,
}