use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

use super::DataProcessor;
use crate::world::world::World;

pub struct DirectDataProcessor;
impl DirectDataProcessor {
    pub fn new() -> Self {
        DirectDataProcessor {}
    }
}

impl DataProcessor for DirectDataProcessor {
    fn process(&mut self, config_path: &str, args: &serde_json::Value) -> Result<World, ()> {
        #[derive(Serialize, Deserialize, Debug)]
        struct Args {
            world_path: String,
        }
        let arg: Args = serde_json::from_value(args.clone()).unwrap();
        let world_path = Path::new(config_path)
            .parent()
            .unwrap()
            .join(arg.world_path);

        let file = File::open(world_path).unwrap();
        let reader = BufReader::new(file);
        let world = serde_json::from_reader(reader).unwrap();

        Ok(world)
    }

    fn write(&self, output_path: &str, world: &World) {
        let file = File::create(output_path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, world).unwrap();
    }
}
