use clap::{Parser, command};
use clap_derive::{Parser, Subcommand, ValueEnum};
use clap_num::number_range;
use rand::{random, random_range};
use serde_json::{Value, json};

/// Program to generate Kaleidoscopes using blender as a backend
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Texture to base the kaleidoscope on
    #[structopt(subcommand)]
    texture: TextureSelector,

    #[clap(flatten)]
    polar: PolarArgs
}

impl CliArgs {
    pub fn get_json(&self) -> Value {
        json!({
            "index": self.texture.get_index(),
            "repetition": self.polar.repetition,
            "scaling": self.polar.scaling,
            "rotation": self.polar.rotation,
            "pingpong": self.polar.pingpong,
            "texture": self.texture.get_json()
        })
        //"repetition": self.tex
    }
}

#[derive(Debug, Parser)]
struct PolarArgs {
    /// Specifies how many Repetitions the kaleidoscope has (3 - 12)
    repetition: u8,

    /// Specifies the scaling
    scaling: f32,

    /// Specifies the rotation
    rotation: f32,

    /// Specifies the Ping-Pong
    pingpong: f32
}

#[derive(Debug, Subcommand)]
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

    fn get_json(&self) -> Value {
        match self {
            TextureSelector::Gabor(gabor_args) => json!({
                "gabor_scale": gabor_args.scale,
                "gabor_frequency": gabor_args.frequency,
                "gabor_anisotropy": gabor_args.anisotropy,
                "gabor_orientation": gabor_args.orientation
            }),
            TextureSelector::Voronoi(voronoi_args) => json!({
                "voronoi_scale": voronoi_args.scale,
                "voronoi_detail": voronoi_args.detail,
                "voronoi_randomize": voronoi_args.randomize
            }),
            TextureSelector::Wave(wave_args) => json!({
                "wave_scale": wave_args.scale,
                "wave_distortion": wave_args.distortion,
                "wave_detail": wave_args.detail,
                "wave_detail_roughness": wave_args.detail_roughness,
                "wave_phase_offset": wave_args.phase_offset
            }),
            TextureSelector::Magic(magic_args) => json!({
                "magic_depth": magic_args.depth,
                "magic_scale": magic_args.scale,
                "magic_distortion": magic_args.dist
            }),
            TextureSelector::Noise(noise_args) => json!({
                "noise_scale": noise_args.scale,
                "noise_detail": noise_args.detail,
                "noise_roughness": noise_args.roughness,
                "noise_lacunary": noise_args.lacunary,
                "noise_distortion": noise_args.distortion
            }),
            TextureSelector::Unoise(unoise_args) => todo!(),
            TextureSelector::Textured(textured_args) => todo!(),
        }
    }
}

#[derive(Parser, Debug)]
struct GaborArgs {
    scale: f32,
    frequency: f32,
    anisotropy: f32,
    orientation: f32
}

#[derive(Parser, Debug)]
struct VoronoiArgs {
    scale: f32,
    detail: f32,
    randomize: f32
}

#[derive(Parser, Debug)]
struct WaveArgs {
    scale: f32,
    distortion: f32,
    detail: f32,
    detail_roughness: f32,
    phase_offset: f32
}

#[derive(Parser, Debug)]
struct MagicArgs {
    depth: u8,
    scale: f32,
    dist: f32
}

#[derive(Parser, Debug)]
struct NoiseArgs {
    scale: f32,
    detail: f32,
    roughness: f32,
    lacunary: f32,
    distortion: f32
}

#[derive(Parser, Debug)]
struct UnoiseArgs {
    
}

#[derive(Parser, Debug)]
struct TexturedArgs {
    
}