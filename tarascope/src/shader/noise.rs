use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Parser, Debug, Clone, Serialize)]
pub struct NoiseArgs {
    #[arg(long)]
    scale: f32,
    
    #[arg(long)]
    detail: f32,
    
    #[arg(long)]
    roughness: f32,
    
    #[arg(long)]
    lacunarity: f32,
    
    #[arg(long)]
    distortion: f32,
}

impl NoiseArgs {
    pub fn random() -> Self {
        Self {
            scale: random_range(1.0..=15.0),
            detail: random_range(0.0..=5.0),
            roughness: random_range(0.0..=1.0),
            lacunarity: random_range(0.0..=10.0),
            distortion: random_range(0.0..=10.0),
        }
    }

    pub fn json(&self) -> Value {
        json!({
            "noise_scale": self.scale,
            "noise_detail": self.detail,
            "noise_roughness": self.roughness,
            "noise_lacunarity": self.lacunarity,
            "noise_distortion": self.distortion
        })
    }
}
