use std::f32;

use eframe::egui;
use serde_json::{Value};
use crate::tree::render_tree;

/// Stats from the parsed JSON data
struct JsonStats {
    nodes: usize,
    depth: usize,
}

impl JsonStats {
    fn compute(value: &Value) -> Self {
        Self {
            nodes: count_nodes(value),
            depth: max_depth(value, 0),
        }
    }
}

fn count_nodes(value: &Value) -> usize {
    match value {
        Value::Object(map) => 1 + map.values().map(count_nodes).sum::<usize>(),
        Value::Array(arr) => 1 + arr.iter().map(count_nodes).sum::<usize>(),
        _ => 1,
    }
}

fn max_depth(value: &Value, current: usize) -> usize {
    match value {
        Value::Object(map) => map
            .values()
            .map(|v| max_depth(v, current + 1))
            .max()
            .unwrap_or(current + 1),
        Value::Array(arr) => arr
            .iter()
            .map(|v| max_depth(v, current + 1))
            .max()
            .unwrap_or(current + 1),
        _ => current,
    }
}

/// Update loop and states
pub struct UnfurlApp {
    input: String,
    parsed: Option<Value>,
    stats: Option<JsonStats>,
    error: Option<String>,
}

impl Default for UnfurlApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            parsed: None,
            stats: None,
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

                if let Some(stats) = &self.stats {
                    ui.separator();
                    ui.colored_label(
                        egui::Color32::from_rgb(140, 140, 170), 
                        format!("nodes: {} depth: {}", stats.nodes, stats.depth),
                    );
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
                        match &self.parsed {
                            Some(value) => {
                                render_tree(ui, None, value);
                            }
                            None => {
                                ui.colored_label(
                                    egui::Color32::from_rgb(120, 120, 120), 
                                    "Paste JSON on the left and press Format ↵",
                                );
                            }
                        }
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
            Ok(v) => {
                self.stats = Some(JsonStats::compute(&v));
                self.parsed = Some(v);
                self.error = None;
            }
            Err(e) => {
                self.parsed = None;
                self.stats = None;
                self.error = Some(format!("Invalid JSON: {e}"));
            }
        }
    }

    fn clear(&mut self) {
        self.input.clear();
        self.parsed = None;
        self.stats = None;
        self.error = None;
    }
}
