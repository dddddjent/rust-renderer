use renderer_derive::{GetVariant, IsVariant};
use serde::{Deserialize, Serialize};

use simple_math::Vector3f;

#[derive(Serialize, Deserialize, Debug, IsVariant, GetVariant)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Mesh {
    Sphere { data: Box<SphereMesh> },
    Plane { data: Box<PlaneMesh> },
    Circle { data: Box<CircleMesh> },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SphereMesh {
    pub center: Vector3f,
    pub radius: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaneMesh {
    pub center: Vector3f,
    pub normal: Vector3f,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CircleMesh {
    pub center: Vector3f,
    pub normal: Vector3f,
    pub radius: f32,
}
