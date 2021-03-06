pub use crate::{CheatModule, Inverse, Runtime};
use crate::cheat;
use crate::settings::Settings;
use crate::entities::{Player, LocalPlayer};


cheat!(Radar);

unsafe impl CheatModule for Radar {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.radar_enabled {
            for enemy in player.get_runtime().get_entities() {
                enemy.set_spotted(true);
            }
        }
    }

}
