/// Update loop and states
pub struct UnfurlApp {

}

impl Default for UnfurlApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for UnfurlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Unfurl");
            ui.label("JSON viewer coming soon");
        })
    }
}
