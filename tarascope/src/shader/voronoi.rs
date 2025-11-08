use std::ops::RangeInclusive;

use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

use crate::shader::{ParseError, parse_f64, validate_range};

fn scale_range() -> RangeInclusive<f32> {
    2.0..=20.0
}
fn detail_range() -> RangeInclusive<f32> {
    0.0..=3.0
}
fn randomize_range() -> RangeInclusive<f32> {
    0.0..=1.0
}

#[derive(Parser, Debug, Clone, Serialize)]
pub struct VoronoiArgs {
    #[arg(long)]
    scale: f32,

    #[arg(long)]
    detail: f32,

    #[arg(long)]
    randomize: f32,
}

impl VoronoiArgs {
    pub fn random() -> Self {
        Self {
            scale: random_range(scale_range()),
            detail: random_range(detail_range()),
            randomize: random_range(randomize_range()),
        }
    }
    pub fn json(&self) -> Value {
        json!({
            "voronoi_scale": self.scale,
            "voronoi_detail": self.detail,
            "voronoi_randomize": self.randomize
        })
    }

    pub fn from_json(v: &Value) -> Result<Self, ParseError> {
        let scale = validate_range(parse_f64(v, "voronoi_scale")? as f32, scale_range())?;
        let detail = validate_range(parse_f64(v, "voronoi_detail")? as f32, detail_range())?;
        let randomize = validate_range(parse_f64(v, "voronoi_randomize")? as f32, randomize_range())?;

        Ok(Self {
            scale,
            detail,
            randomize,
        })
    }
}
