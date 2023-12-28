use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use crate::world::camera::Camera;

use super::Renderer;
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

    /// The real entry of step
    fn step_inner(
        &mut self,
        world: &crate::world::world::World,
        output_buffer: &mut Vec<Vec<simple_math::Vector3u8>>,
    ) {
        if self.step_count == 0 {
            self.init(world);
        }
        // TODO: consider parallel
        for (x, col) in output_buffer.iter_mut().enumerate() {
            for (y, output_pixel) in col.iter_mut().enumerate() {
                let mut pixel_buffer: Vector3f = Vector3f::zeros();

                for i in 0..self.rays_per_pixel {
                    if self.rng.gen::<f32>() > self.p_continue {
                        break;
                    }

                    let mut ray = self.camera.generate_random_ray(x, y, &mut self.rng);
                    let mut color = Vector3f::new([1f32, 1f32, 1f32]);
                    let mut origin = self.camera.camera.position.clone();
                    for iter_depth in 0..self.iteration {}
                }
                pixel_buffer /= self.rays_per_pixel as f32;
                pixel_buffer
                    .iter()
                    .for_each(|val| assert!(*val < u8::MAX as f32));
                *output_pixel = Vector3u8::new([
                    *pixel_buffer.x() as u8,
                    *pixel_buffer.y() as u8,
                    *pixel_buffer.z() as u8,
                ]);
            }
        }
        self.step_count += 1;
    }
}

impl Renderer for BasicPathTracer {
    fn set_args(&mut self, args: &serde_json::Value) {
        self.set_args_inner(args)
    }

    fn step(
        &mut self,
        world: &crate::world::world::World,
        output_buffer: &mut Vec<Vec<simple_math::Vector3u8>>,
    ) {
        self.step_inner(world, output_buffer);
    }
}
