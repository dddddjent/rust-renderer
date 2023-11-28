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

    fn step(&mut self, world: &crate::world::world::World) -> Vec<Vec<simple_math::Vector3u8>> {
        let camera = match &world.camera {
            Some(camera) => camera,
            None => panic!("No camera in the world!"),
        };
        debug!("color: {}", self.color);
        let mut result = vec![vec![self.color; camera.size.1]; camera.size.0];
        for i in 30..80 {
            for j in 60..160 {
                result[i][j] = Vector3u8::new([0u8, 0u8, 0u8]);
            }
        }
        result
    }
}
