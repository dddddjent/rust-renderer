use serde::{Deserialize, Serialize};

use simple_math::{Vector3, Vector3f};
use std::collections::HashMap;

pub trait Mesh: std::fmt::Debug {}

#[derive(Serialize, Deserialize, Debug)]
pub struct SphereMesh {
    pub center: Vector3f,
    pub radius: f32,
}
impl Mesh for SphereMesh {}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaneMesh {
    pub center: Vector3f,
    pub normal: Vector3f,
}
impl Mesh for PlaneMesh {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CircleMesh {
    pub center: Vector3f,
    pub normal: Vector3f,
    pub radius: f32,
}
impl Mesh for CircleMesh {}

pub fn parse_mesh_map(mut map: HashMap<String, serde_json::Value>) -> Box<dyn Mesh> {
    let mesh_type = match map.remove("type") {
        Some(value) => serde_json::from_value::<String>(value).unwrap(),
        None => panic!("No type entry for this mesh object!"),
    };
    
    let mesh = match &mesh_type[..] {
        "sphere" => Box::new(match map.remove("data") {
            Some(value) => serde_json::from_value::<SphereMesh>(value).unwrap(),
            None => panic!("No data entry for this mesh object!"),
        }),
        _ => panic!("No such object type!"),
    };
    mesh
}
