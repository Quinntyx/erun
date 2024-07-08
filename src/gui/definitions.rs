use egui::{vec2, Button, Ui};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::process::{exit, Command, Stdio};
use std::str::FromStr;

use crate::model::context::Context;
use crate::utils::load_icon_to_uri;

#[derive(Copy, Clone, Debug)]
pub enum Pos {
    Pixels(f64), // TO BE IMPLEMENTED lmao
    Points(f64),
}

impl Serialize for Pos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use Pos::*;
        match self {
            Pixels(p) => serializer.serialize_str(&format!("{}px", p)),
            Points(p) => serializer.serialize_str(&format!("{}pt", p)),
        }
    }
}

impl<'de> Deserialize<'de> for Pos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PosVisitor)
    }
}

struct PosVisitor;

impl<'de> Visitor<'de> for PosVisitor {
    type Value = Pos;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 32-bit floating point number with pixel or point suffix")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        let (value, suffix) = value.split_at(value.len() - 2);
        let float_value = f64::from_str(value).expect("Float should be valid");
        Ok(match suffix {
            "px" => Pos::Pixels(float_value),
            "pt" => Pos::Points(float_value),
            e => panic!("Suffix on Pos value should be `px` or `pt`, not `{}`", e),
        })
    }
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

#[derive(Serialize, Deserialize, Copy, Clone, Default, Debug)]
pub enum ComboSize {
    #[default]
    ZERO,
    Even(UPos),
    Symmetric(UPos, UPos),
    PerEdge(UPos, UPos, UPos, UPos),
}

impl Into<egui::Margin> for ComboSize {
    fn into(self) -> egui::Margin {
        use egui::Margin as Em;
        use ComboSize::*;
        match self {
            ZERO => Em::same(0f32),
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

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Style {
    #[serde(default)]
    pub faint_bg_color: Option<egui::Color32>,
    #[serde(default)]
    pub code_bg_color: Option<egui::Color32>,
    #[serde(default)]
    pub extreme_bg_color: Option<egui::Color32>,
    #[serde(default)]
    pub warn_fg_color: Option<egui::Color32>,
    #[serde(default)]
    pub error_fg_color: Option<egui::Color32>,
    #[serde(default)]
    pub window_rounding: Option<egui::Rounding>,
    #[serde(default)]
    pub panel_fill: Option<egui::Color32>,
    #[serde(default)]
    pub zoom_factor: Option<f32>,
}

impl Into<egui::Style> for Style {
    fn into(self) -> egui::Style {
        let mut visuals = egui::Visuals::default();
        visuals.faint_bg_color = self.faint_bg_color.unwrap_or(visuals.faint_bg_color);
        visuals.code_bg_color = self.code_bg_color.unwrap_or(visuals.code_bg_color);
        visuals.extreme_bg_color = self.extreme_bg_color.unwrap_or(visuals.extreme_bg_color);
        visuals.warn_fg_color = self.warn_fg_color.unwrap_or(visuals.warn_fg_color);
        visuals.error_fg_color = self.error_fg_color.unwrap_or(visuals.error_fg_color);
        visuals.window_rounding = self.window_rounding.unwrap_or(visuals.window_rounding);
        visuals.panel_fill = self.panel_fill.unwrap_or(visuals.panel_fill);

        let mut widgets = egui::style::Widgets::default();

        let style = egui::Style {
            visuals,
            ..Default::default()
        };
        style
    }
}

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

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub enum Event {
    #[default]
    Once,
    Poll, // TODO: implement different times
    SearchUpdate,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub enum ListProvider {
    #[default]
    Applications,
    Command {
        // TODO: Implement
        #[serde(default)]
        pass_filter_content: Option<bool>,
        #[serde(default)]
        filter_results: Option<bool>,
        #[serde(default)]
        update_on: Option<Event>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Component {
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
        provider: Option<ListProvider>,
    },
}

impl Component {
    pub fn render(&self, app_ctx: &mut Context, ui: &mut Ui) {
        use Component::*;
        match self {
            Frame {
                padding,
                margin,
                content,
            } => {
                egui::Frame::none()
                    .inner_margin(padding.clone())
                    .outer_margin(margin.clone())
                    .show(ui, |ui| {
                        content.render(app_ctx, ui);
                    });
            }
            Image { uri: _ } => todo!(),
            List {
                show_icons,
                exit_after_selection,
                run_command,
                extra_arguments,
                bridge_stdin,
                bridge_stdout,
                bridge_stderr,
                provider,
            } => match provider.clone().unwrap_or(ListProvider::Applications) {
                ListProvider::Applications => {
                    for app in &mut app_ctx.apps {
                        let button = ui.add(
                            Button::opt_image_and_text(
                                if show_icons.is_some_and(|i| i) {
                                    app.icon_path
                                        .clone()
                                        .map(|i| {
                                            Some(egui::Image::from_uri(load_icon_to_uri(i).ok()?))
                                        })
                                        .flatten()
                                } else {
                                    None
                                },
                                Some(app.name.clone().into()),
                            )
                            .min_size(vec2(ui.available_width(), 0.)),
                        );
                        if button.clicked() {
                            let mut cmd =
                                Command::new(run_command.clone().unwrap_or("open".to_string()));
                            cmd.arg(app.app_desktop_path.clone()).args(extra_arguments);

                            if bridge_stdin.unwrap_or(false) {
                                cmd.stdin(Stdio::null());
                            }
                            if bridge_stdout.unwrap_or(false) {
                                cmd.stdout(Stdio::null());
                            }
                            if bridge_stderr.unwrap_or(false) {
                                cmd.stderr(Stdio::null());
                            }

                            cmd.spawn().unwrap();
                            if exit_after_selection.unwrap_or(true) {
                                println!("app exiting");
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    }
                }
                ListProvider::Command {
                    pass_filter_content,
                    filter_results,
                    update_on,
                } => todo!("Implement commands"),
            },
        }
    }
}
