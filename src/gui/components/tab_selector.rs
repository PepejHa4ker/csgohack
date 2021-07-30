use crate::gui::components::Component;
use eframe::egui::{CtxRef, Ui, menu};
use crate::{Inverse, CheatModule};
use crate::settings::Settings;
use std::sync::{Mutex, Arc};
use eframe::epi::egui::Slider;
use crate::gui::app::{layout, App};

pub struct TabSelectorComponent {
    pub tabs: Vec<TabComponent>,
}

pub enum Tab {
    Aim,
    Visuals,
    Misc,
    Movement,
    SkinChanger,
}

impl TabSelectorComponent {
    pub fn new() -> Self {
        TabSelectorComponent {
            tabs: vec![
                TabComponent { name: "Aim".to_string(), active: true, tab: Tab::Aim },
                TabComponent { name: "Visuals".to_string(), active: false, tab: Tab::Visuals },
                TabComponent { name: "Misc".to_string(), active: false, tab: Tab::Misc },
                TabComponent { name: "Movement".to_string(), active: false, tab: Tab::Movement },
                TabComponent { name: "Skin changer".to_string(), active: false, tab: Tab::SkinChanger },
            ],
        }
    }
}

pub struct TabComponent {
    name: String,
    active: bool,
    tab: Tab,
}

impl Component for TabComponent {
    fn render(&mut self, context: &CtxRef, ui: &mut Ui, settings: &mut Settings, add_contents: impl FnOnce(&mut Ui)) {
        match self.tab {
            Tab::Aim => {
                crate::cheats::aimbot::render_ui_tab(context, settings, ui);
                crate::cheats::recoil::render_ui_tab(context, settings, ui);
                crate::cheats::aimassist::render_ui_tab(context, settings, ui);
                crate::cheats::trigger::render_ui_tab(context, settings, ui);
            }
            Tab::Visuals => {
                crate::cheats::flash::render_ui_tab(context, settings, ui);
                crate::cheats::wh::render_ui_tab(context, settings, ui);
                crate::cheats::radar::render_ui_tab(context, settings, ui);
            }
            Tab::Misc => {
                crate::cheats::fov::render_ui_tab(context, settings, ui);
            }
            Tab::Movement => {
                crate::cheats::bhop::render_ui_tab(context, settings, ui);
            }
            Tab::SkinChanger => {}
        }

    }
}

impl Component for TabSelectorComponent {
    fn render(&mut self, context: &CtxRef, ui: &mut Ui, settings: &mut Settings, add_contents: impl FnOnce(&mut Ui)) {
        let mut active_tab = None;
        menu::bar(ui, |ui| {
            for (index, tab) in self.tabs.iter_mut().enumerate() {
                let label = ui.selectable_label(tab.active, &tab.name);
                if label.clicked() {
                    tab.active.inverse();
                    if tab.active {
                        active_tab = Some(index);
                    }
                }
            }
        });
        ui.separator();
        for (index, tab) in self.tabs.iter_mut().enumerate() {
            if let Some(active_tab) = active_tab {
                if active_tab != index {
                    tab.active = false;
                }
            }
            if tab.active {
                tab.render(context, ui, settings, |_| {});
            }
        }
        // });
    }
}

