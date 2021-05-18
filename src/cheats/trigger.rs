use crate::entities::{Player, EntityPlayer};
use crate::{CheatModule, Inverse, Runtime};

use crate::cheat;
use std::time::Instant;
use std::time::Duration;
use cgmath::MetricSpace;
use crate::settings::Settings;


cheat!(Trigger {
    next_attack: Instant = Instant::now()
});

impl CheatModule for Trigger {
    unsafe fn handle(&mut self, runtime: &mut Runtime, settings: &Settings) {
        if settings.trigger_enabled {
            if let Some(player) = runtime.get_local_player() {
                if let Some(crosshair_id) = player.get_crosshair_id() {
                    if let Some(enemy) = EntityPlayer::get(runtime, crosshair_id - 1) {
                        if player.get_team() != enemy.get_team()
                            && enemy.is_alive()
                            && !enemy.is_immune()
                            && self.next_attack <= Instant::now()
                            && player.get_position().distance(enemy.get_position()).to_radians() <= settings.trigger_distance as f32 {
                            if settings.trigger_only_in_scope && (!player.is_scoped() && player.is_sniper_weapon_in_hand()) {
                                return ();
                            }
                            player.force_attack();
                            self.next_attack = Instant::now() + Duration::from_millis(settings.trigger_delay as u64);
                        }
                    }
                }
            }
        }
    }

}
