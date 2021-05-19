use crate::{CheatModule, Runtime};
use crate::cheat;
use winapi::um::winuser::GetAsyncKeyState;
use crate::math::calculate_angle;
use cgmath::{
    InnerSpace,
    Zero,
    Array,
    MetricSpace,
    Vector2,
    Vector3,
    num_traits::Pow,
};
use itertools::Itertools;
use crate::settings::Settings;
use crate::entities::{Player, LocalPlayer};


cheat!(AimAssist);

fn fov(view_angle: Vector2<f32>, dest: Vector2<f32>, dist: f32) -> f32 {
    let pitch = (view_angle.x - dest.x).to_radians().sin() * dist;
    let yaw = (view_angle.y - dest.y).to_radians().sin() * dist;
    (pitch.powf(2.0) + yaw.powf(2.0)).sqrt()
}

impl CheatModule for AimAssist {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.aim_assist_enabled {
            while GetAsyncKeyState(settings.aim_assist_key) == 1 {
                if player.is_alive() {
                    let enemies = player.get_runtime().get_enemies()
                        .sorted_by_key(|enemy| fov(player.get_view_angles(),
                                                   calculate_angle(&player, enemy.get_bone_position(settings.aim_target as usize).unwrap(), &settings),
                                                   player.get_position().distance(enemy.get_position())) as i32)
                        .collect::<Vec<_>>();
                    if let Some(nearest_enemy) = enemies.get(0) {
                        if let Some(head_bone_pos) = nearest_enemy.get_bone_position(settings.aim_target as usize) {
                            let angle_between = calculate_angle(&player, head_bone_pos, &settings);
                            if !angle_between.is_zero() && angle_between.is_finite() {
                                player.set_view_angles(angle_between);
                            }
                        }
                    }
                }
            }
        }
    }
}