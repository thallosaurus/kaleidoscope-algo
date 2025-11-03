use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Parser, Debug, Clone, Serialize)]
pub struct WaveArgs {
    #[arg(long)]
    scale: f32,
    
    #[arg(long)]
    distortion: f32,
    
    #[arg(long)]
    detail: f32,
    
    #[arg(long)]
    detail_roughness: f32,
    
    #[arg(long)]
    phase_offset: f32,
}

impl WaveArgs {
    pub fn random() -> Self {
        Self {scale: random_range(0.2..=5.0),
            distortion: random_range(-10.0..=10.0),
            detail: random_range(0.0..=5.0),
            detail_roughness: random_range(0.0..=1.0),
            phase_offset: random_range(0.0..=50.0)
        }
    }
    pub fn json(&self) -> Value {
        json!({
            "wave_scale": self.scale,
            "wave_distortion": self.distortion,
            "wave_detail": self.detail,
            "wave_detail_roughness": self.detail_roughness,
            "wave_phase_offset": self.phase_offset
        })
    }
}
