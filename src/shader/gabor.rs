use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Parser, Debug, Clone, Serialize)]
pub struct GaborArgs {
    scale: f32,
    frequency: f32,
    anisotropy: f32,
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
}
