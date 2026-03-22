use eframe::egui;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct SearchNode {
    pub key_match: bool,
    pub value_match: bool,
    pub subtree_match: bool,
    pub match_count: usize,
    children: Vec<SearchNode>,
}

impl SearchNode {
    /// Builds a search index with a depth-first traversal of the JSON tree.
    ///
    /// The returned structure mirrors the JSON shape so the UI can reuse it
    /// during rendering without rescanning the tree every frame.
    pub fn build(key: Option<&str>, value: &Value, query: &str) -> Self {
        let normalized_query = query.trim().to_ascii_lowercase();
        Self::build_with_query(key, value, &normalized_query)
    }

    fn build_with_query(key: Option<&str>, value: &Value, query: &str) -> Self {
        let key_match = key.is_some_and(|entry| contains_case_insensitive(entry, query));

        match value {
            Value::Object(map) => {
                let children: Vec<SearchNode> = map
                    .iter()
                    .map(|(child_key, child_value)| {
                        Self::build_with_query(Some(child_key), child_value, query)
                    })
                    .collect();

                let child_match_count: usize = children.iter().map(|child| child.match_count).sum();
                let subtree_match = key_match || children.iter().any(|child| child.subtree_match);

                Self {
                    key_match,
                    value_match: false,
                    subtree_match,
                    match_count: usize::from(key_match) + child_match_count,
                    children,
                }
            }
            Value::Array(items) => {
                let children: Vec<SearchNode> = items
                    .iter()
                    .enumerate()
                    .map(|(index, child_value)| {
                        Self::build_with_query(Some(&index.to_string()), child_value, query)
                    })
                    .collect();

                let child_match_count: usize = children.iter().map(|child| child.match_count).sum();
                let subtree_match = key_match || children.iter().any(|child| child.subtree_match);

                Self {
                    key_match,
                    value_match: false,
                    subtree_match,
                    match_count: usize::from(key_match) + child_match_count,
                    children,
                }
            }

            _ => {
                let value_match = value_matches_query(value, query);
                let subtree_match = key_match || value_match;

                Self {
                    key_match,
                    value_match,
                    subtree_match,
                    match_count: usize::from(key_match || value_match),
                    children: Vec::new(),
                }
            }
        }
    }

    pub fn child_at(&self, index: usize) -> Option<&SearchNode> {
        self.children.get(index)
    }
}

/// Recursively renders a JSON value as a collapsible tree
pub fn render_tree(
    ui: &mut egui::Ui,
    key: Option<&str>,
    value: &Value,
    search: Option<&SearchNode>,
) {
    match value {
        Value::Object(map) => {
            let label = branch_label(key, "{...}", search);
            egui::CollapsingHeader::new(label)
                .default_open(search.map_or(true, |node| node.subtree_match))
                .show(ui, |ui| {
                    for (index, (k, v)) in map.iter().enumerate() {
                        render_tree(ui, Some(k), v, child_search(search, index));
                    }
                });
        }

        Value::Array(arr) => {
            let label = branch_label(key, &format!("[{} items]", arr.len()), search);

            egui::CollapsingHeader::new(label)
                .default_open(search.map_or(true, |node| node.subtree_match))
                .show(ui, |ui| {
                    for (i, v) in arr.iter().enumerate() {
                        let idx = i.to_string();
                        render_tree(ui, Some(idx.as_str()), v, child_search(search, i));
                    }
                });
        }

        Value::String(s) => {
            ui.horizontal(|ui| {
                key_part(ui, key, search);
                value_part(
                    ui,
                    format!("\"{}\"", s),
                    egui::Color32::from_rgb(100, 200, 120),
                    search.is_some_and(|node| node.value_match),
                );
            });
        }

        Value::Number(n) => {
            ui.horizontal(|ui| {
                key_part(ui, key, search);
                value_part(
                    ui,
                    n.to_string(),
                    egui::Color32::from_rgb(100, 180, 220),
                    search.is_some_and(|node| node.value_match),
                );
            });
        }

        Value::Bool(b) => {
            ui.horizontal(|ui| {
                key_part(ui, key, search);
                value_part(
                    ui,
                    b.to_string(),
                    egui::Color32::from_rgb(220, 150, 80),
                    search.is_some_and(|node| node.value_match),
                );
            });
        }

        Value::Null => {
            ui.horizontal(|ui| {
                key_part(ui, key, search);
                value_part(
                    ui,
                    "null",
                    egui::Color32::from_rgb(160, 100, 200),
                    search.is_some_and(|node| node.value_match),
                );
            });
        }
    }
}

/// Make the collapsible header label, and prefix with the key
fn branch_label(key: Option<&str>, suffix: &str, search: Option<&SearchNode>) -> egui::RichText {
    let label = match key {
        Some(k) => format!("{k}: {suffix}"),
        None => suffix.to_string(),
    };

    if search.is_some_and(|node| node.key_match) {
        egui::RichText::new(label)
            .color(egui::Color32::from_rgb(255, 220, 120))
            .strong()
    } else {
        egui::RichText::new(label)
    }
}

/// Renders the key portion of a key value pair
fn key_part(ui: &mut egui::Ui, key: Option<&str>, search: Option<&SearchNode>) {
    if let Some(k) = key {
        let key_text = egui::RichText::new(format!("{k}:")).color(
            if search.is_some_and(|node| node.key_match) {
                egui::Color32::from_rgb(255, 220, 120)
            } else {
                egui::Color32::from_rgb(180, 180, 140)
            },
        );

        ui.label(key_text);
    }
}

fn value_part(ui: &mut egui::Ui, text: impl Into<String>, color: egui::Color32, matched: bool) {
    let content = text.into();
    let value_text = if matched {
        egui::RichText::new(content)
            .color(egui::Color32::from_rgb(255, 240, 150))
            .strong()
    } else {
        egui::RichText::new(content).color(color)
    };

    ui.label(value_text);
}

fn child_search(search: Option<&SearchNode>, index: usize) -> Option<&SearchNode> {
    search.and_then(|node| node.child_at(index))
}

fn contains_case_insensitive(content: &str, query: &str) -> bool {
    !query.is_empty() && content.to_ascii_lowercase().contains(query)
}

fn value_matches_query(value: &Value, query: &str) -> bool {
    if query.is_empty() {
        return false;
    }

    match value {
        Value::String(text) => contains_case_insensitive(text, query),
        Value::Number(number) => contains_case_insensitive(&number.to_string(), query),
        Value::Bool(boolean) => contains_case_insensitive(&boolean.to_string(), query),
        Value::Null => contains_case_insensitive("null", query),
        Value::Object(_) | Value::Array(_) => false,
    }
}
