use super::Renderer;
use crate::world::serde::deserialize_color;
use log::debug;
use serde::{Deserialize, Serialize};
use simple_math::Vector3u8;

#[derive(Debug, Deserialize, Serialize)]
pub struct NaiveRenderer {
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Vector3u8,
}

impl NaiveRenderer {
    pub fn new() -> Self {
        NaiveRenderer {
            color: Vector3u8::new([255u8, 255u8, 255u8]),
        }
    }
}

impl Renderer for NaiveRenderer {
    fn set_args(&mut self, args: &serde_json::Value) {
        *self = serde_json::from_value(args.clone()).unwrap();
    }

    fn step(
        &mut self,
        _world: &crate::world::world::World,
        output_buffer: &mut Vec<Vec<simple_math::Vector3u8>>,
    ) {
        debug!("color: {}", self.color);
        for col in output_buffer.iter_mut() {
            for pixel in col.iter_mut() {
                *pixel = self.color.clone();
            }
        }
        for i in 30..80 {
            for j in 60..160 {
                output_buffer[i][j] = Vector3u8::new([0u8, 0u8, 0u8]);
            }
        }
    }
}
