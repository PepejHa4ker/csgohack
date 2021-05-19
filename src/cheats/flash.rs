use crate::{CheatModule, Inverse, Runtime};
use winapi::um::winuser::VK_F12;
use crate::cheat;
use crate::settings::Settings;
use crate::entities::{Player, LocalPlayer};


cheat!(AntiFlash);

impl CheatModule for AntiFlash {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.flash_enabled {
            if player.get_flash_duration() != 0.0 {
                player.set_flash_duration(0.0);
            }
        }
    }
}