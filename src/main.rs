use self::data_processor::DataProcessor;
use self::renderer::Renderer;

mod consumer;
mod data_processor;
mod renderer;
mod tests;
mod util;
mod world;

fn init_render_system<DP: DataProcessor, RR: Renderer>(data_processor: &mut DP, renderer: &mut RR) {
}

fn main() {}
