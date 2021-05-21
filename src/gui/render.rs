#[macro_use]
use crate::gui::*;

use imgui::*;
use cgmath::{Vector3, Vector2};

use crate::Runtime;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::f32::consts::PI;
use crate::settings::Settings;
use std::ffi::CString;
use crate::entities::EnemySelectingStrategy;


pub struct UI {}


pub fn world_to_radar(location: Vector3<f32>, origin: Vector3<f32>, angles: Vector2<f32>, width: i32) -> Vector2<f32> {
    let mut x_diff = location.x - origin.x;
    let mut y_diff = location.y - origin.y;
    let i_radar_radius = width as f32;
    let mut fl_offset = (y_diff).atan2(x_diff);
    fl_offset *= 180.0;
    fl_offset /= PI;
    if (x_diff < 0.0) && (y_diff >= 0.0) {
        fl_offset += 180.0;
    } else if (x_diff < 0.0) && (y_diff < 0.0) {
        fl_offset += 180.0;
    } else if (x_diff >= 0.0) && (y_diff < 0.0) {
        fl_offset += 360.0;
    }
    y_diff = -(((x_diff * x_diff) + (y_diff * y_diff)).sqrt());
    x_diff = 0.0;
    fl_offset -= angles.y;
    fl_offset *= PI;
    fl_offset /= 180.0;

    let mut x_new_diff = x_diff * fl_offset.cos() - y_diff * fl_offset.sin();
    let mut y_new_diff = x_diff * fl_offset.sin() + y_diff * fl_offset.cos();
    x_new_diff /= 16.0;
    y_new_diff /= 16.0;
    x_new_diff += i_radar_radius / 2.0;
    y_new_diff += i_radar_radius / 2.0;

    if x_new_diff > i_radar_radius {
        x_new_diff = i_radar_radius - 4.0;
    } else if x_new_diff < 4.0 {
        x_new_diff = 4.0
    }
    if y_new_diff > i_radar_radius {
        y_new_diff = i_radar_radius;
    } else if y_new_diff < 4.0 {
        y_new_diff = 0.0;
    }
    Vector2::new(x_new_diff, y_new_diff)
}

impl UI {
    pub unsafe fn start(runtime: &Runtime) {
        let settings = runtime.settings.clone();

        thread::spawn(move || {
            let system = init("Csgo Hack");
            let mut last_settings = Settings::new();
            let mut actual_settings = Settings::new();

            system.main_loop(move |_, ui| {
                if actual_settings != last_settings {
                    last_settings = actual_settings;
                    let mut settings = settings.lock().unwrap();
                    *settings = last_settings;
                }

                Window::new(im_str!("Config")).size([150.0, 150.0], Condition::FirstUseEver)
                    .resizable(true)
                    .build(&ui, || {
                        ui.columns(3, im_str!("###config"), false);
                        ui.push_item_width(-1.0);
                        {
                            let button_size = [ui.current_column_width() - 1.0, 20.0];
                            if ui.button(im_str!("Update"), button_size) {
                                //TODO update config
                            }
                            ui.next_column();
                            if ui.button(im_str!("Save"), button_size) {
                                //TODO update config
                            }
                            ui.next_column();

                            if ui.button(im_str!("Load"), button_size) {
                                //TODO update config
                            }
                        }
                    });
                Window::new(im_str!("Aimbot")).size([450.0, 300.0], Condition::FirstUseEver)
                    .resizable(true)
                    .build(&ui, || {
                        // window_title_color.pop(&ui);
                        ui.checkbox(im_str!("Enabled"), &mut actual_settings.aimbot_enabled);
                        ui.checkbox(im_str!("Enable Recoil Control"), &mut actual_settings.aimbot_rctl);
                        Slider::new(im_str!("Angle")).range(1.0..=180.0).build(&ui, &mut actual_settings.aimbot_angle);
                        Slider::new(im_str!("Distance")).range(1..=250).build(&ui, &mut actual_settings.aimbot_distance);
                        ui.separator();
                        ui.checkbox(im_str!("Aim assist"), &mut actual_settings.aim_assist_enabled);
                        Slider::new(im_str!("Radius")).range(1..=10).build(&ui, &mut actual_settings.aim_assist_angle);
                        let selected_name = match actual_settings.aim_target {
                            8 => "Head",
                            5 => "Body",
                            _ => "N/A"
                        };
                        ComboBox::new(im_str!("Selecting Strategy"))
                            .preview_mode(ComboBoxPreviewMode::Full)
                            .preview_value( ImStr::from_cstr_unchecked(CString::new(actual_settings.enemy_selecting_strategy.to_string()).unwrap().as_c_str()))
                            .build(&ui, || {
                                for value in EnemySelectingStrategy::iter().map(|e| *e) {
                                    if Selectable::new( ImStr::from_cstr_unchecked(CString::new(value.to_string()).unwrap().as_c_str())).selected(actual_settings.enemy_selecting_strategy == value).build(&ui) {
                                        actual_settings.enemy_selecting_strategy = value;
                                    }
                                }
                                if Selectable::new(im_str!("Head")).selected(actual_settings.aim_target == 8).build(&ui) {
                                    actual_settings.aim_target = 8;
                                }
                                if Selectable::new(im_str!("Body")).selected(actual_settings.aim_target == 5).build(&ui) {
                                    actual_settings.aim_target = 5;
                                }
                            });

                        ComboBox::new(im_str!("Bone"))
                            .preview_mode(ComboBoxPreviewMode::Full)
                            .preview_value( ImStr::from_cstr_unchecked(CString::new(selected_name).unwrap().as_c_str()))
                            .build(&ui, || {
                                if Selectable::new(im_str!("Head")).selected(actual_settings.aim_target == 8).build(&ui) {
                                    actual_settings.aim_target = 8;
                                }
                                if Selectable::new(im_str!("Body")).selected(actual_settings.aim_target == 5).build(&ui) {
                                    actual_settings.aim_target = 5;
                                }
                            });
                    });


                Window::new(im_str!("Misc")).size([250.0, 250.0], Condition::FirstUseEver)
                    .resizable(true)
                    .build(&ui, || {
                        // window_title_color.pop(&ui);
                        ui.checkbox(im_str!("FOV"), &mut actual_settings.fov_enabled);
                        ui.separator();

                        Slider::new(im_str!("Fov Changer")).range(-180..=180).build(&ui, &mut actual_settings.fov);

                        ui.checkbox(im_str!("Recoil Control"), &mut actual_settings.recoil_enabled);
                        ui.separator();
                        Slider::new(im_str!("Shot")).range(1..=100).build(&ui, &mut actual_settings.recoil_shots);

                        ui.checkbox(im_str!("FastTap"), &mut actual_settings.fast_tap_enabled);
                        ui.separator();
                    });

                Window::new(im_str!("Trigger bot")).size([250.0, 250.0], Condition::FirstUseEver)
                    .resizable(true)
                    .build(&ui, || {
                        ui.checkbox(im_str!("Enabled"), &mut actual_settings.trigger_enabled);
                        ui.checkbox(im_str!("Only scope"), &mut actual_settings.trigger_only_in_scope);
                        ui.separator();
                        Slider::new(im_str!("Delay (ms)")).range(0..=1000).build(&ui, &mut actual_settings.trigger_delay);
                        Slider::new(im_str!("Max distance")).range(0..=250).build(&ui, &mut actual_settings.trigger_distance);
                        ui.separator();
                    });
                Window::new(im_str!("WallHack")).size([250.0, 250.0], Condition::FirstUseEver)
                    .resizable(true)
                    .build(&ui, || {
                        // window_title_color.pop(&ui);
                        ui.columns(2, im_str!("###wh"), true);
                        ColorEdit::new(im_str!("Enemy color"), EditableColor::Float4(&mut actual_settings.wh_enemy_color))
                            .tooltip(false)
                            .alpha_bar(false)
                            .display_mode(ColorEditDisplayMode::RGB)
                            .input_mode(ColorEditInputMode::RGB)
                            .inputs(false)
                            .alpha(true)
                            .build(&ui);
                        ColorEdit::new(im_str!("Teammate color"), EditableColor::Float4(&mut actual_settings.wh_local_color))
                            .tooltip(false)
                            .alpha_bar(false)
                            .display_mode(ColorEditDisplayMode::RGB)
                            .input_mode(ColorEditInputMode::RGB)
                            .inputs(false)
                            .alpha(true)
                            .build(&ui);
                        ColorEdit::new(im_str!("Immune color"), EditableColor::Float4(&mut actual_settings.wh_inactive_color))
                            .tooltip(false)
                            .alpha_bar(false)
                            .display_mode(ColorEditDisplayMode::RGB)
                            .input_mode(ColorEditInputMode::RGB)
                            .inputs(false)
                            .alpha(true)
                            .build(&ui);
                        ui.next_column();
                        ui.checkbox(im_str!("Enabled"), &mut actual_settings.wh_enabled);
                        ui.checkbox(im_str!("Full bloom"), &mut actual_settings.wh_full_bloom);
                    });
                Window::new(im_str!("Anti Flash")).size([250.0, 250.0], Condition::FirstUseEver)
                    .resizable(true)
                    .build(&ui, || {
                        ui.checkbox(im_str!("Enabled"), &mut actual_settings.flash_enabled);
                        ui.separator();
                    });
                Window::new(im_str!("Bhop")).size([250.0, 250.0], Condition::FirstUseEver)
                    .resizable(true)
                    .build(&ui, || {
                        // window_title_color.pop(&ui);
                        ui.checkbox(im_str!("Enabled"), &mut actual_settings.bhop_enabled);
                        ui.separator();
                    });


                sleep(Duration::from_millis(10));
            });
        });
    }
}




