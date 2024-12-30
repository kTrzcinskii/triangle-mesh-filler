use std::{
    path::Path,
    time::{Duration, Instant},
};

use anyhow::Result;
use rayon::prelude::*;
use rfd::FileDialog;

use crate::{
    colors_manager::ColorsManager, control_points::ControlPoints, drawer::Drawer,
    light_source::LightSource, mesh::Mesh, polygon_filler::PolygonFiller,
    texture_loader::TextureLoader,
};

pub struct TriangleMeshFiller {
    animation_start_time: Instant,
    controls_state: ControlsState,
    previous_controls_state: ControlsState,
    need_mesh_recalculation: bool,
    control_points: ControlPoints,
    mesh: Mesh,
    light_source: LightSource,
    texture_loader: TextureLoader,
    normal_map_loader: TextureLoader,
}

impl TriangleMeshFiller {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let controls_state = ControlsState::default();
        let control_points = ControlPoints::load_from_file(path)?;
        let mesh = Mesh::triangulation(&control_points, &controls_state);
        let light_source = LightSource::new(400.0, egui::Color32::LIGHT_GREEN, 100.0);
        let texture_loader = TextureLoader::new();
        let normal_map_loader = TextureLoader::new();
        Ok(Self {
            animation_start_time: Instant::now(),
            control_points,
            mesh,
            previous_controls_state: controls_state,
            controls_state,
            light_source,
            texture_loader,
            normal_map_loader,
            need_mesh_recalculation: true,
        })
    }

    fn recalculate_mesh(&mut self) {
        if self.need_mesh_recalculation {
            let new_mesh = Mesh::triangulation(&self.control_points, &self.controls_state);
            self.mesh = new_mesh;
            self.need_mesh_recalculation = false;
        }
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
                                5..=60,
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
                    });
                    ui.vertical(|ui| {
                        ui.label("Texture always take precedence over color. To use shape color texture must be removed.");
                        ui.horizontal(|ui| {
                            ui.label("Shape color");
                            ui.color_edit_button_srgba(&mut self.controls_state.shape_color);
                            ui.add_space(SPACING_X);
                            match self.texture_loader.has_texture() {
                                true => {
                                    if ui.button("Remove texture").clicked() {
                                        self.texture_loader.remove_texture();
                                    }
                                }
                                false => {
                                    if ui.button("Load Texture").clicked() {
                                        if let Some(path) = FileDialog::new()
                                            .add_filter("Texture", &["png", "jpg", "jpeg"])
                                            .pick_file()
                                        {
                                            self.texture_loader
                                                .load_texture_from_file(path)
                                                .expect("Should properly load texture");
                                        }
                                    }
                                }
                            }
                            ui.add_space(SPACING_X);
                            match self.normal_map_loader.has_texture() {
                                true => {
                                    if ui.button("Remove normal map").clicked() {
                                        self.normal_map_loader.remove_texture();
                                    }
                                }
                                false => {
                                    if ui.button("Load normal map").clicked() {
                                        if let Some(path) = FileDialog::new()
                                            .add_filter("Normal map", &["png", "jpg", "jpeg"])
                                            .pick_file()
                                        {
                                            self.normal_map_loader
                                                .load_texture_from_file(path)
                                                .expect("Should properly load normal map");
                                        }
                                    }
                                }
                            }
                            ui.add_space(SPACING_X);
                            ui.add_enabled(self.normal_map_loader.has_texture(), egui::Checkbox::new(&mut self.controls_state.use_normal_map, "Use texture normal map"));
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("Light color");
                        ui.color_edit_button_srgba(self.light_source.color_mut());
                        ui.add_space(SPACING_X);
                        ui.add(
                            egui::Slider::new(
                                &mut self.light_source.position_mut().z,
                                50.0..=700.0,
                            )
                            .text("Light source Z"),
                        );
                        ui.add_space(SPACING_X);
                        ui.checkbox(
                            &mut self.controls_state.show_light_source,
                            "Show light source",
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Slider::new(self.light_source.radius_base_mut(), 50.0..=300.0)
                                .text("Light source radius"),
                        );
                        ui.add_space(SPACING_X);
                        ui.checkbox(&mut self.controls_state.run_animation, "Run animation");
                    })
                });
            });

        let values_changed = self.controls_state != self.previous_controls_state;
        if values_changed {
            self.need_mesh_recalculation = true;
            self.previous_controls_state = self.controls_state;
        }
    }

    fn show_central_panel(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let screen_center = ui.available_rect_before_wrap().center();
            let drawer = Drawer::new(screen_center, painter);
            let pf = PolygonFiller::new(
                self.mesh.points(),
                &drawer,
                &self.light_source,
                ColorsManager::new(
                    self.controls_state.shape_color(),
                    &self.texture_loader,
                    &self.normal_map_loader,
                ),
                self.controls_state.kd(),
                self.controls_state.ks(),
                self.controls_state.m(),
                self.controls_state.use_normal_map(),
            );

            self.mesh.triangles().par_chunks(512).for_each(|chunk| {
                chunk.iter().for_each(|triangle| {
                    let mut pf_clone = pf.clone();
                    pf_clone.fill_polygon(triangle.vertices());
                });
            });

            if self.controls_state.show_mesh() {
                drawer.draw_control_points(&self.control_points, &self.controls_state);
                drawer.draw_mesh(&self.mesh);
            }
            if self.controls_state.show_light_source() {
                drawer.draw_light_source(&self.light_source);
            }
        });
    }
}

impl eframe::App for TriangleMeshFiller {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.recalculate_mesh();
        if self.controls_state.run_animation() {
            ctx.request_repaint_after(Duration::from_millis(16));
            let elapsed = self.animation_start_time.elapsed().as_secs_f32();
            let t = elapsed * 0.5;
            self.light_source.update_position(t);
        }
        self.show_controls(ctx);
        self.show_central_panel(ctx);
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ControlsState {
    triangulation_accuracy: usize,
    alfa: f32,
    beta: f32,
    show_mesh: bool,
    kd: f32,
    ks: f32,
    m: u8,
    shape_color: egui::Color32,
    show_light_source: bool,
    run_animation: bool,
    use_normal_map: bool,
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

    pub fn shape_color(&self) -> egui::Color32 {
        self.shape_color
    }

    pub fn show_light_source(&self) -> bool {
        self.show_light_source
    }

    pub fn run_animation(&self) -> bool {
        self.run_animation
    }

    pub fn use_normal_map(&self) -> bool {
        self.use_normal_map
    }
}

impl Default for ControlsState {
    fn default() -> Self {
        ControlsState {
            triangulation_accuracy: 5,
            alfa: 0.0,
            beta: 0.0,
            show_mesh: false,
            kd: 0.5,
            ks: 0.5,
            m: 50,
            shape_color: egui::Color32::GRAY,
            show_light_source: false,
            run_animation: true,
            use_normal_map: false,
        }
    }
}
