use std::env;

use anyhow::{Error, Result};
use triangle_mesh_filler::TriangleMeshFiller;

mod colors_manager;
mod control_points;
mod drawer;
mod light_source;
mod mesh;
mod point;
mod polygon_filler;
mod rotations;
mod texture_loader;
mod triangle;
mod triangle_mesh_filler;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let config_path = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("config/default_config.txt");
    let app = TriangleMeshFiller::load_from_file(config_path)?;
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Triangle Mesh Filler",
        native_options,
        Box::new(|cc| {
            let style = egui::Style::default();
            cc.egui_ctx.set_style(style);
            Ok(Box::new(app))
        }),
    )
    .map_err(|e| Error::msg(e.to_string()))
}
