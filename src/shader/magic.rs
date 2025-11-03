use clap_derive::Parser;
use rand::random_range;
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Parser, Debug, Clone, Serialize)]
pub struct MagicArgs {
    depth: u8,
    scale: f32,
    dist: f32,
}

impl MagicArgs {
    pub fn random() -> Self {
        Self { depth: random_range(0..=10), scale: random_range(0.0..=5.0), dist: random_range(0.0..=5.0) }
    }

    pub fn json(&self) -> Value {
        json!({
            "magic_depth": self.depth,
            "magic_scale": self.scale,
            "magic_distortion": self.dist
        })
    }
}
