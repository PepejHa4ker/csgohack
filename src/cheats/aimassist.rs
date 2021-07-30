use crate::{CheatModule};
use crate::cheat;
use winapi::um::winuser::GetAsyncKeyState;
use cgmath::{
    Zero,
    Array,
};
use crate::settings::Settings;
use crate::entities::{Player, LocalPlayer, get_enemies_by_strategy};
use crate::util::math::calculate_angle;
use eframe::egui::{CtxRef, Ui, Button, Widget};
use crate::gui::app::layout;
use crate::gui::ToggleSwitch;


cheat!(AimAssist);


pub fn render_ui_tab(ctx: &CtxRef, settings: &mut Settings, ui: &mut Ui) {
    ui.with_layout(layout(), |ui| {
        ui.label("Aim Assist");
    });
    ToggleSwitch::new(&mut settings.aim_assist_enabled, "Enabled").ui(ui);
    ui.separator();

}

unsafe impl CheatModule for AimAssist {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        // if settings.aim_assist_enabled {
        //     if GetAsyncKeyState(settings.aim_assist_key) == 1 {
        //         if player.is_alive() {
        //             if let Some(enemy) = get_enemies_by_strategy( player.get_runtime(), settings).next() {
        //                 if let Some(head_bone_pos) = enemy.get_bone_position(settings.aim_target as usize) {
        //                     let angle_between = calculate_angle(&player, head_bone_pos, &settings);
        //                     if !angle_between.is_zero() && angle_between.is_finite() {
        //                         player.set_view_angles(angle_between);
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
    }
}

