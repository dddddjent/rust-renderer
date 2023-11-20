#[test]
fn deserialize_world() {
    use crate::world::world::World;
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open("../projects/basic_raytracing/world0.json").unwrap();
    let reader = BufReader::new(file);
    let _: World = serde_json::from_reader(reader).unwrap();
    
    let file = File::open("../projects/basic_raytracing/world1.json").unwrap();
    let reader = BufReader::new(file);
    let _: World = serde_json::from_reader(reader).unwrap();
}
