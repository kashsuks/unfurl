use std::f32;

use eframe::egui;

/// Update loop and states
pub struct UnfurlApp {
    input: String,
    error: Option<String>,
}

impl Default for UnfurlApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            error: None,
        }
    }
}

impl eframe::App for UnfurlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Unfurl");
                ui.separator();
                if ui.button("Format ↵").clicked() {
                    self.format();
                }
                if ui.button("Clear").clicked() {
                    self.clear();
                }
                if let Some(err) = &self.error {
                    ui.separator();
                    ui.colored_label(egui::Color32::from_rgb(220, 80, 80), err);
                }
            });
        });

        egui::SidePanel::left("input_panel")
            .resizable(true)
            .default_width(420.0)
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(6.0);
                    ui.label("Raw JSON");
                    ui.add_space(4.0);
                    egui::ScrollArea::vertical()
                        .id_salt("input_scroll")
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.input)
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(40)
                                    .hint_text("Paste JSON here..."),
                            );
                        });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(6.0);
                ui.label("Tree");
                ui.add_space(4.0);
                egui::ScrollArea::vertical()
                    .id_salt("tree_scroll")
                    .show(ui, |ui| {
                        ui.label("Tree renderer coming soon...");
                    });
            });
        });

        ctx.input(|i| {
            if i.modifiers.ctrl && i.key_pressed(egui::Key::Enter) {
                self.format();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::L) {
                self.clear();
            }

        });
    }
}

impl UnfurlApp {
    fn format(&mut self) {
        match serde_json::from_str::<serde_json::Value>(&self.input) {
            Ok(_) => self.error = None,
            Err(e) => self.error = Some(format!("Invalid JSON: {e}")),
        }
    }

    fn clear(&mut self) {
        self.input.clear();
        self.error = None;
    }
}
