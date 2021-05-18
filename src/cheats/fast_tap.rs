use crate::{CheatModule, Runtime};

use winapi::um::winuser::{GetAsyncKeyState, VK_SPACE};
use crate::cheat;
use crate::settings::Settings;
use crate::entities::{Player, LocalPlayer};
use std::time::{Instant, Duration};

cheat!(FastTap {
    next_attack: Instant = Instant::now()
});



impl CheatModule for FastTap {
    unsafe fn handle(&mut self, runtime: &mut Runtime, settings: &Settings) {
        if settings.fast_tap_enabled {
            if let Some(player) = runtime.get_local_player() {
                if GetAsyncKeyState(settings.fast_tap_key) != 0 {
                    if self.next_attack <= Instant::now() {
                        player.force_attack();
                    }
                }
            }
        }
    }
}