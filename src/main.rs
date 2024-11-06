use anyhow::{Error, Result};
use triangle_mesh_filler::TriangleMeshFiller;

mod control_points;
mod drawer;
mod light_source;
mod mesh;
mod point;
mod polygon_filler;
mod rotations;
mod triangle;
mod triangle_mesh_filler;

fn main() -> Result<()> {
    let app = TriangleMeshFiller::load_from_file("config/default_config.txt")?;
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Triangle Mesh Filler",
        native_options,
        Box::new(|cc| {
            let style = egui::Style {
                visuals: egui::Visuals::light(),
                ..Default::default()
            };
            cc.egui_ctx.set_style(style);
            Ok(Box::new(app))
        }),
    )
    .map_err(|e| Error::msg(e.to_string()))
}
