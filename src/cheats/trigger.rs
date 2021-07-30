use crate::entities::{Player, EntityPlayer, LocalPlayer};
use crate::{CheatModule};
use crate::cheat;
use std::time::Instant;
use std::time::Duration;
use cgmath::MetricSpace;
use crate::settings::Settings;
use crate::data::weapon::WeaponId;
use eframe::egui::{CtxRef, Ui, Slider, Widget};
use crate::gui::app::layout;
use crate::gui::ToggleSwitch;


cheat!(Trigger {
    next_attack: Instant = Instant::now()
});

pub fn render_ui_tab(ctx: &CtxRef, settings: &mut Settings, ui: &mut Ui) {
    ui.with_layout(layout(), |ui| {
        ui.label("Trigger Bot")
    });
    ToggleSwitch::new(&mut settings.trigger_enabled, "Enabled").ui(ui);
    ui.checkbox(&mut settings.trigger_only_in_scope, "Fire only in scope");
    let slider = Slider::new(&mut settings.trigger_delay, 0..=3000)
        .text("Delay before fire (measured in ms)")
        .smart_aim(true);
    ui.add(slider);
    ui.separator();
}

unsafe impl CheatModule for Trigger {
    unsafe fn handle(&mut self, player: &LocalPlayer, settings: &Settings) {
        if settings.trigger_enabled {
            let crosshair_id = player.get_crosshair_id();
            if crosshair_id <= 0 {
                return ()
            }
            if let Some(enemy) = EntityPlayer::get(player.get_runtime(), player.get_crosshair_id() - 1) {
                if player.get_team() != enemy.get_team()
                    && enemy.is_alive()
                    && !enemy.is_immune()
                    && self.next_attack <= Instant::now() {
                    if let Some(index) = player.get_active_weapon_index() {
                        if settings.trigger_only_in_scope && WeaponId::get_from_index(index).is_sniper() && !player.is_scoped() {
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


