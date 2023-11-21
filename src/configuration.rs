use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

use crate::data_processor::direct::DirectDataProcessor;
use crate::data_processor::DataProcessor;
use crate::world::world::World;

#[derive(Debug, Serialize, Deserialize)]
pub struct DataProcessorConfiguration {
    #[serde(rename = "type")]
    pub data_processor_type: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RendererConfiguration {
    #[serde(rename = "type")]
    pub renderer_type: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerConfiguration {
    #[serde(rename = "type")]
    pub consumer_type: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(skip_deserializing)]
    config_path: String,
    pub data_processor: Vec<DataProcessorConfiguration>,
    pub renderer: Option<RendererConfiguration>,
    pub consumer: Option<ConsumerConfiguration>,
    pub write_only: Option<bool>,
}

impl Configuration {
    pub fn new(config_path: &str) -> Self {
        let file = File::open(&config_path).unwrap();
        let reader = BufReader::new(file);
        let mut configuration: Configuration = serde_json::from_reader(reader).unwrap();
        configuration.config_path = String::from(config_path);
        configuration
    }

    pub fn data_process_stage(&self) -> World {
        let mut world = World::new();
        let mut data_processors: Vec<Box<dyn DataProcessor>> = vec![];
        for (i, data_processor_configuration) in self.data_processor.iter().enumerate() {
            match &data_processor_configuration.data_processor_type[..] {
                // Add new data processors
                "direct" => data_processors.push(Box::new(DirectDataProcessor::new())),
                _ => panic!("No such kind of data processor type!"),
            };
            let args = &data_processor_configuration.args;
            let world_processed = data_processors[i]
                .process(&self.config_path[..], args)
                .unwrap();
            world.merge_world(world_processed);
        }
        world
    }
}
