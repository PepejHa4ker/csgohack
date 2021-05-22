use crate::{CheatModule, Runtime};

use winapi::um::winuser::{GetAsyncKeyState, VK_SPACE};
use crate::cheat;
use crate::settings::Settings;
use crate::entities::{Player, LocalPlayer};


cheat!(BHop);

unsafe impl CheatModule for BHop {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.bhop_enabled {
            if GetAsyncKeyState(VK_SPACE) != 0 {
                if player.is_on_ground() {
                    player.force_jump();
                }
            }
        }
    }
}
