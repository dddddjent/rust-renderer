use crate::world::camera::Camera;

use super::material::Material;
use super::mesh::Mesh;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldObject {
    pub name: String,
    pub mesh: Mesh,
    pub material: Material,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub camera: Option<Camera>,
    pub objects: Vec<WorldObject>,
}

impl World {
    pub fn merge_world(&mut self, mut another_world: World) {
        if another_world.camera.is_some() {
            match self.camera {
                Some(_) => panic!("More than one camera for this project"),
                None => self.camera = another_world.camera,
            }
        }
        self.objects.append(&mut another_world.objects);
    }
    pub fn new() -> Self {
        World {
            camera: None,
            objects: vec![],
        }
    }
}
