use egui::{vec2, Button, Ui};
use serde::{Deserialize, Serialize};
use std::process::{exit, Command, Stdio};

use crate::model::context::Context;
use crate::utils::load_icon_to_uri;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Pos {
    Pixels(i64), // TO BE IMPLEMENTED lmao
    Unit(f64),
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum UPos {
    Pixels(u64), // TO BE IMPLEMENTED lmao
    Unit(f64),
}

impl UPos {
    pub fn get(&self) -> f64 {
        use UPos::*;
        match self {
            Pixels(_m) => todo!(),
            Unit(m) => {
                if m < &0f64 {
                    panic!("UPos cannot be less than 0")
                }
                m.clone()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Vector2(pub Pos, pub Pos);

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct UVector2(pub UPos, pub UPos);

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Vector4(pub Pos, pub Pos);

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct UVector4(pub UPos, pub UPos);

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum ComboSize {
    Even(UPos),
    Symmetric(UPos, UPos),
    PerEdge(UPos, UPos, UPos, UPos),
}

impl Into<egui::Margin> for ComboSize {
    fn into(self) -> egui::Margin {
        use egui::Margin as Em;
        use ComboSize::*;
        match self {
            Even(x) => Em::same(x.get() as f32),
            Symmetric(x, y) => Em::symmetric(x.get() as f32, y.get() as f32),
            PerEdge(left, right, top, bottom) => Em {
                left: left.get() as f32,
                right: right.get() as f32,
                top: top.get() as f32,
                bottom: bottom.get() as f32,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Window {
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Component {
    Frame {
        padding: ComboSize,
        margin: ComboSize,
        subcomponent: Box<Component>,
    },
    Image {
        uri: String,
    },
    AppList {
        show_icons: Option<bool>,
        exit_after_selection: Option<bool>,
    },
}

impl Component {
    pub fn render(&self, app_ctx: &mut Context, ui: &mut Ui) {
        use Component::*;
        match self {
            Frame {
                padding,
                margin,
                subcomponent,
            } => {
                egui::Frame::none()
                    .inner_margin(padding.clone())
                    .outer_margin(margin.clone())
                    .show(ui, |ui| {
                        subcomponent.render(app_ctx, ui);
                    });
            }
            Image { uri } => todo!(),
            AppList {
                show_icons,
                exit_after_selection,
            } => {
                for app in &mut app_ctx.apps {
                    let button = ui.add(
                        Button::opt_image_and_text(
                            if show_icons.is_some_and(|i| i) {
                                app.icon_path
                                    .clone()
                                    .map(|i| Some(egui::Image::from_uri(load_icon_to_uri(i).ok()?)))
                                    .flatten()
                            } else {
                                None
                            },
                            Some(app.name.clone().into()),
                        )
                        .min_size(vec2(ui.available_width(), 0.)),
                    );
                    if button.clicked() {
                        Command::new("open")
                            .args([app.app_desktop_path.clone()])
                            .stdin(Stdio::null())
                            .stdout(Stdio::null())
                            .stderr(Stdio::null())
                            .spawn()
                            .unwrap();
                        if exit_after_selection.is_some_and(|i| !i) {
                            exit(0);
                        }
                    }
                }
            }
        }
    }
}
