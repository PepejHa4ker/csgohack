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
    num_traits::real::{Real},
    num_traits::Pow
};
use itertools::Itertools;
use crate::settings::Settings;
use crate::entities::Player;


cheat!(AimAssist);

fn fov(view_angle: Vector2<f32>, dest: Vector2<f32>, dist: f32) -> f32 {
    let pitch = (view_angle.x - dest.x).to_radians().sin() * dist;
    let yaw = (view_angle.y - dest.y).to_radians().sin() * dist;
    (pitch.powf(2.0) + yaw.powf(2.0)).sqrt()
}

impl CheatModule for AimAssist {
    unsafe fn handle(&mut self, runtime: &mut Runtime, settings: &Settings) {
        if settings.aim_assist_enabled {
            while GetAsyncKeyState(settings.aim_assist_key) == 1 {
                if let Some(player) = runtime.get_local_player() {
                    if player.is_alive() {
                        let enemies = runtime.get_entities()
                            .filter(|enemy| enemy.is_alive() && !enemy.is_immune())
                            .filter(|enemy| enemy.get_team() != player.get_team())
                            .sorted_by_key(|enemy| fov(player.get_view_angles(), calculate_angle(&player, enemy.get_bone_position(settings.aim_target as usize).unwrap(), &settings), player.get_position().distance(enemy.get_position())) as i32)
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
}