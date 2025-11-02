use clap::{Parser, command};
use clap_derive::{Parser, Subcommand, ValueEnum};
use clap_num::number_range;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Texture to base the kaleidoscope on
    //#[arg(short, long)]
    #[structopt(subcommand)]
    texture: TextureSelector,

    #[clap(flatten)]
    polar: PolarArgs
}

#[derive(Debug, Parser)]
struct PolarArgs {
    /// Specifies how many Repetitions the kaleidoscope has (3 - 12)
    repetition: u8,
    scaling: f32,
    rotation: f32,
    pingpong: f32
}

#[derive(Debug, Subcommand)]
enum TextureSelector {
    /// Gabor Texture
    //#[clap(name = "sender")]
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
    Textured(TexturedArgs)
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
    
}

#[derive(Parser, Debug)]
struct MagicArgs {
    
}

#[derive(Parser, Debug)]
struct NoiseArgs {
    
}

#[derive(Parser, Debug)]
struct UnoiseArgs {
    
}

#[derive(Parser, Debug)]
struct TexturedArgs {
    
}