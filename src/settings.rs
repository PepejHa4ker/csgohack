use winapi::um::winuser::VK_LBUTTON;

#[derive(Copy, Clone)]
pub struct Settings {
    pub aimbot_enabled: bool,
    pub aimbot_angle: f32,
    pub aimbot_rctl: bool,
    pub aimbot_distance: u32,
    pub aimbot_damage: u32,
    pub aim_assist_enabled: bool,
    pub aim_assist_angle: u32,
    pub aim_assist_key: i32,
    pub aim_target: u32,
    pub trigger_enabled: bool,
    pub trigger_distance: u32,
    pub radar_enabled: bool,
    pub flash_enabled: bool,
    pub bhop_enabled: bool,
    pub wh_enabled: bool,
    pub trigger_delay: u32,
    pub trigger_only_in_scope: bool,
    pub recoil_enabled: bool,
    pub recoil_shots: i32,
    pub fov: i32,
    pub fov_enabled: bool,
    pub wh_full_bloom: bool,
    pub wh_enemy_color: [f32; 4],
    pub wh_local_color: [f32; 4],
    pub wh_inactive_color: [f32; 4],
    pub fast_tap_enabled: bool,
    pub fast_tap_key: i32,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            aimbot_enabled: false,
            aimbot_angle: 180.0,
            aimbot_rctl: false,
            aimbot_distance: 100,
            aimbot_damage: 100,
            aim_target: 8,
            aim_assist_enabled: false,
            aim_assist_angle: 10,
            aim_assist_key: 0x45,
            wh_enabled: false,
            wh_full_bloom: false,
            trigger_delay: 0,
            trigger_distance: 100,
            trigger_only_in_scope: true,
            recoil_enabled: false,
            fov: 90,
            fov_enabled: false,
            wh_enemy_color: [1.0, 0.0, 0.0, 1.0],
            wh_local_color: [0.0, 1.0, 0.0, 1.0],
            wh_inactive_color: [0.0, 0.0, 1.0, 1.0],
            trigger_enabled: false,
            radar_enabled: true,
            flash_enabled: false,
            bhop_enabled: false,
            recoil_shots: 1,
            fast_tap_enabled: false,
            fast_tap_key: VK_LBUTTON
        }
    }
}