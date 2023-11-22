use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

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
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
    info!("Start!");

    let args: Vec<String> = std::env::args().collect();
    let config_path = Path::new(&args[1][..]).join("config.json");
    let configuration = Configuration::new(config_path.to_str().unwrap());

    let world = configuration.data_process_stage();
    if let Some(output_path) = configuration.process_only_output_path {
        let output_path = config_path.parent().unwrap().join(output_path);
        DirectDataProcessor::new().write(output_path.to_str().unwrap(), &world);
        info!("Finished successfully!");
        return;
    }

    info!("Finished successfully!");
}
