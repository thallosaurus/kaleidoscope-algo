use core::panic;
use clap_derive::{Parser, Subcommand};
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

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
}

impl KaleidoArgs {
    pub fn random() -> Self {
        Self {
            texture: TextureSelector::random(),
            polar: PolarArgs::random(),
            composite: CompositeArgs::random(),
        }
    }

    pub fn json(&self) -> Value {
        json!({
            "texture_index": self.texture.get_index(),
            "repetition": self.polar.repetition,
            "scaling": self.polar.scaling,
            "rotation": self.polar.rotation,
            "pingpong": self.polar.pingpong,
            "texture": self.texture.json(),
            "composite": self.composite.json()
        })
    }
}

#[derive(Debug, Parser, Clone, Serialize)]
struct PolarArgs {
    /// Specifies how many Repetitions the kaleidoscope has (3 - 12)
    repetition: u8,

    /// Specifies the scaling
    scaling: f32,

    /// Specifies the rotation
    rotation: f32,

    /// Specifies the Ping-Pong
    pingpong: f32,
}

impl PolarArgs {
    pub fn random() -> Self {
        Self {
            repetition: random_range(3..=12),
            scaling: random_range(2.5..=12.0),
            rotation: random_range(0.0..=360.0),
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
        let r = random_range(0..=5);
        Self::from(r)
    }

    fn get_index(&self) -> u8 {
        match self {
            TextureSelector::Gabor(_) => 1,
            TextureSelector::Voronoi(_) => 2,
            TextureSelector::Wave(_) => 3,
            TextureSelector::Magic(_) => 4,
            TextureSelector::Noise(_) => 5,
            TextureSelector::Unoise(_) => 6,
            TextureSelector::Textured(_) => 7,
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
    lens_distortion: f32,
    lens_dispersion: f32,
    hue: f32,
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
            "composite_lens_distortion": -0.1,//self.lens_distortion,
            "composite_lens_dispersion": -0.3, //self.lens_dispersion
            "composite_hue": self.hue,
            "composite_saturation": self.saturation
        })
    }
}

#[derive(Parser, Debug, Clone, Serialize)]
struct TexturedArgs {}

impl TexturedArgs {
    pub fn random() -> Self {
        Self {}
    }

    pub fn json(&self) -> Value {
        json!({})
    }
}
