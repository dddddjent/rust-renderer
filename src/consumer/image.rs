use std::path::Path;

use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use log::info;
use serde::{Deserialize, Serialize};
use simple_math::Vector3u8;

use crate::renderer::Renderer;
use crate::world::world::World;

use super::Consumer;

pub struct ImageConsumer {
    pub world: World,
    pub renderer: Box<dyn Renderer>,
    pub output_path: String,
    pub format: ImageFormat,
}

impl ImageConsumer {
    pub fn new(renderer: Box<dyn Renderer>) -> Self {
        ImageConsumer {
            world: World::new(),
            renderer,
            output_path: String::from(""),
            format: ImageFormat::Png,
        }
    }
}

impl Consumer for ImageConsumer {
    fn set_args(&mut self, config_path: &str, args: &serde_json::Value) {
        #[derive(Serialize, Deserialize, Debug)]
        struct Args {
            format: String,
            #[serde(rename = "output_name")]
            output_path: String,
        }
        let arg: Args = serde_json::from_value(args.clone()).unwrap();

        match &arg.format[..] {
            "PNG" | "Png" | "png" => self.format = ImageFormat::Png,
            "JPEG" | "Jpeg" | "jpeg" | "JPG" | "Jpg" | "jpg" => self.format = ImageFormat::Jpeg,
            "GIF" | "Gif" | "gif" => self.format = ImageFormat::Gif,
            "BMP" | "Bmp" | "bmp" => self.format = ImageFormat::Bmp,
            _ => panic!("Unknown image type!"),
        }
        let output_path = Path::new(config_path)
            .parent()
            .unwrap()
            .join(arg.output_path);
        self.output_path = String::from(output_path.to_str().unwrap());
    }

    fn set_world(&mut self, world: World) {
        self.world = world;
    }

    fn set_renderer(&mut self, renderer: Box<dyn crate::renderer::Renderer>) {
        self.renderer = renderer;
    }

    fn run(&mut self) {
        let camera = match &self.world.camera {
            Some(camera) => camera,
            None => panic!("No camera in the world!"),
        };
        let mut output_buffer = vec![vec![Vector3u8::zeros(); camera.size.1]; camera.size.0];
        self.renderer.step(&self.world, &mut output_buffer);

        let mut img: RgbImage = ImageBuffer::new(camera.size.0 as u32, camera.size.1 as u32);
        for i in 0..camera.size.0 {
            for j in 0..camera.size.1 {
                let result_pixel: &Vector3u8 = &output_buffer[i][j];
                img[(i as u32, j as u32)] =
                    Rgb([*result_pixel.x(), *result_pixel.y(), *result_pixel.z()]);
            }
        }

        info!("{}", self.output_path);
        img.save_with_format(&self.output_path[..], self.format)
            .unwrap();
    }
}
