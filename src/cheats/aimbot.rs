use crate::{CheatModule, util};
use cgmath::{Array, Zero, MetricSpace, InnerSpace};
use crate::cheat;
use crate::entities::{LocalPlayer, Player, get_enemies_by_strategy};
use crate::settings::Settings;
use crate::util::math::calculate_angle;



cheat!(Aimbot);



impl CheatModule for Aimbot {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.aimbot_enabled {
            if player.is_alive() {
                if let Some(enemy) = get_enemies_by_strategy(player.get_runtime(), settings).next() {
                    let head_bone_pos = enemy.get_bone_position(settings.aim_target as usize).unwrap();
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
