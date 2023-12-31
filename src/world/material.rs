use crate::world::serde::deserialize_color;
use renderer_derive::{GetVariant, IsVariant};
use serde::{Deserialize, Serialize};
use simple_math::Vector3;

#[derive(Serialize, Deserialize, Debug, IsVariant, GetVariant)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Material {
    Diffuse { data: Box<DiffuseMaterial> },
    Specular { data: Box<SpecularMaterial> },
    RoughMetal { data: Box<RoughMetalMaterial> },
    Dielectric { data: Box<DielectricMaterial> },
    IsotropicLight { data: Box<IsotropicLightMaterial> },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiffuseMaterial {
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Vector3<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecularMaterial {
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Vector3<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoughMetalMaterial {
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Vector3<u8>,
    pub rough_index: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DielectricMaterial {
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Vector3<u8>,
    pub n: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsotropicLightMaterial {
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Vector3<u8>,
    pub brightness: f32,
}
