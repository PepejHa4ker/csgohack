use crate::{CheatModule};
use crate::cheat;
use crate::settings::Settings;
use crate::entities::{Player, LocalPlayer};
use eframe::egui::{CtxRef, Ui, Widget};
use crate::gui::app::layout;
use crate::gui::ToggleSwitch;


cheat!(AntiFlash);

pub fn render_ui_tab(ctx: &CtxRef, settings: &mut Settings, ui: &mut Ui) {
    ui.with_layout(layout(), |ui| {
        ui.label("Anti Flash")
    });
    ToggleSwitch::new(&mut settings.flash_enabled, "Enabled").ui(ui);
    ui.separator();
}

unsafe impl CheatModule for AntiFlash {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.flash_enabled {
            if player.get_flash_duration() != 0.0 {
                player.set_flash_duration(0.0);
            }
        }
    }
}