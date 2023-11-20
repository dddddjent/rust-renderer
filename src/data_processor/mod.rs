use crate::world::world::World;

pub mod direct;

pub trait DataProcessor {
    fn process(&mut self,config_path:&str,args:&serde_json::Value) -> Result<(), ()>;
    fn get_erase(&mut self) -> World;
    fn get(&mut self) -> &mut World;
    fn write(&self, output_path: &str);
}
