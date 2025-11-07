use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

use crate::shader::{ParseError, parse_f64};

#[derive(Parser, Debug, Clone, Serialize)]
pub struct UnoiseArgs {
    scale: f32,
    detail: f32,
    roughness: f32,
    lacunarity: f32,
    distortion: f32,
}

impl UnoiseArgs {
    #[deprecated]
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
            "unoise_scale": self.scale,
            "unoise_detail": self.detail,
            "unoise_roughness": self.roughness,
            "unoise_lacunarity": self.lacunarity,
            "unoise_distortion": self.distortion
        })
    }

    pub fn from_json(v: &Value) -> Result<Self, ParseError> {
        let scale = parse_f64(v, "unoise_scale".to_string())? as f32;
        let detail = parse_f64(v, "unoise_detail".to_string())? as f32;
        let roughness = parse_f64(v, "unoise_roughness".to_string())? as f32;
        let lacunarity = parse_f64(v, "unoise_lacunarity".to_string())? as f32;
        let distortion = parse_f64(v, "unoise_distortion".to_string())? as f32;

        Ok(Self {
            scale,
            detail,
            roughness,
            lacunarity,
            distortion,
        })
    }
}
