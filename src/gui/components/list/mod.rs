pub mod providers;
pub mod render;

use egui::{vec2, Button, Image, Ui, Widget};
use serde::{Deserialize, Serialize};

use std::process::Command;
use std::rc::Rc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct ListEntry<'a> {
    pub display_title: String,
    // for search, defaults to `display_title` if None
    pub filter_string: Option<String>,
    pub icon: Option<Image<'a>>,
    // can be hidden by filter?
    pub omnipresent: bool,
    pub click_callback: Rc<Mutex<Command>>,
    pub exit_after_selection: bool,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub enum ListEvent {
    #[default]
    Once,
    Poll, // TODO: implement different times
    SearchboxUpdate,
}

impl Widget for ListEntry<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let button = ui.add(
            Button::opt_image_and_text(self.icon, Some(self.display_title.into()))
                .min_size(vec2(ui.available_width(), 0.)),
        );

        if button.clicked() {
            self.click_callback
                .lock()
                .expect("Command should be acquirable")
                .spawn()
                .expect("Command should not error");
            if self.exit_after_selection {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }

        button
    }
}

impl<'a> ListEntry<'a> {
    #[allow(unused_variables)]
    pub fn matches_filter(&self, key: &str) -> bool {
        self.omnipresent || true // TODO: implement real filtering
    }
}
