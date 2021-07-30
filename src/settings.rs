use winapi::um::winuser::VK_LBUTTON;
use crate::entities::EnemySelectingStrategy;
use crate::entities::EnemySelectingStrategy::DistanceFlatten;
use crate::data::weapon::WeaponId;
use crate::cheats::AimTarget;
use eframe::egui::Color32;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs::{File, Permissions, OpenOptions};
use serde_json::Error;
use crate::util::helpers::cheat_dir;


#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub enemy_selecting_strategy: EnemySelectingStrategy,
    pub aimbot_enabled: bool,
    pub aimbot_angle: f32,
    pub aimbot_rctl: bool,
    pub aimbot_distance: u32,
    pub aimbot_damage: u32,
    pub aim_assist_enabled: bool,
    pub aim_assist_angle: u32,
    pub aim_target: usize,
    pub trigger_enabled: bool,
    pub trigger_distance: u32,
    pub radar_enabled: bool,
    pub flash_enabled: bool,
    pub bhop_enabled: bool,
    pub wh_enabled: bool,
    pub trigger_delay: u32,
    pub trigger_only_in_scope: bool,
    // pub trigger_allowed_weapons: Vec<WeaponId>,
    pub recoil_enabled: bool,
    pub recoil_shots: i32,
    pub fov: i32,
    pub force_fov: bool,
    pub fov_enabled: bool,
    pub wh_full_bloom: bool,
    #[serde(with = "Color32Def")]
    pub wh_enemy_color: Color32,
    #[serde(with = "Color32Def")]
    pub wh_local_color: Color32,
    #[serde(with = "Color32Def")]
    pub wh_inactive_color: Color32,
    pub fast_tap_enabled: bool,
    pub fast_tap_key: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            enemy_selecting_strategy: DistanceFlatten,
            aimbot_enabled: false,
            aimbot_angle: 180.0,
            aimbot_rctl: false,
            aimbot_distance: 100,
            aimbot_damage: 100,
            aim_target: 8,
            aim_assist_enabled: false,
            aim_assist_angle: 10,
            wh_enabled: false,
            wh_full_bloom: false,
            trigger_delay: 0,
            force_fov: false,
            trigger_distance: 100,
            trigger_only_in_scope: true,
            fov: 90,
            wh_enemy_color: Color32::from_rgb(255, 0, 0),
            wh_local_color: Color32::from_rgb(0, 255, 0),
            wh_inactive_color: Color32::from_rgb(0, 0, 255),
            trigger_enabled: false,
            radar_enabled: true,
            flash_enabled: true,
            recoil_shots: 1,
            fast_tap_enabled: false,
            fast_tap_key: VK_LBUTTON,
            bhop_enabled: false,
            recoil_enabled: false,
            fov_enabled: false
        }
    }
}

impl Settings {
    pub fn load() -> Result<Settings, Error> {
        let mut settings = Settings::default();
        let file = get_settings_file();
        let new_settings = serde_json::from_reader(file)?;
        settings = new_settings;
        Ok(settings)
    }

    pub fn save(&self) -> Result<(), Error> {
        let file = get_settings_file();
        serde_json::to_writer_pretty(file, &self)
    }
}

fn get_settings_file() -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(cheat_dir().join("settings.json"))
        .expect("Failed to get settings file")
}


#[derive(Serialize, Deserialize)]
#[serde(remote = "Color32")]
pub struct Color32Def([u8; 4]);

