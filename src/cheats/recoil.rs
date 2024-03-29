use crate::{CheatModule};
use cgmath::{Vector2, InnerSpace};
use crate::cheat;
use crate::settings::Settings;
use crate::entities::{Player, LocalPlayer};
use eframe::egui::{CtxRef, Ui, Widget};
use crate::gui::app::layout;
use crate::gui::ToggleSwitch;


cheat!(Recoil {
    old_punch_angle: Vector2<f32> = Vector2::new(0.0,0.0)
});


pub fn render_ui_tab(ctx: &CtxRef, settings: &mut Settings, ui: &mut Ui) {
    ui.with_layout(layout(), |ui| {
        ui.label("Recoil Control")
    });
    ToggleSwitch::new(&mut settings.aimbot_rctl, "Enabled").ui(ui);

    ui.separator();

}

unsafe impl CheatModule for Recoil {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.recoil_enabled {
            if player.get_shots_fired() >= settings.recoil_shots {
                let view_angles = player.get_view_angles();
                let punch_angles: Vector2<f32> = player.get_punch_angles() * 2.0;
                let mut new_angle: Vector2<f32> = view_angles + self.old_punch_angle - punch_angles;
                new_angle.normalize();
                player.set_view_angles(new_angle);
                self.old_punch_angle = punch_angles;
            } else {
                self.old_punch_angle.x = 0.0;
                self.old_punch_angle.y = 0.0;
            }
        }
    }

}
