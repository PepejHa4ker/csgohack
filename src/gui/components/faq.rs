use crate::gui::components::Component;
use eframe::egui;
use eframe::egui::*;
use crate::settings::Settings;
use crate::gui::app::App;

#[derive(Default)]
pub struct FaqComponent {}

impl Component for FaqComponent {
    fn render(&mut self, context: &egui::CtxRef, ui: &mut Ui, settings: &mut Settings, add_contents: impl FnOnce(&mut Ui)) {
        menu::menu(ui, "FAQ", |ui|{
            hyperlink_button("Github sources", "https://github.com/PepejHa4ker/csgohack", ui);
            hyperlink_button("Developer", "https://vk.com/atleastnotbad", ui);

        });
    }
}

fn hyperlink_button(text: &str, link: &str, ui: &mut Ui) {
    if ui.small_button(text).clicked() {
        ui.output().open_url(link)
    }
}