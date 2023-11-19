use crate::world::camera::Camera;

use super::material::Material;
use super::mesh::Mesh;
use serde::Deserialize;

pub struct WorldObjects {
    pub name: String,
    pub mesh: Box<dyn Mesh>,
    pub material: Material,
}

pub struct World {
    pub camera: Camera,
    pub objects: Vec<WorldObjects>,
}
