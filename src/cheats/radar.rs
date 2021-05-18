pub use crate::{CheatModule, Inverse, Runtime};
use winapi::um::winuser::VK_F11;
use crate::cheat;
use crate::settings::Settings;
use crate::entities::Player;


cheat!(Radar);

impl CheatModule for Radar {
    unsafe fn handle(&mut self, runtime: &mut Runtime, settings: &Settings) {
        if settings.radar_enabled {
            for enemy in runtime.get_entities() {
                enemy.set_spotted(true);
            }
        }
    }

}
