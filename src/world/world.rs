use crate::world::camera::Camera;

use super::material::Material;
use super::mesh::Mesh;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldObjects {
    pub name: String,
    pub mesh: Mesh,
    pub material: Material,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub camera: Camera,
    pub objects: Vec<WorldObjects>,
}

impl World {
    pub fn merge_world(&mut self, another_world: World) {}
}
