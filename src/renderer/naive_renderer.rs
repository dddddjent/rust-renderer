use super::{OutputBuffer, Renderer};
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

    fn step(&mut self, _world: &crate::world::world::World, output_buffer: &mut OutputBuffer) {
        debug!("color: {}", self.color);
        output_buffer
            .iter_2d_mut()
            .for_each(|(_, _, val)| *val = self.color.clone());
        output_buffer.iter_2d_mut().for_each(|(x, y, val)| {
            if x >= 30 && x < 100 {
                if y >= 50 && y < 150 {
                    *val = Vector3u8::zeros();
                }
            }
        });
    }
}
