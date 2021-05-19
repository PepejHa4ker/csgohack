use crate::{CheatModule, Runtime, math};
use cgmath::{Vector3, Vector2, Array, Zero, MetricSpace, InnerSpace};
use crate::cheat;
use crate::entities::{LocalPlayer, Player};
use crate::settings::Settings;
use itertools::Itertools;


cheat!(Aimbot);



impl CheatModule for Aimbot {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.aimbot_enabled {
            if player.is_alive() {
                let enemies = player.get_runtime().get_enemies()
                    .sorted_by_key(|enemy| player.get_position().distance(enemy.get_position()).to_degrees() as u32)
                    .collect::<Vec<_>>();
                if let Some(nearest_enemy) = enemies.get(0) {
                    let head_bone_pos = nearest_enemy.get_bone_position(settings.aim_target as usize).unwrap();
                    let angle_between = math::calculate_angle(&player, head_bone_pos, &settings);
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
