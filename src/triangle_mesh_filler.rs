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
        const SPACING_X: f32 = 30.0;
        const SPACING_Y: f32 = 25.0;
        egui::SidePanel::right("ControlsPanle")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Controls");
                ui.separator();
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = SPACING_Y;
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Slider::new(
                                &mut self.controls_state.triangulation_accuracy,
                                5..=40,
                            )
                            .text("Triangulation accuracy"),
                        );
                        ui.add_space(SPACING_X);
                        ui.checkbox(&mut self.controls_state.show_mesh, "Show mesh");
                    });
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Slider::new(&mut self.controls_state.alfa, -45.0..=45.0)
                                .text("Alfa"),
                        );
                        ui.add_space(SPACING_X);
                        ui.add(
                            egui::Slider::new(&mut self.controls_state.beta, 0.0..=10.0)
                                .text("Beta"),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Slider::new(&mut self.controls_state.kd, 0.0..=1.0).text("kd"),
                        );
                        ui.add_space(SPACING_X);
                        ui.add(
                            egui::Slider::new(&mut self.controls_state.ks, 0.0..=1.0).text("ks"),
                        );
                        ui.add_space(SPACING_X);
                        ui.add(egui::Slider::new(&mut self.controls_state.m, 1..=100).text("m"));
                    })
                });
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
    kd: f32,
    ks: f32,
    m: u8,
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

    pub fn kd(&self) -> f32 {
        self.kd
    }

    pub fn ks(&self) -> f32 {
        self.ks
    }

    pub fn m(&self) -> u8 {
        self.m
    }
}

impl Default for ControlsState {
    fn default() -> Self {
        ControlsState {
            triangulation_accuracy: 5,
            alfa: 0.0,
            beta: 0.0,
            show_mesh: false,
            kd: 0.0,
            ks: 0.0,
            m: 1,
        }
    }
}
