use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use super::DataProcessor;
use crate::world::world::World;

pub struct DirectDataProcessor {
    pub world: World,
}
impl DirectDataProcessor {
    pub fn new() -> Self {
        DirectDataProcessor {
            world: World::new(),
        }
    }
}

impl DataProcessor for DirectDataProcessor {
    fn process(&mut self, config_path: &str, args: &serde_json::Value) -> Result<(), ()> {
        #[derive(Serialize, Deserialize, Debug)]
        struct Args {
            world_path: String,
        }
        let arg: Args = serde_json::from_value(args.clone()).unwrap();
        let world_path = Path::new(config_path).parent().unwrap().join(arg.world_path);
        
        let file = File::open(world_path).unwrap();
        let reader = BufReader::new(file);
        self.world = serde_json::from_reader(reader).unwrap();
        
        Ok(())
    }

    fn write(&self, output_path: &str) {
        todo!()
    }

    fn get_erase(&mut self) -> World {
        std::mem::replace(&mut self.world, World::new())
    }

    fn get(&mut self) -> &mut World {
        &mut self.world
    }
}
