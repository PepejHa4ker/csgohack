pub use crate::{CheatModule, Inverse, Runtime};
use crate::cheat;
use crate::settings::Settings;
use crate::entities::{Player, LocalPlayer};
use eframe::egui::{CtxRef, Ui, Widget};
use crate::gui::app::layout;
use crate::gui::ToggleSwitch;


cheat!(Radar);

pub fn render_ui_tab(ctx: &CtxRef, settings: &mut Settings, ui: &mut Ui) {
    ui.with_layout(layout(), |ui| {
        ui.label("Radar")
    });
    ToggleSwitch::new(&mut settings.radar_enabled, "Enabled").ui(ui);
    ui.separator();

}

unsafe impl CheatModule for Radar {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.radar_enabled {
            for enemy in player.get_runtime().get_entities() {
                enemy.set_spotted(true);
            }
        }
    }
}
