use crate::entities::{Player, EntityPlayer, LocalPlayer};
use crate::{CheatModule};
use crate::cheat;
use std::time::Instant;
use std::time::Duration;
use cgmath::MetricSpace;
use crate::settings::Settings;
use crate::data::weapon::WeaponId;


cheat!(Trigger {
    next_attack: Instant = Instant::now()
});

impl CheatModule for Trigger {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.trigger_enabled {
            if let Some(crosshair_id) = player.get_crosshair_id() {
                if let Some(enemy) = EntityPlayer::get(player.get_runtime(), crosshair_id - 1) {
                    if player.get_team() != enemy.get_team()
                        && enemy.is_alive()
                        && !enemy.is_immune()
                        && self.next_attack <= Instant::now()
                        && enemy.get_distance_flatten(&player  ) <= settings.trigger_distance as f32 {
                        if let Some(index) = player.get_active_weapon_index() {
                            if settings.trigger_only_in_scope && WeaponId::get_from_index(index).is_sniper() {
                                return ();
                            }
                        }
                        player.force_attack();
                        self.next_attack = Instant::now() + Duration::from_millis(settings.trigger_delay as u64);
                    }
                }
            }
        }
    }
}
