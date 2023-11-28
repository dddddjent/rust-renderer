use simple_math::Vector3u8;

use crate::world::world::World;

pub mod basic_path_tracer;
pub mod naive_renderer;

pub trait Renderer {
    fn set_args(&mut self, args: &serde_json::Value);
    fn step(&mut self, world: &World) -> Vec<Vec<Vector3u8>>;
}
