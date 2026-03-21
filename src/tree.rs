use eframe::egui;
use serde_json::Value;

/// Recursively renders a JSON value as a collapsible tree.
pub fn render_tree(ui: &mut egui::Ui, key: Option<&str>, value: &Value) {
    match value {
        Value::Object(map) => {
            let label = key_label(key, "{...}");
            egui::CollapsingHeader::new(label)
                .default_open(true)
                .show(ui, |ui| {
                    for (k, v) in map {
                        render_tree(ui, Some(k), v);
                    }
                });
        }

        Value::Array(arr) => {
            let label = key_label(key, &format!("[{} items]", arr.len()));

            egui::CollapsingHeader::new(label)
                .default_open(true)
                .show(ui, |ui| {
                    for (i, v) in arr.iter().enumerate() {
                        let idx = i.to_string();
                        render_tree(ui, Some(idx.as_str()), v);
                    }
                });
        }

        Value::String(s) => {
            ui.horizontal(|ui| {
                key_part(ui, key);
                ui.colored_label(
                    egui::Color32::from_rgb(100, 200, 120),
                    format!("\"{}\"", s),
                );
            });
        }

        Value::Number(n) => {
            ui.horizontal(|ui| {
                key_part(ui, key);
                ui.colored_label(egui::Color32::from_rgb(100, 180, 220), n.to_string());
            });
        }

        Value::Bool(b) => {
            ui.horizontal(|ui| {
                key_part(ui, key);
                ui.colored_label(egui::Color32::from_rgb(220, 150, 80), b.to_string());
            });
        }

        Value::Null => {
            ui.horizontal(|ui| {
                key_part(ui, key);
                ui.colored_label(egui::Color32::from_rgb(160, 100, 200), "null");
            });
        }
    }
}

/// Make the collapsible header label, and prefix with the key
fn key_label(key: Option<&str>, suffix: &str) -> String {
    match key {
        Some(k) => format!("{k}: {suffix}"),
        None => suffix.to_string(),
    }
}

/// Rendersr the key portion of a key value pair
fn key_part(ui: &mut egui::Ui, key: Option<&str>) {
    if let Some(k) = key {
        ui.colored_label(
            egui::Color32::from_rgb(180, 180, 140), 
            format!("{k}:")
        );
    }
}
