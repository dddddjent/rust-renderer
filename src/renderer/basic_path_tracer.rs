use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use crate::util::tools::{v3f_to_v3u8, v3u8_to_v3f};
use crate::world::camera::Camera;
use crate::world::material::Material;
use crate::world::world::{World, WorldObject};

use super::{OutputBuffer, Renderer};
use log::info;
use serde::Deserialize;
use simple_math::{VCross, VNorm, Vector3f, Vector3u8};

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
        tracer_camera.v = camera.direction.cross(&tracer_camera.u).normalized();
        tracer_camera.top_left_corner = &camera.position + camera.distance * &camera.direction
            - 0.5
                * (tracer_camera.height * &tracer_camera.u
                    + tracer_camera.width * &tracer_camera.v);

        tracer_camera.camera = camera;
        tracer_camera
    }

    #[inline]
    fn generate_random_ray<T>(&self, x_index: usize, y_index: usize, rng: &mut T) -> Vector3f
    where
        T: Rng,
    {
        let ray: Vector3f = &self.top_left_corner
            + self.width
                * &self.v
                * ((x_index as f32 + rng.gen::<f32>()) / self.camera.size.0 as f32)
            + self.height
                * &self.u
                * ((y_index as f32 + rng.gen::<f32>()) / self.camera.size.1 as f32);
        ray
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct BasicPathTracer {
    iteration: u32,
    rays_per_pixel: u32,
    p_continue: f32,
    n_air: f32,

    #[serde(skip_deserializing)]
    rng: ThreadRng,

    #[serde(default = "TracerCamera::default")]
    camera: TracerCamera,
    #[serde(default = "u64::default")]
    step_count: u64,
}

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

        self.rng = thread_rng();
    }

    // TODO: return (u,v) of the intersection point
    /// return: (t, object, normal)
    fn intersect<'a>(
        origin: &Vector3f,
        ray: &Vector3f,
        world: &'a World,
    ) -> Option<(f32, &'a WorldObject, Vector3f)> {
        todo!()
    }

    // TODO: use (u,v) to compute color from the texture
    fn compute_color(object: &WorldObject) -> Vector3f {
        v3u8_to_v3f(match &object.material {
            Material::Diffuse { data } => &data.color,
            Material::Specular { data } => &data.color,
            Material::RoughMetal { data } => &data.color,
            Material::Dialetric { data } => &data.color,
            Material::IsotropicLight { data } => &data.color,
            _ => panic!("Can't deal with this kind of material yet!"),
        })
    }

    // TODO: use (u,v) to compute the next ray from the texture
    /// return: (origin, ray)
    fn compute_next_ray(object: &WorldObject, normal: &Vector3f) -> (Vector3f, Vector3f) {
        todo!()
    }

    #[inline]
    fn is_light(object: &WorldObject) -> bool {
        object.material.is_isotropic_light()
    }

    fn trace_each_pixel(&mut self, x: usize, y: usize, world: &World) -> Vector3u8 {
        let mut pixel_buffer: Vector3f = Vector3f::zeros();

        for _ in 0..self.rays_per_pixel {
            if self.rng.gen::<f32>() > self.p_continue {
                break;
            }

            let mut ray = self.camera.generate_random_ray(x, y, &mut self.rng);
            let mut color = Vector3f::new([1f32, 1f32, 1f32]);
            let mut origin = self.camera.camera.position.clone();
            for _ in 0..self.iteration {
                let (_, object, normal) = match Self::intersect(&origin, &ray, world) {
                    Some(result) => result,
                    None => break,
                };

                let color_mask = Self::compute_color(object) / 255f32;
                color
                    .iter_mut()
                    .zip(color_mask.iter())
                    .for_each(|(val, mask)| *val = *val * mask / self.p_continue);
                if Self::is_light(object) {
                    pixel_buffer += color;
                    break;
                }

                (origin, ray) = Self::compute_next_ray(object, &normal);
            }
        }
        pixel_buffer /= self.rays_per_pixel as f32;
        pixel_buffer
            .iter()
            .for_each(|val| assert!(*val < u8::MAX as f32));
        v3f_to_v3u8(&pixel_buffer)
    }

    /// The real entry of step
    fn step_inner(&mut self, world: &World, output_buffer: &mut OutputBuffer) {
        if self.step_count == 0 {
            self.init(world);
        }
        // TODO: consider parallel
        output_buffer
            .iter_2d_mut()
            .for_each(|(x, y, output_pixel)| *output_pixel = self.trace_each_pixel(x, y, &world));
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
