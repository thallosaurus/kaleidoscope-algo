use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

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
            scale: random_range(2.0..=20.0),
            detail: random_range(0.0..=3.0), 
            randomize: random_range(0.0..=1.0)
        }
    }
    pub fn json(&self) -> Value {
        json!({
            "voronoi_scale": self.scale,
            "voronoi_detail": self.detail,
            "voronoi_randomize": self.randomize
        })
    }
}
