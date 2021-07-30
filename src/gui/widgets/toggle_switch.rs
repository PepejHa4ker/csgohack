use eframe::egui;
use egui::*;

pub struct ToggleSwitch<'a> {
    text: String,
    value: &'a mut bool
}

impl<'a> ToggleSwitch<'a> {

    pub fn text(mut self, text: impl ToString) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn new(value: &'a mut bool, text: impl ToString) -> Self {
        ToggleSwitch {
            text: text.to_string(),
            value
        }
    }
}

impl<'a> Widget for ToggleSwitch<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal_wrapped(|ui | {
            let ToggleSwitch {
                text,
                value
            } = self;
            let desired_size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
            let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
            if response.clicked() {
                *value = !*value;
                response.mark_changed();
            }
            response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, *value, ""));
            let how_on = ui.ctx().animate_bool(response.id, *value);
            let visuals = ui.style().interact_selectable(&response, *value);
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter().rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
            let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = pos2(circle_x, rect.center().y);
            ui.painter().circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
            ui.label(text);
            response
        }).inner
    }
}


