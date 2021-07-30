use eframe::{egui, epi};
use crate::settings::Settings;
use crate::gui::components::*;
use egui::*;
use eframe::egui::plot::{ Line, Values};
use std::f64::consts::*;
use std::rc::Rc;
use crate::gui::components::tab_selector::TabSelectorComponent;
use std::ops::Deref;
use crate::{Runtime, CheatModule, Map};
use std::sync::{Arc, Mutex};


pub struct App {
    tab_selector: TabSelectorComponent,
    pub last_settings: Settings,
    pub settings: Arc<Mutex<Settings>>,
}


impl App {
    pub fn create(settings: Arc<Mutex<Settings>>) -> Self {
        let last_settings = settings.lock().unwrap().clone();

        App {
            tab_selector: TabSelectorComponent::new(),
            last_settings,
            settings
        }
    }
}

pub fn layout() -> Layout {
    Layout::from_main_dir_and_cross_align(Direction::TopDown, Align::Center)
}

impl epi::App for App {

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        frame.repaint_signal().request_repaint();
        ctx.request_repaint();
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(15.0, 8.0);
        style.spacing.indent = 25.0;
        ctx.set_style(style);
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                config::ConfigComponent::default().render(ctx, ui, &mut self.last_settings, |_| {});
                faq::FaqComponent::default().render(ctx, ui, &mut self.last_settings,|_| {});
            });

        });


        CentralPanel::default().show(ctx, |ui| {
            self.tab_selector.render(ctx, ui, &mut self.last_settings, |_| {});
            user_info::UserInfoComponent::default().render(ctx, ui, &mut self.last_settings, |ui| {
                ui.hyperlink_to("Profile", "https://vk.com/atleastnotbad");

            });

        });

        let settings = self.settings.clone();
        let mut actual_settings = settings.lock().unwrap();
        *actual_settings = self.last_settings;

    }

    fn name(&self) -> &str {
        "CsgoHack"
    }
}



