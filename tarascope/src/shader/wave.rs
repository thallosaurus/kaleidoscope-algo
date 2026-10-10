use std::ops::RangeInclusive;

use clap_derive::Parser;
use rand::random_range;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::shader::{ParseError, parse_f64, validate_range};

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
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

/// Defines the allowed range for the scale parameter.
fn scale_range() -> RangeInclusive<f32> {
    0.2..=5.0
}
/// Defines the allowed range for the distortion parameter.
fn distortion_range() -> RangeInclusive<f32> {
    -10.0..=10.0
}
/// Defines the allowed range for the detail parameter.
fn detail_range() -> RangeInclusive<f32> {
    0.0..=5.0
}
/// Defines the allowed range for detail roughness.
fn detail_roughness_range() -> RangeInclusive<f32> {
    0.0..=1.0
}
/// Defines the allowed range for phase offset.
fn phase_offset_range() -> RangeInclusive<f32> {
    0.0..=50.0
}

impl WaveArgs {
    /// Creates a random configuration within the value ranges defined for this type.
    pub fn random() -> Self {
        Self {
            scale: random_range(scale_range()),
            distortion: random_range(distortion_range()),
            detail: random_range(detail_range()),
            detail_roughness: random_range(detail_roughness_range()),
            phase_offset: random_range(phase_offset_range()),
        }
    }
    /// Serializes the configuration to the project JSON format.
    pub fn json(&self) -> Value {
        json!({
            "wave_scale": self.scale,
            "wave_distortion": self.distortion,
            "wave_detail": self.detail,
            "wave_detail_roughness": self.detail_roughness,
            "wave_phase_offset": self.phase_offset
        })
    }

    /// Reads the configuration from JSON and validates its types and allowed value ranges.
    pub fn from_json(v: &Value) -> Result<Self, ParseError> {
        let detail = validate_range(parse_f64(v, "wave_detail")? as f32, detail_range())?;
        let scale = validate_range(parse_f64(v, "wave_scale")? as f32, scale_range())?;
        let distortion = validate_range(parse_f64(v, "wave_distortion")? as f32, distortion_range())?;
        let detail_roughness =
            validate_range(parse_f64(v, "wave_detail_roughness")? as f32, detail_roughness_range())?;
        let phase_offset =
            validate_range(parse_f64(v, "wave_phase_offset")? as f32, phase_offset_range())?;

        Ok(Self {
            scale,
            distortion,
            detail,
            detail_roughness,
            phase_offset,
        })
    }
}
