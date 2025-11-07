use std::ops::RangeInclusive;

use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

use crate::shader::{ParseError, parse_f64, validate_range};

#[derive(Parser, Debug, Clone, Serialize)]
pub struct UnoiseArgs {
    scale: f32,
    detail: f32,
    roughness: f32,
    lacunarity: f32,
    distortion: f32,
}

fn scale_range() -> RangeInclusive<f32> {
    1.0..=15.0
}
fn detail_range() -> RangeInclusive<f32> {
    0.0..=5.0
}
fn roughness_range() -> RangeInclusive<f32> {
    0.0..=1.0
}
fn lacunarity_range() -> RangeInclusive<f32> {
    0.0..=10.0
}
fn distortion_range() -> RangeInclusive<f32> {
    0.0..=10.0
}

impl UnoiseArgs {
    #[deprecated]
    pub fn random() -> Self {
        Self {
            scale: random_range(scale_range()),
            detail: random_range(detail_range()),
            roughness: random_range(roughness_range()),
            lacunarity: random_range(lacunarity_range()),
            distortion: random_range(distortion_range()),
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
        let scale = validate_range(parse_f64(v, "unoise_scale")? as f32, scale_range())?;
        let detail = validate_range(parse_f64(v, "unoise_detail")? as f32, detail_range())?;
        let roughness =
            validate_range(parse_f64(v, "unoise_roughness")? as f32, roughness_range())?;
        let lacunarity = validate_range(
            parse_f64(v, "unoise_lacunarity")? as f32,
            lacunarity_range(),
        )?;
        let distortion = validate_range(
            parse_f64(v, "unoise_distortion")? as f32,
            lacunarity_range(),
        )?;

        Ok(Self {
            scale,
            detail,
            roughness,
            lacunarity,
            distortion,
        })
    }
}
