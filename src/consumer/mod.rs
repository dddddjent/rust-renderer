use crate::renderer::Renderer;
use crate::world::world::World;
// use crate::renderer::Renderer;

pub mod image;

pub trait Consumer {
    fn set_args(&mut self, config_path: &str, args: &serde_json::Value);
    fn set_world(&mut self, world: World);
    fn set_renderer(&mut self, renderer: Box<dyn Renderer>);
    fn run(&mut self);
}
