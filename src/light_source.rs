use egui::Color32;
use nalgebra::Vector3;

pub struct LightSource {
    position: Vector3<f32>,
    color: Color32,
}

impl LightSource {
    pub fn new(position: Vector3<f32>, color: Color32) -> Self {
        LightSource { position, color }
    }

    pub fn position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn color(&self) -> Color32 {
        self.color
    }

    pub fn color_mut(&mut self) -> &mut Color32 {
        &mut self.color
    }
}
