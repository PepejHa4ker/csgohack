
use crate::{CheatModule, Inverse, Runtime};
use winapi::um::winuser::VK_F12;
use crate::cheat;
use crate::settings::Settings;
use crate::entities::Player;


cheat!(AntiFlash);

impl CheatModule for AntiFlash {
    unsafe fn handle(&mut self, runtime: &mut Runtime, settings: &Settings) {
        if settings.flash_enabled {
            if let Some(player) = runtime.get_local_player() {
                if player.get_flash_duration() != 0.0 {
                    player.set_flash_duration(0.0);
                }
            }
        }
    }


}