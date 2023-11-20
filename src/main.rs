use self::configuration::Configuration;
use self::data_processor::direct::DirectDataProcessor;
use self::data_processor::DataProcessor;
use self::world::world::World;

use std::fs::File;
use std::io::BufReader;
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
    let file = File::open(&config_path).unwrap();
    let reader = BufReader::new(file);
    let configuration: Configuration = serde_json::from_reader(reader).unwrap();

    let mut world = World::new();
    let mut data_processors: Vec<Box<dyn DataProcessor>> = vec![];
    for (i, data_processor_configuration) in configuration.data_processor.iter().enumerate() {
        match &data_processor_configuration.data_processor_type[..] {
            // Add new data processors
            "direct" => data_processors.push(Box::new(DirectDataProcessor::new())),
            _ => panic!("No such kind of data processor type!"),
        };
        let args = &data_processor_configuration.args;
        data_processors[i]
            .process(config_path.to_str().unwrap(), args)
            .unwrap();
        world.merge_world(data_processors[i].get_erase());
    }
}
