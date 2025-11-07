use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

use crate::shader::{ParseError, parse_f64};

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
    
    pub fn from_json(v: &Value) -> Result<Self, ParseError> {
        let scale = parse_f64(v, "noise_scale".to_string())? as f32;
        let detail = parse_f64(v, "noise_detail".to_string())? as f32;
        let roughness = parse_f64(v, "noise_roughness".to_string())? as f32;
        let lacunarity = parse_f64(v, "noise_lacunarity".to_string())? as f32;
        let distortion = parse_f64(v, "noise_distortion".to_string())? as f32;
        Ok(Self {
            scale,
            detail,
            roughness,
            lacunarity,
            distortion,
        })
    }
}
