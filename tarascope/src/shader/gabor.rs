use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

use crate::shader::{ParseError, parse_f64};

#[derive(Parser, Debug, Clone, Serialize)]
pub struct GaborArgs {
    #[arg(long)]
    scale: f32,

    #[arg(long)]
    frequency: f32,

    #[arg(long)]
    anisotropy: f32,

    #[arg(long)]
    orientation: f32,
}

impl GaborArgs {
    pub fn random() -> Self {
        Self {
            scale: random_range(0.0..=20.0),
            frequency: random_range(0.0..=20.0),
            anisotropy: random_range(0.0..=1.0),
            orientation: random_range(0.0..=360.0),
        }
    }

    pub fn json(&self) -> Value {
        json!({
            "gabor_scale": self.scale,
            "gabor_frequency": self.frequency,
            "gabor_anisotropy": self.anisotropy,
            "gabor_orientation": self.orientation
        })
    }

    pub fn from_json(v: &Value) -> Result<Self, ParseError> {
        let scale = parse_f64(v, "gabor_scale".to_string())? as f32;
        let anisotropy = parse_f64(v, "gabor_anisotropy".to_string())? as f32;
        let orientation = parse_f64(v, "gabor_orientation".to_string())? as f32;
        let frequency = parse_f64(v, "gabor_frequency".to_string())? as f32;

        Ok(Self { scale, frequency, anisotropy, orientation })
    }
}
