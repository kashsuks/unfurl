use eframe::egui;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
    Solarized,
    Dracula,
    Nord,
}

impl Theme {
    pub const ALL: &'static [Theme] = &[
        Theme::Dark,
        Theme::Light,
        Theme::Solarized,
        Theme::Dracula,
        Theme::Nord,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::Solarized => "Solarized",
            Theme::Dracula => "Dracula",
            Theme::Nord => "Nord",
        }
    }
}

pub fn apply(ctx: &egui::Context, theme: Theme) {
    load_fonts(ctx);
    apply_visuals(ctx, theme);
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

fn apply_visuals(ctx: &egui::Context, theme: Theme) {
    let mut visuals = match theme {
        Theme::Light => egui::Visuals::light(),
        _ => egui::Visuals::dark(),
    };

    match theme {
        Theme::Dark => {
            visuals.panel_fill = egui::Color32::from_rgb(18, 18, 26);
            visuals.extreme_bg_color = egui::Color32::from_rgb(12, 12, 18);
            visuals.faint_bg_color = egui::Color32::from_rgb(24, 24, 34);

            // widgets
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 30, 44);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 45, 65);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(60, 60, 90);

            visuals.override_text_color = Some(egui::Color32::from_rgb(215, 215, 200));

            visuals.selection.bg_fill = egui::Color32::from_rgb(55, 75, 140);
            visuals.selection.stroke =
                egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 130, 220));

            visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 40, 60));
        }
        Theme::Light => {
            visuals.panel_fill = egui::Color32::from_rgb(245, 245, 240);
            visuals.extreme_bg_color = egui::Color32::from_rgb(255, 255, 255);
            visuals.faint_bg_color = egui::Color32::from_rgb(235, 235, 228);

            // widgets
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(220, 220, 210);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(200, 200, 188);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(180, 180, 165);

            visuals.override_text_color = Some(egui::Color32::from_rgb(40, 40, 40));

            visuals.selection.bg_fill = egui::Color32::from_rgb(180, 200, 240);
            visuals.selection.stroke =
                egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 120, 200));

            visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 170));
        }
        Theme::Solarized => {
            visuals.panel_fill = egui::Color32::from_rgb(0, 43, 54);
            visuals.extreme_bg_color = egui::Color32::from_rgb(7, 54, 66);
            visuals.faint_bg_color = egui::Color32::from_rgb(0, 43, 54);

            // widgets
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(7, 54, 66);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(88, 110, 117);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(101, 123, 131);

            visuals.override_text_color = Some(egui::Color32::from_rgb(131, 148, 150));

            visuals.selection.bg_fill = egui::Color32::from_rgb(38, 139, 210);
            visuals.selection.stroke =
                egui::Stroke::new(1.0, egui::Color32::from_rgb(147, 161, 161));

            visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(7, 54, 66));
        }
        Theme::Dracula => {
            visuals.panel_fill = egui::Color32::from_rgb(40, 42, 54);
            visuals.extreme_bg_color = egui::Color32::from_rgb(33, 34, 44);
            visuals.faint_bg_color = egui::Color32::from_rgb(48, 50, 65);

            // widgets
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(55, 57, 74);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(68, 71, 90);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(98, 114, 164);

            visuals.override_text_color = Some(egui::Color32::from_rgb(248, 248, 242));

            visuals.selection.bg_fill = egui::Color32::from_rgb(98, 114, 164);
            visuals.selection.stroke =
                egui::Stroke::new(1.0, egui::Color32::from_rgb(189, 147, 249));

            visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(68, 71, 90));
        }
        Theme::Nord => {
            visuals.panel_fill = egui::Color32::from_rgb(46, 52, 64);
            visuals.extreme_bg_color = egui::Color32::from_rgb(36, 41, 51);
            visuals.faint_bg_color = egui::Color32::from_rgb(59, 66, 82);

            //widgets
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(67, 76, 94);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(76, 86, 106);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(129, 161, 193);

            visuals.override_text_color = Some(egui::Color32::from_rgb(216, 222, 233));

            visuals.selection.bg_fill = egui::Color32::from_rgb(94, 129, 172);
            visuals.selection.stroke =
                egui::Stroke::new(1.0, egui::Color32::from_rgb(136, 192, 208));

            visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(76, 86, 106));
        }
    }

    ctx.set_visuals(visuals);
}
