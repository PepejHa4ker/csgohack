use crate::{CheatModule, Runtime};

use winapi::um::winuser::{GetAsyncKeyState, VK_SPACE};
use crate::cheat;
use crate::settings::Settings;
use crate::entities::{Player, LocalPlayer};
use eframe::egui::{CtxRef, Ui, Widget};
use eframe::egui;
use crate::gui::app::layout;
use crate::gui::ToggleSwitch;


cheat!(BHop);

pub fn render_ui_tab(ctx: &CtxRef, settings: &mut Settings, ui: &mut Ui) {
    ui.with_layout(layout(), |ui| {
        ui.label("Bunny Hop");
    });
    ToggleSwitch::new(&mut settings.bhop_enabled, "Enabled").ui(ui);
    ui.separator();

}

unsafe impl CheatModule for BHop {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.bhop_enabled {
            if GetAsyncKeyState(VK_SPACE) != 0 {
                if player.is_on_ground() {
                    player.force_jump();
                }
            }
        }
    }
}
