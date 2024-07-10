pub mod frame;
pub mod image;
pub mod list;

use egui::Ui;
use serde::{Deserialize, Serialize};
// TODO: implement kdl support
// use knuffel::Decode;

use crate::gui::primitives::dimensions::*;
use crate::model::context::Context;
use list::providers::ListContentProvider;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
// #[derive(Decode)] // Knuffel, for KDL
pub enum Component {
    #[default]
    Empty,
    Frame {
        #[serde(default)]
        padding: ComboSize,
        #[serde(default)]
        margin: ComboSize,
        content: Box<Component>,
    },
    Image {
        uri: String,
    },
    List {
        #[serde(default)]
        show_icons: Option<bool>,
        #[serde(default)]
        exit_after_selection: Option<bool>,
        #[serde(default)]
        run_command: Option<String>,
        #[serde(default)]
        extra_arguments: Vec<String>,
        #[serde(default)]
        bridge_stdin: Option<bool>,
        #[serde(default)]
        bridge_stdout: Option<bool>,
        #[serde(default)]
        bridge_stderr: Option<bool>,
        #[serde(default)]
        content: Option<ListContentProvider>,
        // TODO: Implement updating
        // #[serde(default)]
        // update_on: Option<ListEvent>,
    },
}

impl Component {
    pub fn render(&self, app_ctx: &mut Context, ui: &mut Ui) {
        use Component::*;
        match self {
            Empty => (),
            Frame { .. } => frame::render::render(self, app_ctx, ui),
            Image { .. } => image::render::render(self, app_ctx, ui),
            List { .. } => list::render::render(self, app_ctx, ui),
        }
    }
}
