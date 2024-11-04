pub struct TriangleMeshFiller {}

impl TriangleMeshFiller {
    pub fn new() -> Self {
        Self {}
    }
}

impl eframe::App for TriangleMeshFiller {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Triangle Mesh Filler");
        });
    }
}
