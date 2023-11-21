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

impl Camera {
    #[allow(dead_code)]
    pub fn new() -> Camera {
        Camera {
            position: Vector3f::zeros(),
            direction: Vector3f::zeros(),
            up: Vector3f::zeros(),
            distance: 0f32,
            fov: 0f32,
            size: [0u32, 0u32],
        }
    }
}
