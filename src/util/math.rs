use cgmath::{Vector4, Vector3, Vector2};
use crate::entities::{LocalPlayer, Player};
#[repr(C)]
#[derive(Debug)]
pub struct Matrix3x4 {
    // X Y Z W
    pub x: Vector4<f32>,
    // X Y Z W
    pub y: Vector4<f32>,
    // X Y Z W
    pub z: Vector4<f32>,
}


use crate::settings::Settings;

pub trait Normalize {
    fn normalize(&mut self) -> &Self;
}

impl Normalize for Vector2<f32> {
    fn normalize(&mut self) -> &Vector2<f32> {
        clamp(&mut self.y, -180.0, 180.0);
        clamp(&mut self.x, -89.0, 89.0);
        self
    }
}

/// Clamps the value
fn clamp(value: &mut f32, low: f32, high: f32) -> f32 {
    if *value < low {
        low
    } else {
        value.min(high)
    }
}
/// Calculate Field Of View by given angles
pub fn fov(view_angle: Vector2<f32>, dest: Vector2<f32>, dist: f32) -> f32 {
    let pitch = (view_angle.x - dest.x).to_radians().sin() * dist;
    let yaw = (view_angle.y - dest.y).to_radians().sin() * dist;
    (pitch.powf(2.0) + yaw.powf(2.0)).sqrt()
}

/// Truncates y coordinate in Vector3
pub fn truncate_y_vector<S>(vector: Vector3<S>) -> Vector2<S> {
    Vector2::new(vector.x, vector.z)
}

/// Calculates rotation angle from source player to dist Vector including settings
pub unsafe fn calculate_angle(source: &LocalPlayer, dist: Vector3<f32>, settings: &Settings) -> Vector2<f32> {
    if let Some(source_bone_pos) = source.get_head_bone_position() {
        let punch_angle: Vector2<f32> = source.get_punch_angles() * 2.0;
        let diff: Vector3<f32> = dist - (source_bone_pos + source.get_view_offset());
        let delta_length = (diff.x * diff.x + diff.y * diff.y).sqrt();
        let mut pitch = -diff.z.atan2(delta_length).to_degrees();
        let mut yaw = diff.y.atan2(diff.x).to_degrees();
        if settings.aimbot_rctl {
            pitch -= punch_angle.x;
            yaw -= punch_angle.y;
        }
        Vector2::new(pitch, yaw)
    } else {
        Vector2::new(0.0, 0.0)
    }
}