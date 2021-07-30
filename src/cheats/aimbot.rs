use crate::{CheatModule, util, gui};
use cgmath::{Array, Zero, InnerSpace};
use crate::cheat;
use crate::entities::{LocalPlayer, Player, get_enemies_by_strategy};
use crate::settings::Settings;
use crate::util::math::calculate_angle;
use eframe::egui::{CtxRef, Ui, Slider, ComboBox, Widget};
use crate::cheats::AimTarget::*;
use crate::gui::app::layout;
use std::sync::Arc;
use std::sync::Mutex;
use crate::gui::ToggleSwitch;




cheat!(Aimbot);

#[derive(Debug, Eq, PartialEq)]
pub enum AimTarget {
    Head = 8,
    Body = 7,
    Hands = 12
}

impl AimTarget {

    fn from_index(index: usize) -> AimTarget {
        match index {
            8 => Head,
            7 => Body,
            12 => Hands,
            _ => Head
        }
    }
}

pub fn render_ui_tab(ctx: &CtxRef, settings: &mut Settings, ui: &mut Ui) {
    ui.with_layout(layout(), |ui| {
        ui.label("Aim Bot");
    });
    ToggleSwitch::new(&mut settings.aimbot_enabled, "Enabled").ui(ui);

    let slider = Slider::new(&mut settings.aimbot_angle, 0.0..=360.0)
        .text("Angle")
        .smart_aim(true);
    ui.add(slider);
    let current = AimTarget::from_index(settings.aim_target);
    ComboBox::from_label("Aim Target bone")
        .selected_text(format!("{:?}", &current))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut settings.aim_target, 8, "Head");
            ui.selectable_value(&mut settings.aim_target, 7, "Body");
            ui.selectable_value(&mut settings.aim_target, 12, "Hands");
        });
    ui.separator();
}


unsafe impl CheatModule for Aimbot {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.aimbot_enabled {
            if player.is_alive() {
                if let Some(enemy) = get_enemies_by_strategy(player.get_runtime(), settings).next() {
                    let head_bone_pos = enemy.get_bone_position(settings.aim_target).unwrap();
                    let angle_between = calculate_angle(&player, head_bone_pos, &settings);
                    if !angle_between.is_zero() && angle_between.is_finite() {
                        if angle_between.magnitude() <= settings.aimbot_angle {
                            player.set_view_angles(angle_between);
                        }
                    }
                }
            }
        }
    }

}
