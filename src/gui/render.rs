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
use std::ptr::null;

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

            system.main_loop(move |_, ui| {
                let mut settings = settings.lock().unwrap();

                macro_rules! window {
                            ($name:literal, ($width:literal, $height:literal) : $block:block) => {
                                Window::new(im_str!($name)).size([$width, $height], Condition::FirstUseEver)
                                .resizable(true)
                                .build(&ui, || $block);
                            }
                        }

                macro_rules! hover {
                            ($text:literal) => {
                             if ui.is_item_hovered() {
                                 ui.tooltip_text($text);
                                }
                            };
                        }
                // macro_rules! menu {
                //             ($text:literal, $block:block) => {
                //                 ui.menu(im_str!($text), true, || $block);
                //             };
                //         }
                // let window_title_color  = ui.push_style_color(StyleColor::Text, [1.0, 0.0, 0.8, 1.0]);

                // let check_mark_color = ui.push_style_color(StyleColor::CheckMark, [1.0, 0.6, 0.74, 1.0]);
                // let slider_grab_color = ui.push_style_color(StyleColor::SliderGrab, [0.4, 0.6, 0.74, 1.0]);
                // let text_color = ui.push_style_color(StyleColor::Text, [0.75, 0.3, 0.02, 0.9]);
                // window_title_color.pop(&ui);
                // let mut settings = settings.lock().unwrap();
                // let window_title_color  = ui.push_style_color(StyleColor::Text, [1.0, 0.0, 0.8, 1.0]);

                window!("Config", (150.0, 150.0) : {

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
                window!("Aimbot", (250.0, 250.0) : {
                            // window_title_color.pop(&ui);
                            ui.checkbox(im_str!("Включен"), &mut settings.aimbot_enabled);
                            ui.checkbox(im_str!("Анти Отдача"), &mut settings.aimbot_rctl);
                            hover!("Включить/Выключить антиотдачу в аимботе");
                            Slider::new(im_str!("Угол")).range(1.0..=180.0).build(&ui, &mut settings.aimbot_angle);
                            hover!("Максимальный угол между вами и противником");
                            Slider::new(im_str!("Дистанция")).range(1..=250).build(&ui, &mut settings.aimbot_distance);
                            hover!("Максимальная дистанция, на которой будет работать аимбот");
                            ui.separator();
                            ui.checkbox(im_str!("Аим Ассист"), &mut settings.aim_assist_enabled);
                            hover!("Доводит прицел до цели, если выбранная часть тела находится в указанном радиусе");
                            Slider::new(im_str!("Радиус")).range(1..=10).build(&ui, &mut settings.aim_assist_angle);
                            let selected_name = match settings.aim_target {
                                8 => "Голова",
                                5 => "Тело",
                                37 => "Рука",
                                _ => "Не выбрано"

                            };

                            ComboBox::new(im_str!("Кость")).preview_mode(ComboBoxPreviewMode::Full).preview_value(ImStr::from_cstr_unchecked(CString::new(selected_name).unwrap().as_c_str())).build(&ui, || {
                                if Selectable::new(im_str!("Голова")).selected(settings.aim_target == 8).build(&ui) {
                                     settings.aim_target = 8;
                                }
                                if Selectable::new(im_str!("Тело")).selected(settings.aim_target == 5).build(&ui) {
                                      settings.aim_target = 5;
                                }
                                if Selectable::new(im_str!("Рука")).selected(settings.aim_target == 37).build(&ui) {
                                      settings.aim_target = 37;
                                }

                            });


                        });


                window!("Misc", (250.0, 250.0) : {
                            // window_title_color.pop(&ui);
                            ui.checkbox(im_str!("FOV"), &mut settings.fov_enabled);
                            ui.separator();

                            Slider::new(im_str!("Смена поля зрения")).range(-180..=180).build(&ui, &mut settings.fov);
                            hover!("ВАЖНО! При отрицательных значениях голова будет перевернута");

                            ui.checkbox(im_str!("RCS"), &mut settings.recoil_enabled);
                            ui.separator();
                            Slider::new(im_str!("Выстрел")).range(1..=100).build(&ui, &mut settings.recoil_shots);
                            hover!("Выстрел, после которого нужно начинать гасить отдачу");

                            ui.checkbox(im_str!("FastTap"), &mut settings.fast_tap_enabled);
                            ui.separator();
                            hover!("Быстрые выстрелы, зажав ЛКМ")


                        });
                // let window_title_color  = ui.push_style_color(StyleColor::Text, [1.0, 0.0, 0.8, 1.0]);

                // let window_title_color  = ui.push_style_color(StyleColor::Text, [1.0, 0.0, 0.8, 1.0]);
                window!("Trigger bot", (250.0, 250.0) : {
                        // window_title_color.pop(&ui);

                        ui.checkbox(im_str!("Включен"), &mut settings.trigger_enabled);
                        ui.checkbox(im_str!("Стрелять только прицелившись"), &mut settings.trigger_only_in_scope);
                        hover!("Триггер бот не будет срабатывать если у Вас в руках снайперская винтовка и Вы не целитесь (чтобы избежать случайных выстрелов)");
                        ui.separator();
                        Slider::new(im_str!("Задержка (ms)")).range(0..=1000).build(&ui, &mut settings.trigger_delay);
                        hover!("Задержка перед выстрелом");
                        Slider::new(im_str!("Дистанция")).range(0..=250).build(&ui, &mut settings.trigger_distance);
                        hover!("Максимальная дистанция, на которой будет работать триггер");
                        ui.separator();
                    });
                // let window_title_color  = ui.push_style_color(StyleColor::Text, [1.0, 0.0, 0.8, 1.0]);
                window!("WallHack", (250.0, 250.0) : {
                        // window_title_color.pop(&ui);
                        ui.columns(2, im_str!("###wh"), true);
                        ColorEdit::new(im_str!("Цвет врага"), EditableColor::Float4(&mut settings.wh_enemy_color))
                            .tooltip(false)
                            .alpha_bar(false)
                            .display_mode(ColorEditDisplayMode::RGB)
                            .input_mode(ColorEditInputMode::RGB)
                            .inputs(false)
                            .alpha(true)
                            .build(&ui);
                        ColorEdit::new(im_str!("Цвет союзника"), EditableColor::Float4(&mut settings.wh_local_color))
                            .tooltip(false)
                            .alpha_bar(false)
                            .display_mode(ColorEditDisplayMode::RGB)
                            .input_mode(ColorEditInputMode::RGB)
                            .inputs(false)
                            .alpha(true)
                            .build(&ui);
                        ColorEdit::new(im_str!("Цвет неуязвимого"), EditableColor::Float4(&mut settings.wh_inactive_color))
                            .tooltip(false)
                            .alpha_bar(false)
                            .display_mode(ColorEditDisplayMode::RGB)
                            .input_mode(ColorEditInputMode::RGB)
                            .inputs(false)
                            .alpha(true)
                            .build(&ui);
                        ui.next_column();
                        ui.checkbox(im_str!("Полное заполнение"), &mut settings.wh_full_bloom);
                        ui.checkbox(im_str!("Включен"), &mut settings.wh_enabled);
;
                    });
                // let window_title_color  = ui.push_style_color(StyleColor::Text, [1.0, 0.0, 0.8, 1.0]);
                window!("Anti Flash", (250.0, 250.0) : {
                        // window_title_color.pop(&ui);
                        ui.checkbox(im_str!("Включен"), &mut settings.flash_enabled);
                        ui.separator();
                    });
                // let window_title_color  = ui.push_style_color(StyleColor::Text, [1.0, 0.0, 0.8, 1.0]);
                window!("Bhop", (250.0, 250.0) : {
                        // window_title_color.pop(&ui);
                        ui.checkbox(im_str!("Включен"), &mut settings.bhop_enabled);
                        ui.separator();
                    });
                // let window_title_color  = ui.push_style_color(StyleColor::Text, [1.0, 0.0, 0.8, 1.0]);
                window!("Radar", (250.0, 250.0) : {
                        // window_title_color.pop(&ui);
                        ui.checkbox(im_str!("Включен"), &mut settings.radar_enabled);
                        ui.separator();
                        let draw_list = ui.get_window_draw_list();
                        let win_pos: Vector2<f32> = Vector2::new(*ui.window_pos().get(0).unwrap(), *ui.window_pos().get(1).unwrap());
                        let win_size: Vector2<f32> = Vector2::new(*ui.window_size().get(0).unwrap(), *ui.window_size().get(1).unwrap());
                        draw_list.add_line(
                                       [win_pos.x + win_size.x * 0.5, win_pos.y],
                                       [win_pos.x + win_size.x * 0.5, win_pos.y + win_size.y],
                                       ImColor::from([70.0/255.0, 70.0/255.0, 70.0/255.0, 1.0]))
                                       .build();
                        draw_list.add_line(
                                       [win_pos.x, win_pos.y + win_size.y * 0.5],
                                       [win_pos.x + win_size.x, win_pos.y + win_size.y * 0.5],
                                       ImColor::from([70.0/255.0, 70.0/255.0, 70.0/255.0, 1.0]))
                                       .build();
                        draw_list.add_line(
                                       [win_pos.x + win_size.x * 0.5, win_pos.y + win_size.y * 0.5],
                                       [win_pos.x, win_pos.y],
                                       ImColor::from([90.0/255.0, 90.0/255.0, 90.0/255.0, 1.0]))
                                       .build();
                        draw_list.add_line(
                                       [win_pos.x + win_size.x * 0.5, win_pos.y + win_size.y * 0.5],
                                       [win_pos.x + win_size.x, win_pos.y],
                                       ImColor::from([90.0/255.0, 90.0/255.0, 90.0/255.0, 1.0]))
                                       .build();

                        draw_list.add_circle([win_pos.x + win_size.x * 0.5, win_pos.y + win_size.y * 0.5], 4.5, ImColor::from([1.0, 1.0, 1.0, 1.0])).build();


                });
                // slider_grab_color.pop(&ui);
                // check_mark_color.pop(&ui);
                // text_color.pop(&ui);
            });
            sleep(Duration::from_millis(10));
        });
    }
}




