use anyhow::Result;
use egui::Color32;
use nalgebra::Vector3;

use crate::{point::Point, texture_loader::TextureLoader};

#[derive(Clone)]
pub struct ColorsManager<'tl, 'nm> {
    base_color: Color32,
    texture_loader: &'tl TextureLoader,
    normal_map_loader: &'nm TextureLoader,
}

impl<'tl, 'nm> ColorsManager<'tl, 'nm> {
    pub fn new(
        base_color: Color32,
        texture_loader: &'tl TextureLoader,
        normal_map_loader: &'nm TextureLoader,
    ) -> Self {
        Self {
            base_color,
            texture_loader,
            normal_map_loader,
        }
    }

    pub fn get_point_base_color(&self, point: &Point) -> Color32 {
        let should_use_texture = self.texture_loader.has_texture();
        match should_use_texture {
            true => self
                .texture_loader
                .get_color_in_point(point)
                .expect("Should properly get color if texture is loaded"),
            false => self.base_color,
        }
    }

    pub fn get_point_n_vector(&self, point: &Point, use_normal_map: bool) -> Vector3<f32> {
        let has_normal_map = self.normal_map_loader.has_texture();
        if has_normal_map && use_normal_map {
            let normal_map_n = self.get_normal_map_n_vector(point).unwrap();
            point.after_rotation().n_with_normal_map(normal_map_n)
        } else {
            point.after_rotation().n()
        }
    }

    fn get_normal_map_n_vector(&self, point: &Point) -> Result<Vector3<f32>> {
        self.normal_map_loader.get_n_in_point(point)
    }
}
