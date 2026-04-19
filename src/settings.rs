use eframe::egui;

use crate::theme::Theme;

pub fn render_settings(ui: &mut egui::Ui, current_theme: &mut Theme) -> bool {
    let mut back = false;

    ui.add_space(24.0);
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        if ui.button("← Back").clicked() {
            back = true;
        }
    });
    ui.add_space(16.0);
    ui.separator();

    ui.add_space(12.0);
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        ui.vertical(|ui| {
            ui.label("Theme");
            ui.add_space(6.0);
            egui::ComboBox::from_id_salt("theme_selector")
                .selected_text(current_theme.label())
                .width(200.0)
                .show_ui(ui, |ui| {
                    for theme in Theme::ALL {
                        ui.selectable_value(current_theme, *theme, theme.label());
                    }
                });
        });
    });
    ui.add_space(12.0);
    ui.separator();

    back
}
