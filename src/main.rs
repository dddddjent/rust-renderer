use self::configuration::Configuration;
use self::data_processor::direct::DirectDataProcessor;
use self::data_processor::DataProcessor;

use std::path::Path;

mod configuration;
mod consumer;
mod data_processor;
mod renderer;
mod tests;
mod util;
mod world;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config_path = Path::new(&args[1][..]).join("config.json");
    let configuration = Configuration::new(config_path.to_str().unwrap());

    let world = configuration.data_process_stage();
    let data_processor = DirectDataProcessor::new();
    data_processor.write("./output_world.json", &world);
}
