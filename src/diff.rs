/// This file handles all the code necessary for the
/// Diff tree that compares two pieces of JSON Data
/// It uses two seperate trees (one on the left and the other on the right)
/// In order to compare and show differences between both files

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

/// Shows the differences between the files based on additions and removals
/// 
/// # Arguments
/// 
/// - `key` (`Option<&str>`) - Key ie location of each value.
/// - `old` (`&Value`) - Old removals.
/// - `new` (`&Value`) - New additions.
/// 
/// # Returns
/// 
/// - `DiffNode` - A DiffNode represents all the changes of the JSON in a single section.
/// 
/// # Examples
/// 
/// ```
/// use crate::...;
/// 
/// let _ = diff_values();
/// ```
fn diff_values(key: Option<&str>, old: &Value, new: &Value) -> DiffNode {
    let k = key.map(|s| s.to_string());

    match (old, new) {
        (Value::Object(old_map), Value::Object(new_map)) => {
            let mut children = Vec::new();

            // collect all keys from both sides and preserve
            // new_map order as the authoritative order
            // and removed keys inserted at old position
            let mut all_keys: Vec<&String> = Vec::new();
            for key in new_map.keys() {
                all_keys.push(key);
            }
            for key in old_map.keys() {
                if !new_map.contains_key(key) {
                    all_keys.push(key);
                }
            }

            for key in all_keys {
                match (old_map.get(key), new_map.get(key)) {
                    (Some(old_val), Some(new_val)) => {
                        children.push(diff_values(Some(key), old_val, new_val));
                    }
                    (Some(old_val), None) => {
                        children.push(DiffNode::Removed {
                            key: Some(key.clone()),
                            value: old_val.clone(),
                        });
                    }
                    (None, Some(new_val)) => {
                        children.push(DiffNode::Added {
                            key: Some(key.clone()),
                            value: new_val.clone(),
                        });
                    }
                    (None, None) => {}
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
    // below are all the possible cases for data types in the tree
    // and how to render them accordingly
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

/// Strips the whitespace and formats data accordingly
/// 
/// # Arguments
/// 
/// - `value` (`&Value`) - Value of the data that is to be formatted.
/// 
/// # Returns
/// 
/// - `String` - Final formatted string.
/// 
/// # Examples
/// 
/// ```
/// use crate::...;
/// 
/// let _ = format_value();
/// ```
fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s),
        Value::Null => "null".to_string(),
        other => other.to_string(),
    }
}
