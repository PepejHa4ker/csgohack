
use crate::{CheatModule, Runtime};

use winapi::um::winuser::{GetAsyncKeyState, VK_SPACE};
use crate::cheat;
use crate::settings::Settings;
use crate::entities::Player;


cheat!(BHop);

impl CheatModule for BHop {
    unsafe fn handle(&mut self, runtime: &mut Runtime, settings: &Settings) {
        if settings.bhop_enabled {
            if let Some(player) = runtime.get_local_player() {
                if GetAsyncKeyState(VK_SPACE) != 0 {
                    if player.is_on_ground() {
                        player.force_jump();
                    }
                }
            }
        }
    }
}
