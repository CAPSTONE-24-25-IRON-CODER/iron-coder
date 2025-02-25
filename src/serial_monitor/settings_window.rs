use crate::gui::GuiSettingsContainer;
use eframe::egui;
use eframe::egui::{Align2, InnerResponse, Vec2, Visuals};
use egui_theme_switch::ThemeSwitch;

pub fn settings_window(
    ctx: &egui::Context,
    gui_conf: &mut GuiSettingsContainer,
    settings_window_open: &mut bool,
) -> Option<InnerResponse<Option<()>>> {
    egui::Window::new("Settings")
        .fixed_size(Vec2 { x: 600.0, y: 200.0 })
        .anchor(Align2::CENTER_CENTER, Vec2 { x: 0.0, y: 0.0 })
        .collapsible(false)
        .show(ctx, |ui| {
            egui::Grid::new("theme settings")
                .striped(true)
                .show(ui, |ui| {
                    if ui
                        .add(ThemeSwitch::new(&mut gui_conf.theme_preference))
                        .changed()
                    {
                        ui.ctx().set_theme(gui_conf.theme_preference);
                    };
                    gui_conf.dark_mode = ui.visuals() == &Visuals::dark();

                    ui.end_row();
                    ui.end_row();
                });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Exit Settings").clicked() {
                        *settings_window_open = false;
                    }
                });

            });
        })
}
