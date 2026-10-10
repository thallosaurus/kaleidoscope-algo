use std::ops::RangeInclusive;

use clap_derive::Parser;
use rand::random_range;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::shader::{ParseError, parse_f64, parse_u64, validate_range};

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
pub struct MagicArgs {
    #[arg(long)]
    depth: u8,

    #[arg(long)]
    scale: f32,

    #[arg(long)]
    dist: f32,
}

/// Defines the allowed range for recursion depth.
fn depth_range() -> RangeInclusive<u8> {
    0..=10
}

/// Defines the allowed range for the scale parameter.
fn scale_range() -> RangeInclusive<f32> {
    0.0..=5.0
}

/// Defines the allowed range for the distortion parameter.
fn distortion_range() -> RangeInclusive<f32> {
    0.0..=5.0
}

impl MagicArgs {
    /// Creates a random configuration within the value ranges defined for this type.
    pub fn random() -> Self {
        Self {
            depth: random_range(depth_range()),
            scale: random_range(scale_range()),
            dist: random_range(distortion_range()),
        }
    }

    /// Serializes the configuration to the project JSON format.
    pub fn json(&self) -> Value {
        json!({
            "magic_depth": self.depth,
            "magic_scale": self.scale,
            "magic_distortion": self.dist
        })
    }

    /// Reads the configuration from JSON and validates its types and allowed value ranges.
    pub fn from_json(v: &Value) -> Result<Self, ParseError> {
        let depth = validate_range(parse_u64(v, "magic_depth")? as u8, depth_range())?;
        let scale = validate_range(parse_f64(v, "magic_scale")? as f32, scale_range())?;
        let dist = validate_range(parse_f64(v, "magic_distortion")? as f32, distortion_range())?;

        Ok(Self { depth, scale, dist })
    }
}
