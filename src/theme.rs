use eframe::egui;

pub fn apply(ctx: &egui::Context) {
    load_fonts(ctx);
    apply_visuals(ctx);
}

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "JetBrainsMono".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/JetBrainsMono.ttf")).into(),
    );

    // place mono first into family
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "JetBrainsMono".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "JetBrainsMono".to_owned());

    ctx.set_fonts(fonts);
}

fn apply_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    visuals.panel_fill = egui::Color32::from_rgb(18, 18, 26);
    visuals.extreme_bg_color = egui::Color32::from_rgb(12, 12, 18);
    visuals.faint_bg_color = egui::Color32::from_rgb(24, 24, 34);

    // widgets
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 30, 44);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 45, 65);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(60, 60, 90);

    visuals.override_text_color = Some(egui::Color32::from_rgb(215, 215, 200));

    visuals.selection.bg_fill = egui::Color32::from_rgb(55, 75, 140);
    visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 130, 220));

    visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 40, 60));

    ctx.set_visuals(visuals);
}
