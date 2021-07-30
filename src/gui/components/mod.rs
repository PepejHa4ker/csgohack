use eframe::egui;
use egui::Ui;
use crate::settings::Settings;
use crate::gui::app::App;
use std::sync::Arc;
use std::sync::Mutex;

pub mod config;
pub mod faq;
pub mod user_info;
pub mod frame_history;
pub mod tab_selector;
mod player_info;

pub trait Component {

    fn render(&mut self, context: &egui::CtxRef, ui: &mut Ui, settings: &mut Settings, add_contents: impl FnOnce(&mut Ui));

}