use std::f32::consts::PI;

use crate::world::camera::Camera;

use super::Renderer;
use log::info;
use serde::Deserialize;
use simple_math::{VCross, VNorm, Vector3f};

#[derive(Debug, Default, Deserialize)]
struct TracerCamera {
    camera: Camera,
    u: Vector3f,
    v: Vector3f,
    /// Real Size
    height: f32,
    /// Real size
    width: f32,
    aspect_ratio: f32,
    /// The absolute position of the camera's left bottom corner screen
    left_bottom_corner: Vector3f,
}

impl TracerCamera {
    fn new(mut camera: Camera) -> Self {
        let mut tracer_camera = TracerCamera::default();
        camera.fov = camera.fov * 2.0 * PI / 360.0;
        tracer_camera.aspect_ratio = camera.size.1 as f32 / camera.size.0 as f32;
        tracer_camera.height = 2.0 * f32::tan(camera.fov / 2.0) * camera.distance;
        tracer_camera.width = tracer_camera.height * tracer_camera.aspect_ratio;
        tracer_camera.u = camera.up.normalized();
        tracer_camera.v = camera.direction.cross(&tracer_camera.u).normalized();
        tracer_camera.left_bottom_corner = camera.position + camera.distance * camera.direction
            - 0.5
                * (tracer_camera.height * tracer_camera.u + tracer_camera.width * tracer_camera.v);

        tracer_camera.camera = camera;
        tracer_camera
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct BasicPathTracer {
    iteration: u32,
    rays_per_pixel: u32,
    p_continue: f32,
    n_air: f32,

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
