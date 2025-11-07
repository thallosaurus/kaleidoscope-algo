use std::ops::RangeInclusive;

use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

use crate::shader::ParseError;

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
            randomize: random_range(randomize_range())
        }
    }
    pub fn json(&self) -> Value {
        json!({
            "voronoi_scale": self.scale,
            "voronoi_detail": self.detail,
            "voronoi_randomize": self.randomize
        })
    }

    pub fn from_json(json: &Value) -> Result<Self, ParseError> {
        let scale = json["voronoi_scale"].as_f64().expect("voronoi_scale was not a number") as f32;
        let detail = json["voronoi_detail"].as_f64().expect("voronoi_detail was not a number") as f32;
        let randomize = json["voronoi_randomize"].as_f64().expect("voronoi_randomize was not a number") as f32;

        Ok(Self { scale, detail, randomize })
    }
}
