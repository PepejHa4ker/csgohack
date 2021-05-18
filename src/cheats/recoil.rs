
use crate::{CheatModule, Inverse, Runtime};
use winapi::um::winuser::VK_F10;
use cgmath::{Vector2, Vector3};
use crate::math::Normalizable;
use crate::cheat;
use crate::settings::Settings;
use crate::entities::Player;


cheat!(Recoil {
    old_punch_angle: Vector2<f32> = Vector2::new(0.0,0.0)
});



impl CheatModule for Recoil {
    unsafe fn handle(&mut self, runtime: &mut Runtime, settings: &Settings) {
        if settings.recoil_enabled {
            if let Some(player) = runtime.get_local_player() {
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

}
