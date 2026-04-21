use std::f32;

use eframe::egui;
use serde_json::Value;

use crate::diff::{compute_diff, render_diff_tree};
use crate::settings::render_settings;
use crate::theme::{self, Theme};
use crate::tree::{render_tree, SearchNode};

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
    search_query: String,
    parsed: Option<Value>,
    stats: Option<JsonStats>,
    search: Option<SearchNode>,
    error: Option<String>,
    theme: Theme,
    show_settings: bool,
    diff_mode: bool,
    diff_input_b: String,
    diff_error: Option<String>,
}

impl Default for UnfurlApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            search_query: String::new(),
            parsed: None,
            stats: None,
            search: None,
            error: None,
            theme: crate::persist::load_theme(),
            show_settings: false,
            diff_mode: false,
            diff_input_b: String::new(),
            diff_error: None,
        }
    }
}

impl eframe::App for UnfurlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        theme::apply(ctx, self.theme);

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.show_settings {
                    ui.heading("Settings");
                } else {
                    ui.heading("Unfurl");
                    ui.separator();

                    if ui.button("Format ↵").clicked() {
                        self.format();
                    }
                    if ui.button("Clear").clicked() {
                        self.clear();
                    }

                    ui.separator();
                    ui.label("Search");
                    let search_changed = ui
                        .add(
                            egui::TextEdit::singleline(&mut self.search_query)
                                .desired_width(180.0)
                                .hint_text("key or value"),
                        )
                        .changed();

                    if search_changed {
                        self.refresh_search();
                    }

                    if !self.search_query.is_empty() && ui.button("Reset Search").clicked() {
                        self.search_query.clear();
                        self.refresh_search();
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

                    if let Some(search) = &self.search {
                        ui.separator();
                        ui.colored_label(
                            egui::Color32::from_rgb(255, 220, 120),
                            format!("matches {}", search.match_count),
                        );
                    }
                }

                // gear button pinned to the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let gear_label = if self.show_settings { "✕" } else { "⚙" };
                    if ui.button(gear_label).clicked() {
                        self.show_settings = !self.show_settings;
                        self.diff_mode = false;
                    }
                    ui.separator();
                    let diff_label = if self.diff_mode { "Diff ✕" } else { "Diff" };
                    if ui.button(diff_label).clicked() {
                        self.diff_mode = !self.diff_mode;
                        self.show_settings = false;
                    }
                });
            });
        });

        if self.show_settings {
            egui::CentralPanel::default().show(ctx, |ui| {
                let prev_theme = self.theme;
                let back = render_settings(ui, &mut self.theme);
                if self.theme != prev_theme {
                    theme::apply(ctx, self.theme);
                    crate::persist::save_theme(self.theme);
                }
                if back {
                    self.show_settings = false;
                }
            });
            return;
        }

        if self.diff_mode {
            // Left input A
            egui::SidePanel::left("diff_input_a")
                .resizable(true)
                .default_width(300.0)
                .min_width(150.0)
                .show(ctx, |ui| {
                    ui.add_space(6.0);
                    ui.label("JSON A");
                    ui.add_space(4.0);
                    egui::ScrollArea::vertical()
                        .id_salt("diff_scroll_a")
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.input)
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(40)
                                    .hint_text("Paste first JSON here..."),
                            );
                        });
                });

            // Right input B
            egui::SidePanel::right("diff_input_b")
                .resizable(true)
                .default_width(300.0)
                .min_width(150.0)
                .show(ctx, |ui| {
                    ui.add_space(6.0);
                    ui.label("JSON B");
                    ui.add_space(4.0);
                    egui::ScrollArea::vertical()
                        .id_salt("diff_scroll_b")
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.diff_input_b)
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(40)
                                    .hint_text("Paste second JSON here..."),
                            );
                        });
                });

            // Middle diff tree
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label("Diff");
                    ui.add_space(8.0);
                    if ui.button("Compare").clicked() {
                        self.diff_error = None;
                        let a = serde_json::from_str::<Value>(&self.input);
                        let b = serde_json::from_str::<Value>(&self.diff_input_b);
                        match (a, b) {
                            (Ok(a), Ok(b)) => {
                                self.parsed = Some(Value::Array(vec![a.clone(), b.clone()]));
                                // store both for diff — reuse parsed as a sentinel
                                // we compute diff inline below using input strings
                                let _ = (a, b);
                            }
                            (Err(_), _) => self.diff_error = Some("JSON A is invalid".to_string()),
                            (_, Err(_)) => self.diff_error = Some("JSON B is invalid".to_string()),
                        }
                    }
                });
                ui.add_space(4.0);

                if let Some(err) = &self.diff_error {
                    ui.colored_label(egui::Color32::from_rgb(220, 80, 80), err);
                    return;
                }

                let a = serde_json::from_str::<Value>(&self.input);
                let b = serde_json::from_str::<Value>(&self.diff_input_b);

                match (a, b) {
                    (Ok(a), Ok(b)) => {
                        egui::ScrollArea::vertical()
                            .id_salt("diff_tree_scroll")
                            .show(ui, |ui| {
                                let diff = compute_diff(&a, &b);
                                render_diff_tree(ui, &diff);
                            });
                    }
                    _ => {
                        ui.colored_label(
                            egui::Color32::from_rgb(120, 120, 120),
                            "Paste JSON in both panels and press Compare",
                        );
                    }
                }
            });

            return;
        }

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
                    .show(ui, |ui| match &self.parsed {
                        Some(value) => {
                            render_tree(ui, None, value, self.search.as_ref());
                        }
                        None => {
                            ui.colored_label(
                                egui::Color32::from_rgb(120, 120, 120),
                                "Paste JSON on the left and press Format ↵",
                            );
                        }
                    });
            });
        });

        let dropped: Option<String> = ctx.input(|i| {
            i.raw.dropped_files.iter().find_map(|f| {
                // bytes exist case (web)
                if let Some(bytes) = &f.bytes {
                    return String::from_utf8(bytes.to_vec()).ok();
                }

                // file path (native)
                if let Some(path) = &f.path {
                    return std::fs::read_to_string(path).ok();
                }

                None
            })
        });
        if let Some(contents) = dropped {
            self.input = contents;
            self.format();
        }

        // drag-over fullscreen overlay
        let is_hovering = ctx.input(|i| !i.raw.hovered_files.is_empty());
        if is_hovering {
            let screen = ctx.screen_rect();
            let painter = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("drop_overlay"),
            ));

            painter.rect_filled(screen, 0.0, egui::Color32::from_black_alpha(160));

            let dash_color = if self.theme == Theme::Light {
                egui::Color32::from_rgb(30, 30, 30)
            } else {
                egui::Color32::WHITE
            };

            let margin = 12.0;
            let r = egui::Rect::from_min_max(
                screen.min + egui::vec2(margin, margin),
                screen.max - egui::vec2(margin, margin),
            );
            draw_dashed_rect(&painter, r, 16.0, 8.0, egui::Stroke::new(2.0, dash_color));

            painter.text(
                screen.center(),
                egui::Align2::CENTER_CENTER,
                "Drop JSON file",
                egui::FontId::proportional(24.0),
                dash_color,
            );
        }

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
        if self.input.trim().is_empty() {
            self.parsed = None;
            self.stats = None;
            self.search = None;
            self.error = Some("Invalid: Empty file".to_string());
            return;
        }

        match serde_json::from_str::<serde_json::Value>(&self.input) {
            Ok(v) => {
                self.stats = Some(JsonStats::compute(&v));
                self.parsed = Some(v);
                self.error = None;
                self.refresh_search();
            }
            Err(e) => {
                self.parsed = None;
                self.stats = None;
                self.search = None;
                self.error = Some(format!("Invalid JSON: {e}"));
            }
        }
    }

    fn clear(&mut self) {
        self.input.clear();
        self.search_query.clear();
        self.parsed = None;
        self.stats = None;
        self.search = None;
        self.error = None;
    }

    fn refresh_search(&mut self) {
        let query = self.search_query.trim();
        self.search = self
            .parsed
            .as_ref()
            .and_then(|value| (!query.is_empty()).then(|| SearchNode::build(None, value, query)));
    }
}

fn draw_dashed_rect(
    painter: &egui::Painter,
    rect: egui::Rect,
    dash_len: f32,
    gap_len: f32,
    stroke: egui::Stroke,
) {
    let step = dash_len + gap_len;

    for &y in &[rect.min.y, rect.max.y] {
        let mut x = rect.min.x;
        while x < rect.max.x {
            let end_x = (x + dash_len).min(rect.max.x);
            painter.line_segment([egui::pos2(x, y), egui::pos2(end_x, y)], stroke);
            x += step;
        }
    }

    for &x in &[rect.min.x, rect.max.x] {
        let mut y = rect.min.y;
        while y < rect.max.y {
            let end_y = (y + dash_len).min(rect.max.y);
            painter.line_segment([egui::pos2(x, y), egui::pos2(x, end_y)], stroke);
            y += step;
        }
    }
}
