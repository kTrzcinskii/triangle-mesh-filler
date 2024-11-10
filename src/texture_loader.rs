use std::path::Path;

use anyhow::{Error, Result};
use egui::{Color32, ColorImage};
use image::ImageReader;
use nalgebra::Vector3;

use crate::point::Point;

pub struct TextureLoader {
    texture: Option<ColorImage>,
}

impl TextureLoader {
    pub fn new() -> Self {
        Self { texture: None }
    }

    pub fn load_texture_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let image = ImageReader::open(path)?.decode()?;
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        self.texture = Some(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()));
        Ok(())
    }

    pub fn remove_texture(&mut self) {
        self.texture = None
    }

    pub fn has_texture(&self) -> bool {
        self.texture.is_some()
    }

    pub fn get_color_in_point(&self, point: &Point) -> Result<Color32> {
        self.texture
            .as_ref()
            .map(|texture| {
                let u = point.u().clamp(0.0, 0.9999);
                let v = point.v().clamp(0.0, 0.9999);

                let width = texture.size[0] as f32;
                let height = texture.size[1] as f32;

                let y = (u * width) as usize;
                let x = (v * height) as usize;
                texture.pixels[y * width as usize + x]
            })
            .ok_or(Error::msg("Missing texture"))
    }

    pub fn get_n_in_point(&self, point: &Point) -> Result<Vector3<f32>> {
        let color = self.get_color_in_point(point)?;
        Ok(Vector3::<f32>::new(
            self.scale_rgb(color.r()),
            self.scale_rgb(color.g()),
            self.scale_rgb(color.b()),
        )
        .normalize())
    }

    pub fn scale_rgb(&self, component: u8) -> f32 {
        component as f32 / (u8::MAX as f32) * 2.0 - 1.0
    }
}
