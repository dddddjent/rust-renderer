use core::f32;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use rayon::iter::ParallelIterator;
use std::f32::consts::PI;

use crate::util::collision_manager::CollisionManager;
use crate::util::tools::{ray_collision_epsilon, v3f_to_v3u8, v3u8_to_v3f};
use crate::world::camera::Camera;
use crate::world::material::{DielectricMaterial, Material};
use crate::world::world::{World, WorldObject};

use super::{OutputBuffer, Renderer};
use log::{debug, info};
use serde::Deserialize;
use simple_math::{VCross, VDot, VNorm, Vector3f, Vector3u8};

#[derive(Debug, Default, Deserialize)]
struct TracerCamera {
    camera: Camera,
    u: Vector3f, // from top to bottom
    v: Vector3f, // from left to right
    /// Real Size
    height: f32,
    /// Real size
    width: f32,
    aspect_ratio: f32,
    /// The absolute position of the camera's left bottom corner screen
    top_left_corner: Vector3f,
}

impl TracerCamera {
    fn new(mut camera: Camera) -> Self {
        let mut tracer_camera = TracerCamera::default();
        camera.fov = camera.fov * 2.0 * PI / 360.0;
        tracer_camera.aspect_ratio = camera.size.1 as f32 / camera.size.0 as f32;
        tracer_camera.height = 2.0 * f32::tan(camera.fov / 2.0) * camera.distance;
        tracer_camera.width = tracer_camera.height * tracer_camera.aspect_ratio;
        tracer_camera.u = -camera.up.normalized();
        tracer_camera.v = -camera.direction.cross(&tracer_camera.u).normalized();
        tracer_camera.top_left_corner = &camera.position + camera.distance * &camera.direction
            - 0.5
                * (tracer_camera.height * &tracer_camera.u
                    + tracer_camera.width * &tracer_camera.v);
        debug!("Top Left Corner: {}", tracer_camera.top_left_corner);

        tracer_camera.camera = camera;
        tracer_camera
    }

    #[inline]
    fn generate_random_ray<T>(&self, x_index: usize, y_index: usize, rng: &mut T) -> Vector3f
    where
        T: Rng,
    {
        let mut ray: Vector3f = &self.top_left_corner
            + self.width
                * &self.v
                * ((x_index as f32 + rng.gen::<f32>()) / self.camera.size.0 as f32)
            + self.height
                * &self.u
                * ((y_index as f32 + rng.gen::<f32>()) / self.camera.size.1 as f32)
            - &self.camera.position;
        ray.normalized()
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct BasicPathTracer {
    iteration: u32,
    rays_per_pixel: u32,
    p_continue: f32,
    n_air: f32,

    #[serde(skip_deserializing)]
    collion_manager: CollisionManager,

    #[serde(default = "TracerCamera::default")]
    camera: TracerCamera,
    #[serde(default = "u64::default")]
    step_count: u64,
}

impl BasicPathTracer {}

impl BasicPathTracer {
    pub fn new() -> Self {
        Self::default()
    }

    /// The real entry of set_args
    fn set_args_inner(&mut self, args: &serde_json::Value) {
        *self = serde_json::from_value(args.clone()).unwrap();
        info!("{:?}", self);
    }

    fn init(&mut self, world: &crate::world::world::World) {
        let camera = match &world.camera {
            Some(camera) => camera,
            None => panic!("No camera in the world!"),
        };
        self.camera = TracerCamera::new(camera.clone());
        info!("camera: {:?}", self.camera);

        self.collion_manager = CollisionManager::new();
    }

    // TODO: use (u,v) to compute color from the texture
    fn compute_color(object: &WorldObject) -> Vector3f {
        match &object.material {
            Material::Diffuse { data } => v3u8_to_v3f(&data.color),
            Material::Specular { data } => v3u8_to_v3f(&data.color),
            Material::RoughMetal { data } => v3u8_to_v3f(&data.color),
            Material::Dielectric { data } => v3u8_to_v3f(&data.color),
            Material::IsotropicLight { data } => v3u8_to_v3f(&data.color) * data.brightness,
            _ => panic!("Can't deal with this kind of material yet!"),
        }
    }

    #[inline]
    fn reflect(in_dir: &Vector3f, normal: &Vector3f) -> Vector3f {
        in_dir + 2f32 * (-in_dir.dot(normal)) * normal
    }

    fn compute_next_ray_dielectric(
        &self,
        ray: &Vector3f,
        normal: &Vector3f,
        material: &DielectricMaterial,
        rng: &mut ThreadRng,
    ) -> Vector3f {
        let ray_dot_normal = ray.dot(normal);

        let air_to_dielectric = ray_dot_normal < 0f32;
        let (n1, n2) = if air_to_dielectric {
            (self.n_air, material.n)
        } else {
            (material.n, self.n_air)
        };

        let theta_in = f32::acos(f32::abs(ray_dot_normal));
        let (theta_out, r) = if (n2 / n1 - f32::sin(theta_in)) < 0f32 {
            (PI / 2f32, 1f32)
        } else {
            let r0 = ((material.n - self.n_air) / (material.n + self.n_air)).powi(2);
            let r = r0 + (1f32 - r0) * ((1f32 - f32::cos(theta_in)).powi(5));
            let theta_out = f32::asin(n1 / n2 * f32::sin(theta_in));
            (theta_out, r)
        };

        if rng.gen::<f32>() < (r + 1e-4) {
            Self::reflect(ray, normal)
        } else {
            let n = ray.cross(normal);
            let tangent = normal.cross(&n).normalized();
            if air_to_dielectric {
                f32::sin(theta_out) * tangent - f32::cos(theta_out) * normal
            } else {
                f32::sin(theta_out) * tangent + f32::cos(theta_out) * normal
            }
        }
        .normalized()
    }

    // TODO: use (u,v) to compute the next ray from the texture
    /// return: (origin, ray)
    fn compute_next_ray(
        &self,
        object: &WorldObject,
        origin: &Vector3f,
        ray: &Vector3f,
        t: f32,
        normal: &Vector3f,
        rng: &mut ThreadRng,
    ) -> (Vector3f, Vector3f) {
        let new_origin = origin + t * ray;
        match &object.material {
            Material::Diffuse { data: _ } => {
                let phi = rng.gen::<f32>() * 2f32 * PI;
                let theta = f32::acos(rng.gen::<f32>() * 2f32 - 1f32);
                let r = 1f32;
                let x = r * f32::cos(phi) * f32::sin(theta);
                let y = r * f32::sin(phi) * f32::sin(theta);
                let z = r * f32::cos(theta);
                (new_origin, (Vector3f::new([x, y, z]) + normal).normalized())
            }
            Material::Specular { data: _ } => (new_origin, Self::reflect(ray, normal).normalized()),
            Material::RoughMetal { data } => {
                let phi = rng.gen::<f32>() * 2f32 * PI;
                let theta = f32::acos(rng.gen::<f32>() * 2f32 - 1f32);
                let r = 1f32;
                let x = r * f32::cos(phi) * f32::sin(theta);
                let y = r * f32::sin(phi) * f32::sin(theta);
                let z = r * f32::cos(theta);
                let mut new_ray = Self::reflect(ray, normal).normalized();
                new_ray += data.rough_index * Vector3f::new([x, y, z]);
                if new_ray.dot(normal) < 0f32 {
                    let n1 = new_ray.cross(normal).normalized();
                    let tangent = normal.cross(&n1).normalized();
                    new_ray = Self::reflect(&new_ray, &tangent);
                }
                (new_origin, new_ray.normalized())
            }
            Material::Dielectric { data } => (
                new_origin,
                self.compute_next_ray_dielectric(ray, normal, data.as_ref(), rng),
            ),
            Material::IsotropicLight { data: _ } => {
                panic!("If it's IsotropicLight, you shouldn't generate any ray anymore")
            }
        }
    }

    #[inline]
    fn is_light(object: &WorldObject) -> bool {
        object.material.is_isotropic_light()
    }

    #[inline]
    fn clip(val: f32, threshold: f32) -> f32 {
        if val > threshold {
            threshold
        } else {
            val
        }
    }

    #[inline]
    fn tone_remap(in_pixel: &mut Vector3f) {
        *in_pixel.x_mut() = Self::clip(f32::sqrt(*in_pixel.x()), 1.0);
        *in_pixel.y_mut() = Self::clip(f32::sqrt(*in_pixel.y()), 1.0);
        *in_pixel.z_mut() = Self::clip(f32::sqrt(*in_pixel.z()), 1.0);
    }

    fn trace_each_pixel(&self, x: usize, y: usize, world: &World) -> Vector3u8 {
        let mut pixel_buffer: Vector3f = Vector3f::zeros();
        let mut rng = thread_rng();

        for _ in 0..self.rays_per_pixel {
            if rng.gen::<f32>() > self.p_continue {
                break;
            }

            let mut ray = self.camera.generate_random_ray(x, y, &mut rng);
            let mut color = Vector3f::new([1f32, 1f32, 1f32]);
            let mut origin = self.camera.camera.position.clone();
            for _ in 0..self.iteration {
                let (object, t, normal) = match self.collion_manager.intersect(&origin, &ray, world)
                {
                    Some(result) => result,
                    None => break,
                };

                let color_mask = Self::compute_color(object) / 255f32;
                color = Vector3f::new([
                    color.x() * color_mask.x(),
                    color.y() * color_mask.y(),
                    color.z() * color_mask.z(),
                ]) / self.p_continue;
                if Self::is_light(object) {
                    pixel_buffer += color;
                    break;
                }

                assert!(t > ray_collision_epsilon());
                (origin, ray) = self.compute_next_ray(object, &origin, &ray, t, &normal, &mut rng);
            }
        }
        pixel_buffer /= self.rays_per_pixel as f32;
        Self::tone_remap(&mut pixel_buffer);
        pixel_buffer *= 255.0;
        pixel_buffer
            .iter()
            .for_each(|val| assert!(*val <= u8::MAX as f32));
        v3f_to_v3u8(&pixel_buffer)
    }

    /// The real entry of step
    fn step_inner(&mut self, world: &World, output_buffer: &mut OutputBuffer) {
        if self.step_count == 0 {
            self.init(world);
        }
        // TODO: consider parallel
        output_buffer
            .iter_2d_mut_par()
            .for_each(|(x, y, output_pixel)| *output_pixel = self.trace_each_pixel(x, y, world));
        self.step_count += 1;
    }
}

impl Renderer for BasicPathTracer {
    fn set_args(&mut self, args: &serde_json::Value) {
        self.set_args_inner(args)
    }

    fn step(&mut self, world: &World, output_buffer: &mut OutputBuffer) {
        self.step_inner(world, output_buffer);
    }
}
