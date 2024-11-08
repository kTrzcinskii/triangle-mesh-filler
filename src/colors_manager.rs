use egui::Color32;

use crate::{point::Point, texture_loader::TextureLoader};

pub struct ColorsManager<'tl> {
    base_color: Color32,
    texture_loader: &'tl TextureLoader,
}

impl<'tl> ColorsManager<'tl> {
    pub fn new(base_color: Color32, texture_loader: &'tl TextureLoader) -> Self {
        Self {
            base_color,
            texture_loader,
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
}
