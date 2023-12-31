use renderer_derive::{GetVariant, IsVariant};
use serde::{Deserialize, Serialize};

use simple_math::{VDot, VNorm, Vector3f};

use crate::util::tools::ray_collision_epsilon;

#[derive(Serialize, Deserialize, Debug, IsVariant, GetVariant)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Mesh {
    Sphere { data: Box<SphereMesh> },
    Plane { data: Box<PlaneMesh> },
    Circle { data: Box<CircleMesh> },
}

impl Mesh {
    pub fn intersect(&self, origin: &Vector3f, ray: &Vector3f) -> Option<(f32, Vector3f)> {
        match self {
            Mesh::Sphere { data } => data.intersect(origin, ray),
            Mesh::Plane { data } => data.intersect(origin, ray),
            Mesh::Circle { data } => data.intersect(origin, ray),
            _ => panic!("No intersect detection implemented for this mesh type yet!"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SphereMesh {
    pub center: Vector3f,
    pub radius: f32,
}

impl SphereMesh {
    pub fn intersect(&self, origin: &Vector3f, ray: &Vector3f) -> Option<(f32, Vector3f)> {
        let temp = origin - &self.center;
        let a = ray.dot(ray);
        let b = 2f32 * ray.dot(&temp);
        let c = temp.dot(&temp) - self.radius * self.radius;
        let delta = b * b - 4f32 * a * c;
        if delta < 0f32 {
            return None;
        }
        let delta_sqrt = delta.sqrt();
        let t1 = (-b + delta_sqrt) / (2f32 * a); // t1 is always larger than t2
        let t2 = (-b - delta_sqrt) / (2f32 * a);

        // get the smaller positive root
        if t2 > ray_collision_epsilon() {
            let normal = (origin + t2 * ray - &self.center).normalized();
            return Some((t2, normal));
        }
        if t1 > ray_collision_epsilon() {
            let normal = (origin + t1 * ray - &self.center).normalized();
            return Some((t1, normal));
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaneMesh {
    pub center: Vector3f,
    pub normal: Vector3f,
}

impl PlaneMesh {
    pub fn intersect(&self, origin: &Vector3f, ray: &Vector3f) -> Option<(f32, Vector3f)> {
        let a = ray.dot(&self.normal);
        if a == 0f32 {
            return None;
        }
        let b = (origin - &self.center).dot(&self.normal);
        let t = -b / a;

        if t > ray_collision_epsilon() {
            Some((t, self.normal.clone()))
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CircleMesh {
    pub center: Vector3f,
    pub normal: Vector3f,
    pub radius: f32,
}

impl CircleMesh {
    pub fn intersect(&self, origin: &Vector3f, ray: &Vector3f) -> Option<(f32, Vector3f)> {
        let a = ray.dot(&self.normal);
        if a == 0f32 {
            return None;
        }
        let b = (origin - &self.center).dot(&self.normal);
        let t = -b / a;

        if t < ray_collision_epsilon() {
            return None;
        }
        if (origin + t * ray - &self.center).norm() < self.radius {
            Some((t, self.normal.clone()))
        } else {
            None
        }
    }
}
