use egui::Color32;
use nalgebra::Vector3;

pub struct LightSource {
    position: Vector3<f32>,
    color: Color32,
    radius_base: f32,
}

impl LightSource {
    pub fn new(z: f32, color: Color32, radius_base: f32) -> Self {
        LightSource {
            position: Vector3::<f32>::new(0.0, 0.0, z),
            color,
            radius_base,
        }
    }

    pub fn position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn position_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.position
    }

    pub fn color(&self) -> Color32 {
        self.color
    }

    pub fn color_mut(&mut self) -> &mut Color32 {
        &mut self.color
    }

    pub fn radius_base_mut(&mut self) -> &mut f32 {
        &mut self.radius_base
    }

    pub fn update_position(&mut self, t: f32) {
        let radius = self.radius_base * (t.sin() + 2.0);
        let x = radius * t.cos();
        let y = radius * t.sin();
        self.position = Vector3::<f32>::new(x, y, self.position.z);
    }
}
