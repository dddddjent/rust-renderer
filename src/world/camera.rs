use serde::{Deserialize, Serialize};
use simple_math::Vector3f;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Camera {
    pub position: Vector3f,
    pub direction: Vector3f,
    pub up: Vector3f,
    pub distance: f32,
    pub fov: f32,
    /// In pixel: (width, height)
    pub size: (usize, usize),
}

impl Camera {
    #[allow(dead_code)]
    pub fn new() -> Camera {
        Camera::default()
    }
}
