use std::path::Path;

use anyhow::Result;

use crate::{
    control_points::ControlPoints, drawer::Drawer, mesh::Mesh, polygon_filler::PolygonFiller,
};

pub struct TriangleMeshFiller {
    controls_state: ControlsState,
    control_points: ControlPoints,
    mesh: Mesh,
}

impl TriangleMeshFiller {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let controls_state = ControlsState::default();
        let control_points = ControlPoints::load_from_file(path)?;
        let mesh = Mesh::triangulation(&control_points, &controls_state);
        Ok(Self {
            control_points,
            mesh,
            controls_state,
        })
    }

    fn recalculate_mesh(&mut self) {
        let new_mesh = Mesh::triangulation(&self.control_points, &self.controls_state);
        self.mesh = new_mesh;
    }

    fn show_controls(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("ControlsPanle")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Controls");
                ui.separator();
                ui.add(
                    egui::Slider::new(&mut self.controls_state.triangulation_accuracy, 5..=40)
                        .text("Triangulation accuracy"),
                );
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Slider::new(&mut self.controls_state.alfa, -45.0..=45.0).text("Alfa"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.controls_state.beta, 0.0..=10.0).text("Beta"),
                    );
                });
                ui.checkbox(&mut self.controls_state.show_mesh, "Show mesh");
            });
    }

    fn show_central_panel(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let screen_center = ui.available_rect_before_wrap().center();
            let drawer = Drawer::new(screen_center, painter);
            let mut pf = PolygonFiller::new(self.mesh.points(), &drawer);
            for triangle in self.mesh.triangles() {
                pf.fill_polygon(triangle.vertices());
            }
            if self.controls_state.show_mesh() {
                drawer.draw_control_points(&self.control_points, &self.controls_state);
                drawer.draw_mesh(&self.mesh);
            }
        });
    }
}

impl eframe::App for TriangleMeshFiller {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.recalculate_mesh();
        self.show_controls(ctx);
        self.show_central_panel(ctx);
    }
}

pub struct ControlsState {
    triangulation_accuracy: usize,
    alfa: f32,
    beta: f32,
    show_mesh: bool,
}

impl ControlsState {
    pub fn triangulation_accuracy(&self) -> usize {
        self.triangulation_accuracy
    }

    pub fn alfa(&self) -> f32 {
        self.alfa
    }

    pub fn beta(&self) -> f32 {
        self.beta
    }

    pub fn show_mesh(&self) -> bool {
        self.show_mesh
    }
}

impl Default for ControlsState {
    fn default() -> Self {
        ControlsState {
            triangulation_accuracy: 5,
            alfa: 0.0,
            beta: 0.0,
            show_mesh: false,
        }
    }
}
