use eframe::egui;
use crate::gui::components::Component;
use egui::*;
use crate::settings::Settings;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
use crate::util::helpers::cheat_dir;
use std::io::Write;


#[derive(Default)]
pub struct ConfigComponent {

}

impl Component for ConfigComponent {
    fn render(&mut self, context: &egui::CtxRef, ui: &mut Ui, settings: &mut Settings, add_contents: impl FnOnce(&mut Ui)) {
        menu::menu(ui, "Config", |ui| {
            ui.horizontal(|ui| {
                if ui.button("Load").clicked() {
                    *settings = Settings::load().expect("Failed to load config");
                }
                if ui.button("Save").clicked() {
                    settings.save().expect("Failed to save config");
                }
            });
        });
    }
}


