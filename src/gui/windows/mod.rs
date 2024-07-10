use serde::{Deserialize, Serialize};

use crate::gui::components::Component;
use crate::gui::config::Style;
use crate::gui::primitives::{UPos, UVector2, Vector2};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Window {
    #[serde(default)]
    pub fullscreen: Option<bool>,
    #[serde(default)]
    pub maximized: Option<bool>,
    #[serde(default)]
    pub resizable: Option<bool>,
    #[serde(default)]
    pub transparent: Option<bool>,
    #[serde(default)]
    pub decorations: Option<bool>,
    #[serde(default)]
    pub title_shown: Option<bool>,
    #[serde(default)]
    pub titlebar_buttons_shown: Option<bool>,
    #[serde(default)]
    pub titlebar_shown: Option<bool>,
    #[serde(default)]
    pub drag_and_drop: Option<bool>,
    #[serde(default)]
    pub taskbar: Option<bool>,
    #[serde(default)]
    pub close_button: Option<bool>,
    #[serde(default)]
    pub minimize_button: Option<bool>,
    #[serde(default)]
    pub maximize_button: Option<bool>,
    #[serde(default)]
    pub mouse_passthrough: Option<bool>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    pub fullsize_content_view: Option<bool>,
    #[serde(default)]
    pub zoom_factor: Option<f32>,
    #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")] // skip?
    pub title: Option<String>,
    #[serde(default)]
    pub position: Option<Vector2>, // normalize to center of screen
    #[serde(default)]
    pub size: Option<UVector2>,
    // UPos amount to indent from respective side
    #[serde(default)]
    pub top_panel: Option<(Component, UPos)>,
    #[serde(default)]
    pub left_panel: Option<(Component, UPos)>,
    #[serde(default)]
    pub right_panel: Option<(Component, UPos)>,
    #[serde(default)]
    pub bottom_panel: Option<(Component, UPos)>,
    // centerpanel, takes up all space after drawing sides
    #[serde(default)]
    pub content: Option<Component>,
    #[serde(default)]
    pub style: Option<Style>,
}
