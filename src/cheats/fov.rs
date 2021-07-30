use crate::CheatModule;
use crate::entities::{LocalPlayer, Player};
use crate::settings::Settings;
use crate::cheat;
use eframe::egui::{CtxRef, Ui, Widget, Slider};
use crate::gui::ToggleSwitch;
use crate::gui::app::layout;

cheat!(FOVOverride);


pub fn render_ui_tab(ctx: &CtxRef, settings: &mut Settings, ui: &mut Ui) {
    ui.with_layout(layout(), |ui| {
        ui.label("FOV")
    });
    ToggleSwitch::new(&mut settings.fov_enabled, "Enabled").ui(ui);
    ToggleSwitch::new(&mut settings.force_fov, "Force FOV").ui(ui).on_hover_text("Restores default FOV in zoom");
    Slider::new(&mut settings.fov, -180..=180).text("FOV").ui(ui);
    ui.separator();

}


unsafe impl CheatModule for FOVOverride {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.fov_enabled {
            let fov = player.get_fov();
            if fov != settings.fov {
                if !settings.force_fov && player.is_scoped() && fov != 90  {
                    player.set_fov(90);
                    return ()
                }
                player.set_fov(settings.fov);
            }
        }
    }
}