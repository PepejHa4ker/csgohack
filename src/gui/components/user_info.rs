use crate::gui::components::Component;
use eframe::egui;
use eframe::egui::*;
use crate::settings::Settings;
use crate::gui::app::App;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Default)]
pub struct UserInfoComponent {}

impl Component for UserInfoComponent {
    fn render(&mut self, context: &egui::CtxRef, ui: &mut Ui, settings: &mut Settings, add_contents: impl FnOnce(&mut Ui)) {
        SidePanel::right("user_info_side_panel")
            .resizable(false)
            .show(context, |ui| {
                ui.label(format!("Your username: {}", "test"));
                ui.label(format!("Subscription till: Infinity"));
                add_contents(ui);

            });
    }
}