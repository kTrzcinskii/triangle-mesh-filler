use triangle_mesh_filler::TriangleMeshFiller;

mod triangle_mesh_filler;

fn main() -> eframe::Result<()> {
    let app = TriangleMeshFiller::new();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Triangle Mesh Filler",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
