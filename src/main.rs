use self::data_processor::DataProcessor;
use self::renderer::Renderer;

mod camera;
mod consumer;
mod data_processor;
mod renderer;
mod utils;
mod world;

fn init_render_system<DP: DataProcessor, RR: Renderer>(data_processor: &mut DP, renderer: &mut RR) {
}

fn main() {}
