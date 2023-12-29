pub mod basic_path_tracer;
pub mod naive_renderer;

// TODO: Shouldn't be put here. Consider move it else where
pub struct OutputBuffer {
    pub output_buffer: Vec<Vec<simple_math::Vector3u8>>,
}

impl OutputBuffer {
    pub fn iter_2d(&self) -> impl Iterator<Item = (usize, usize, &simple_math::Vector3u8)> {
        self.output_buffer
            .iter()
            .enumerate()
            .flat_map(|(x, col)| col.iter().enumerate().map(move |(y, val)| (x, y, val)))
    }
    pub fn iter_2d_mut(
        &mut self,
    ) -> impl Iterator<Item = (usize, usize, &mut simple_math::Vector3u8)> {
        self.output_buffer
            .iter_mut()
            .enumerate()
            .flat_map(|(x, col)| col.iter_mut().enumerate().map(move |(y, val)| (x, y, val)))
    }
}

pub trait Renderer {
    fn set_args(&mut self, args: &serde_json::Value);
    fn step(&mut self, world: &crate::world::world::World, output_buffer: &mut OutputBuffer);
}
