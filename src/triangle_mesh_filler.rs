use std::path::Path;

use anyhow::Result;

use crate::{control_points::ControlPoints, drawer::Drawer, mesh::Mesh};

pub struct TriangleMeshFiller {
    control_points: ControlPoints,
    mesh: Mesh,
}

impl TriangleMeshFiller {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let control_points = ControlPoints::load_from_file(path)?;
        let mesh = Mesh::triangulation(&control_points, 30);
        Ok(Self {
            control_points,
            mesh,
        })
    }
}

impl eframe::App for TriangleMeshFiller {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("ControlsPanle")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Controls");
                ui.separator();
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let screen_center = ui.available_rect_before_wrap().center();
            Drawer::draw_control_points(painter, &screen_center, &self.control_points);
            Drawer::draw_mesh(painter, &screen_center, &self.mesh);
        });
    }
}
