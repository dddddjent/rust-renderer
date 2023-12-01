pub mod basic_path_tracer;
pub mod naive_renderer;

pub trait Renderer {
    fn set_args(&mut self, args: &serde_json::Value);
    fn step(
        &mut self,
        world: &crate::world::world::World,
        output_buffer: &mut Vec<Vec<simple_math::Vector3u8>>,
    );
}
