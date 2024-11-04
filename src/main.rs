use anyhow::{Error, Result};
use triangle_mesh_filler::TriangleMeshFiller;

mod control_points;
mod triangle_mesh_filler;

fn main() -> Result<()> {
    let app = TriangleMeshFiller::load_from_file("config/default_config.txt")?;
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Triangle Mesh Filler",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .map_err(|e| Error::msg(e.to_string()))
}
