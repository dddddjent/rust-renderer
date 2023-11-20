#[test]
fn deserialize_configuration() {
    use crate::configuration::Configuration;
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open("../projects/basic_raytracing/config.json").unwrap();
    let reader = BufReader::new(file);
    let _: Configuration = serde_json::from_reader(reader).unwrap();
}
