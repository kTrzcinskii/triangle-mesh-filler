use std::path::Path;

use anyhow::Result;

use crate::control_points::ControlPoints;

pub struct TriangleMeshFiller {
    control_points: ControlPoints,
}

impl TriangleMeshFiller {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let control_points = ControlPoints::load_from_file(path)?;
        Ok(Self { control_points })
    }
}

impl eframe::App for TriangleMeshFiller {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Triangle Mesh Filler");
        });
    }
}
