use std::ops::RangeInclusive;

use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

use crate::shader::{ParseError, parse_f64, parse_u64};

#[derive(Parser, Debug, Clone, Serialize)]
pub struct MagicArgs {
    #[arg(long)]
    depth: u8,

    #[arg(long)]
    scale: f32,

    #[arg(long)]
    dist: f32,
}

fn depth_range() -> RangeInclusive<u8> {
    0..=10
}

fn scale_range() -> RangeInclusive<f32> {
    0.0..=5.0
}

fn distortion_range() -> RangeInclusive<f32> {
    0.0..=5.0
}

impl MagicArgs {
    pub fn random() -> Self {
        Self {
            depth: random_range(depth_range()),
            scale: random_range(scale_range()),
            dist: random_range(distortion_range()),
        }
    }

    pub fn json(&self) -> Value {
        json!({
            "magic_depth": self.depth,
            "magic_scale": self.scale,
            "magic_distortion": self.dist
        })
    }

    pub fn from_json(v: &Value) -> Result<Self, ParseError> {
        let depth = parse_u64(v, "magic_depth")? as u8;
        let scale = parse_f64(v, "magic_scale")? as f32;
        let dist = parse_f64(v, "magic_distortion")? as f32;

        Ok(Self { depth, scale, dist })
    }
}
