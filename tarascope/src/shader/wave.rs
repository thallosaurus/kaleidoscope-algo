use std::ops::RangeInclusive;

use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

use crate::shader::{ParseError, parse_f64};

#[derive(Parser, Debug, Clone, Serialize)]
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

fn scale_range() -> RangeInclusive<f32> {
    0.2..=5.0
}
fn distortion_range() -> RangeInclusive<f32> {
    -10.0..=10.0
}
fn detail_range() -> RangeInclusive<f32> {
    0.0..=5.0
}
fn detail_roughness_range() -> RangeInclusive<f32> {
    0.0..=1.0
}
fn phase_offset_range() -> RangeInclusive<f32> {
    0.0..=50.0
}

impl WaveArgs {
    pub fn random() -> Self {
        Self {
            scale: random_range(scale_range()),
            distortion: random_range(distortion_range()),
            detail: random_range(detail_range()),
            detail_roughness: random_range(detail_roughness_range()),
            phase_offset: random_range(phase_offset_range()),
        }
    }
    pub fn json(&self) -> Value {
        json!({
            "wave_scale": self.scale,
            "wave_distortion": self.distortion,
            "wave_detail": self.detail,
            "wave_detail_roughness": self.detail_roughness,
            "wave_phase_offset": self.phase_offset
        })
    }

    pub fn from_json(v: &Value) -> Result<Self, ParseError> {
        let detail = parse_f64(v, "wave_detail")? as f32;
        let scale = parse_f64(v, "wave_scale")? as f32;
        let distortion = parse_f64(v, "wave_distortion")? as f32;
        let detail_roughness = parse_f64(v, "wave_detail_roughness")? as f32;
        let phase_offset = parse_f64(v, "wave_phase_offset")? as f32;

        Ok(Self {
            scale,
            distortion,
            detail,
            detail_roughness,
            phase_offset,
        })
    }
}
