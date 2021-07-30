use crate::{CheatModule};
use cgmath::{Vector4};
use crate::cheat;
use crate::settings::Settings;
use crate::entities::{Player, LocalPlayer};
use eframe::egui::{CtxRef, Ui, Color32, Rgba, menu, Widget};
use crate::gui::app::layout;
use std::sync::Arc;
use std::sync::Mutex;
use crate::gui::ToggleSwitch;


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

fn convert_color_to_vector(color: Color32) -> Vector4<f32> {
    Vector4::new(
        color.r() as f32/255.0,
        color.g() as f32/255.0,
        color.b() as f32/255.0,
        color.a() as f32/255.0,
    )
}

fn glow_enemy_by_color(color: Color32, full_bloom: bool) -> GlowEnemy {
    GlowEnemy {
        color: convert_color_to_vector(color),
        padding: [0; 8],
        unknown: 1.0,
        padding2: [0; 4],
        render_occluded: true,
        render_unoccluded: false,
        full_bloom,
    }
}


pub fn render_ui_tab(ctx: &CtxRef, settings: &mut Settings, ui: &mut Ui) {
    ui.with_layout(layout(), |ui| {
        ui.label("Wall Hack");
    });
    ToggleSwitch::new(&mut settings.wh_enabled, "Enabled").ui(ui);
    ui.horizontal(|ui| {
        ui.color_edit_button_srgba(&mut settings.wh_enemy_color);
        ui.label("Enemy color");
    });
    ui.horizontal(|ui| {
        ui.color_edit_button_srgba(&mut settings.wh_local_color);
        ui.label("Team color");
    });
    ui.horizontal(|ui| {
        ui.color_edit_button_srgba(&mut settings.wh_inactive_color);
        ui.label("Inactive color");
    });
    ui.separator();



}

unsafe impl CheatModule for WallHack {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        for current_player in player.get_runtime().get_entities() {
            let glow_manager = player.get_glow_object();
            let glow_index = current_player.get_glow_index();
            let glow = glow_manager.add(((glow_index * 0x38) + 0x4) as usize);
            if settings.wh_enabled {
                if current_player.is_immune() {
                    glow.cast().write(&glow_enemy_by_color(settings.wh_inactive_color, settings.wh_full_bloom));
                } else if player.get_team() != current_player.get_team() {
                    glow.cast().write(&glow_enemy_by_color(settings.wh_enemy_color, settings.wh_full_bloom));
                } else {
                    glow.cast().write(&glow_enemy_by_color(settings.wh_local_color, settings.wh_full_bloom));
                }
            } else {
                glow.cast().write(&glow_enemy_by_color(Color32::from_black_alpha(255), settings.wh_full_bloom));
            }
        }
    }
}
