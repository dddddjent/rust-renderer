use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

use crate::consumer::image::ImageConsumer;
use crate::consumer::Consumer;
use crate::data_processor::direct::DirectDataProcessor;
use crate::data_processor::DataProcessor;
use crate::renderer::basic_path_tracer::BasicPathTracer;
use crate::renderer::naive_renderer::NaiveRenderer;
use crate::renderer::Renderer;
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
    pub process_only_output_path: Option<String>,
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
        // let mut data_processors: Vec<Box<dyn DataProcessor>> = vec![];
        for data_processor_configuration in &self.data_processor {
            let mut data_processor = match &data_processor_configuration.data_processor_type[..] {
                // Add new data processors
                "direct" => Box::new(DirectDataProcessor::new()),
                _ => panic!("No such data processor type!"),
            };
            let args = &data_processor_configuration.args;
            let world_processed = data_processor.process(&self.config_path[..], args).unwrap();
            world.merge_world(world_processed);
        }
        world
    }

    pub fn renderer_stage(&self) -> Box<dyn Renderer> {
        let render_configuration = match &self.renderer {
            Some(configuration) => configuration,
            None => panic!("No renderer configuration!"),
        };

        let mut renderer: Box<dyn Renderer> = match &render_configuration.renderer_type[..] {
            "naive_renderer" => Box::new(NaiveRenderer::new()),
            "basic_path_tracer" => Box::new(BasicPathTracer::new()),
            _ => panic!("No such renderer type!"),
        };
        renderer.set_args(&render_configuration.args);
        renderer
    }

    pub fn consumer_stage(&self, world: World, renderer: Box<dyn Renderer>) -> Box<dyn Consumer> {
        let consumer_configuration = match &self.consumer {
            Some(configuration) => configuration,
            None => panic!("No consumer configuration!"),
        };

        let mut consumer = match &consumer_configuration.consumer_type[..] {
            "image" => Box::new(ImageConsumer::new(renderer)),
            _ => panic!("No such consumer type!"),
        };
        consumer.set_args(&self.config_path[..], &consumer_configuration.args);
        consumer.set_world(world);
        consumer
    }
}
