use crate::{CheatModule};

use winapi::um::winuser::{GetAsyncKeyState};
use crate::cheat;
use crate::settings::Settings;
use crate::entities::{LocalPlayer};
use std::time::{Instant};

cheat!(FastTap {
    next_attack: Instant = Instant::now()
});



impl CheatModule for FastTap {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.fast_tap_enabled {
            if GetAsyncKeyState(settings.fast_tap_key) != 0 {
                if self.next_attack <= Instant::now() {
                    player.force_attack();
                }
            }
        }
    }
}