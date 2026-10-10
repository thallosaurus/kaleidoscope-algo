use std::ops::RangeInclusive;

use clap_derive::Parser;
use rand::random_range;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::shader::{ParseError, parse_f64, validate_range};

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
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

/// Defines the allowed range for the scale parameter.
fn scale_range() -> RangeInclusive<f32> {
    1.0..=15.0
}

/// Defines the allowed range for the detail parameter.
fn detail_range() -> RangeInclusive<f32> {
    0.0..=5.0
}

/// Defines the allowed range for the roughness parameter.
fn roughness_range() -> RangeInclusive<f32> {
    0.0..=1.0
}

/// Defines the allowed range for the lacunarity parameter.
fn lacunarity_range() -> RangeInclusive<f32> {
    0.0..=10.0
}

/// Defines the allowed range for the distortion parameter.
fn distortion_range() -> RangeInclusive<f32> {
    0.0..=10.0
}

impl NoiseArgs {
    /// Creates a random configuration within the value ranges defined for this type.
    pub fn random() -> Self {
        Self {
            scale: random_range(scale_range()),
            detail: random_range(detail_range()),
            roughness: random_range(roughness_range()),
            lacunarity: random_range(lacunarity_range()),
            distortion: random_range(distortion_range()),
        }
    }

    /// Serializes the configuration to the project JSON format.
    pub fn json(&self) -> Value {
        json!({
            "noise_scale": self.scale,
            "noise_detail": self.detail,
            "noise_roughness": self.roughness,
            "noise_lacunarity": self.lacunarity,
            "noise_distortion": self.distortion
        })
    }

    /// Reads the configuration from JSON and validates its types and allowed value ranges.
    pub fn from_json(v: &Value) -> Result<Self, ParseError> {
        let scale = validate_range(parse_f64(v, "noise_scale")? as f32, scale_range())?;
        let detail = validate_range(parse_f64(v, "noise_detail")? as f32, detail_range())?;
        let roughness = validate_range(parse_f64(v, "noise_roughness")? as f32, roughness_range())?;
        let lacunarity =
            validate_range(parse_f64(v, "noise_lacunarity")? as f32, lacunarity_range())?;
        let distortion =
            validate_range(parse_f64(v, "noise_distortion")? as f32, distortion_range())?;

        Ok(Self {
            scale,
            detail,
            roughness,
            lacunarity,
            distortion,
        })
    }
}
