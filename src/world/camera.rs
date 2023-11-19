use serde::{Deserialize, Serialize};
use simple_math::Vector3f;

#[derive(Debug, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vector3f,
    pub direction: Vector3f,
    pub up: Vector3f,
    pub distance: f32,
    pub fov: f32,
    pub size: [u32; 2],
}
