use std::ops::RangeInclusive;

use clap_derive::Parser;
use rand::random_range;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::shader::{ParseError, parse_f64, validate_range};

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
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

fn scale_range() -> RangeInclusive<f32> {
    0.0..=20.0
}
fn frequency_range() -> RangeInclusive<f32> {
    0.0..=20.0
}
fn anisotropy_range() -> RangeInclusive<f32> {
    0.0..=1.0
}
fn orientation_range() -> RangeInclusive<f32> {
    0.0..=360.0
}

impl GaborArgs {
    pub fn random() -> Self {
        Self {
            scale: random_range(scale_range()),
            frequency: random_range(frequency_range()),
            anisotropy: random_range(anisotropy_range()),
            orientation: random_range(orientation_range()),
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
        let scale = validate_range(parse_f64(v, "gabor_scale")? as f32, scale_range())?;
        let anisotropy = validate_range(parse_f64(v, "gabor_anisotropy")? as f32, anisotropy_range())?;
        let orientation = validate_range(parse_f64(v, "gabor_orientation")? as f32, orientation_range())?;
        let frequency = validate_range(parse_f64(v, "gabor_frequency")? as f32, frequency_range())?;

        Ok(Self {
            scale,
            frequency,
            anisotropy,
            orientation,
        })
    }
}
