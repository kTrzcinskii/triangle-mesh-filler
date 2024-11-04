use std::path::Path;

use anyhow::Result;

use crate::{control_points::ControlPoints, drawer::Drawer, mesh::Mesh};

pub struct TriangleMeshFiller {
    controls_state: ControlsState,
    control_points: ControlPoints,
    mesh: Mesh,
}

impl TriangleMeshFiller {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let controls_state = ControlsState::default();
        let control_points = ControlPoints::load_from_file(path)?;
        let mesh = Mesh::triangulation(&control_points, controls_state.triangulation_accuracy);
        Ok(Self {
            control_points,
            mesh,
            controls_state: ControlsState::default(),
        })
    }

    fn recalculate_mesh(&mut self) {
        let new_mesh = Mesh::triangulation(
            &self.control_points,
            self.controls_state.triangulation_accuracy,
        );
        self.mesh = new_mesh;
    }
}

impl eframe::App for TriangleMeshFiller {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.recalculate_mesh();

        egui::SidePanel::right("ControlsPanle")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Controls");
                ui.separator();
                ui.add(
                    egui::Slider::new(&mut self.controls_state.triangulation_accuracy, 10..=60)
                        .text("Triangulation accuracy"),
                )
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let screen_center = ui.available_rect_before_wrap().center();
            Drawer::draw_control_points(painter, &screen_center, &self.control_points);
            Drawer::draw_mesh(painter, &screen_center, &self.mesh);
        });
    }
}

struct ControlsState {
    triangulation_accuracy: usize,
}

impl Default for ControlsState {
    fn default() -> Self {
        ControlsState {
            triangulation_accuracy: 20,
        }
    }
}
