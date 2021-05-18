use crate::{CheatModule, Runtime, Inverse};
use cgmath::{Vector4};
use winapi::um::winuser::*;
use crate::cheat;
use crate::settings::Settings;
use crate::entities::Player;


cheat!(WallHack);


#[repr(C)]
struct GlowEnemy {
    color: Vector4<f32>,
    padding: [u8; 8],
    unknown: f32,
    padding2: [u8; 4],
    render_occluded: bool,
    render_unoccluded: bool,
    full_bloom: bool,
}

// const ENEMY: GlowEnemy = GlowEnemy {
//     color: Vector4::new(1.0, 0.0, 0.2, 0.75),
//     padding: [0; 8],
//     unknown: 1.0,
//     padding2: [0; 4],
//     render_occluded: true,
//     render_unoccluded: false,
//     full_bloom: true,
// };
//
// const LOCAL_ENEMY: GlowEnemy = GlowEnemy {
//     color: Vector4::new(0.0, 1.0, 0.0, 0.75),
//
// };

fn convert_array_to_vector(data: [f32; 4]) -> Vector4<f32> {
    Vector4::new(
        *data.get(0).unwrap(),
        *data.get(1).unwrap(),
        *data.get(2).unwrap(),
        *data.get(3).unwrap(),
    )

}

fn glow_enemy_by_color(data: [f32; 4], full_bloom: bool) -> GlowEnemy {
    GlowEnemy {
        color: convert_array_to_vector(data),
        padding: [0; 8],
        unknown: 1.0,
        padding2: [0; 4],
        render_occluded: true,
        render_unoccluded: false,
        full_bloom,
    }
}

impl CheatModule for WallHack {
    unsafe fn handle(&mut self, runtime: &mut Runtime, settings: &Settings) {
        if let Some(player) = runtime.get_local_player() {
            for current_player in runtime.get_entities() {
                if let Some(glow_manager) = runtime.read_ptr::<usize>(runtime.get_signature("dwGlowObjectManager"), true) {
                    let glow_index = current_player.get_glow_index();
                    let glow = glow_manager.add(((glow_index * 0x38) + 0x4) as usize);
                    if settings.wh_enabled {
                        if current_player.is_immune() {
                            glow.cast().write(&glow_enemy_by_color(settings.wh_inactive_color, settings.wh_full_bloom))
                        } else if player.get_team() != current_player.get_team() {
                            glow.cast().write(&glow_enemy_by_color(settings.wh_enemy_color, settings.wh_full_bloom));
                        } else {
                            glow.cast().write(&glow_enemy_by_color(settings.wh_local_color, settings.wh_full_bloom));
                        }
                    } else {
                        glow.cast().write(&glow_enemy_by_color([0.0, 0.0, 0.0, 0.0], settings.wh_full_bloom));
                    }
                }
            }
        }
    }





}