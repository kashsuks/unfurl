use eframe::egui;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum DiffNode {
    Unchaged {
        key: Option<String>,
        value: Value,
    },
    Added {
        key: Option<String>,
        value: Value,
    },
    Removed {
        key: Option<String>,
        value: Value,
    },
    Changed {
        key: Option<String>,
        old: Value,
        new: Value,
    },
    Object {
        key: Option<String>,
        children: Vec<DiffNode>,
    },
    Array {
        key: Option<String>,
        children: Vec<DiffNode>,
    },
}

pub fn compute_diff(old: &Value, new: &Value) -> DiffNode {
    diff_values(None, old, new)
}

fn diff_values(key: Option<&str>, old: &Value, new: &Value) -> DiffNode {
    let k = key.map(|s| s.to_string());

    match (old, new) {
        (Value::Object(old_map), Value::Object(new_map)) => {
            let mut children = Vec::new();

            for (key, old_val) in old_map {
                if let Some(new_val) = new_map.get(key) {
                    children.push(diff_values(Some(key), old_val, new_val));
                } else {
                    children.push(DiffNode::Removed {
                        key: Some(key.clone()),
                        value: old_val.clone(),
                    });
                }
            }

            for (key, new_val) in new_map {
                if !old_map.contains_key(key) {
                    children.push(DiffNode::Added {
                        key: Some(key.clone()),
                        value: new_val.clone(),
                    });
                }
            }

            DiffNode::Object { key: k, children }
        }

        (Value::Array(old_arr), Value::Array(new_arr)) => {
            let mut children = Vec::new();
            let max_len = old_arr.len().max(new_arr.len());

            for i in 0..max_len {
                let idx = i.to_string();
                match (old_arr.get(i), new_arr.get(i)) {
                    (Some(o), Some(n)) => children.push(diff_values(Some(&idx), o, n)),
                    (Some(o), None) => children.push(DiffNode::Removed {
                        key: Some(idx),
                        value: o.clone(),
                    }),
                    (None, Some(n)) => children.push(DiffNode::Added {
                        key: Some(idx),
                        value: n.clone(),
                    }),
                    (None, None) => {}
                }
            }

            DiffNode::Array { key: k, children }
        }

        _ => {
            if old == new {
                DiffNode::Unchaged {
                    key: k,
                    value: new.clone(),
                }
            } else {
                DiffNode::Changed {
                    key: k,
                    old: old.clone(),
                    new: new.clone(),
                }
            }
        }
    }
}

pub fn render_diff_tree(ui: &mut egui::Ui, node: &DiffNode) {
    match node {
        DiffNode::Object { key, children } => {
            let label = key_prefix(key, "{...}");
            egui::CollapsingHeader::new(label)
                .default_open(true)
                .show(ui, |ui| {
                    for child in children {
                        render_diff_tree(ui, child);
                    }
                });
        }

        DiffNode::Array { key, children } => {
            let label = key_prefix(key, &format!("[{} items]", children.len()));
            egui::CollapsingHeader::new(label)
                .default_open(true)
                .show(ui, |ui| {
                    for child in children {
                        render_diff_tree(ui, child);
                    }
                });
        }

        DiffNode::Unchaged { key, value } => {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(" ").color(egui::Color32::from_rgb(180, 180, 140)));
                ui.label(
                    egui::RichText::new(format_leaf(key, value))
                        .color(egui::Color32::from_rgb(180, 180, 140)),
                );
            });
        }

        DiffNode::Added { key, value } => {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("+ ")
                        .color(egui::Color32::from_rgb(100, 220, 100))
                        .strong(),
                );
                ui.label(
                    egui::RichText::new(format_leaf(key, value))
                        .color(egui::Color32::from_rgb(100, 220, 100))
                        .strong(),
                );
            });
        }

        DiffNode::Removed { key, value } => {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("- ")
                        .color(egui::Color32::from_rgb(220, 80, 80))
                        .strong(),
                );
                ui.label(
                    egui::RichText::new(format_leaf(key, value))
                        .color(egui::Color32::from_rgb(220, 80, 80))
                        .strong(),
                );
            });
        }

        DiffNode::Changed { key, old, new } => {
            let key_str = key.as_deref().unwrap_or("");
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("~ ")
                        .color(egui::Color32::from_rgb(255, 200, 60))
                        .strong(),
                );
                ui.label(
                    egui::RichText::new(format!(
                        "{}: {} -> {}",
                        key_str,
                        format_value(old),
                        format_value(new)
                    ))
                    .color(egui::Color32::from_rgb(255, 200, 60))
                    .strong(),
                );
            });
        }
    }
}

fn key_prefix(key: &Option<String>, suffix: &str) -> String {
    match key {
        Some(k) => format!("{k}: {suffix}"),
        None => suffix.to_string(),
    }
}

fn format_leaf(key: &Option<String>, value: &Value) -> String {
    let val_str = format_value(value);
    match key {
        Some(k) => format!("{k}: {val_str}"),
        None => val_str,
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s),
        Value::Null => "null".to_string(),
        other => other.to_string(),
    }
}
