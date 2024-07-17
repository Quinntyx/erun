pub mod commands;
pub use commands::*;

use strum::EnumString;

#[derive(EnumString)]
pub enum FileFormat {
    #[strum(serialize = "infer")]
    Infer,
    #[strum(serialize = "ron")]
    Ron,
    #[strum(serialize = "kdl")]
    Kdl,
    #[strum(serialize = "json")]
    Json,
    // hjson,
    #[strum(serialize = "yaml")]
    Yaml,
    #[strum(serialize = "toml")]
    Toml,
    #[strum(serialize = "sexpr")]
    SExpression,
    #[strum(serialize = "url")]
    Url,
    // TODO: implement csv support (?)
    // #[strum(serialize = "csv")]
    // Csv,
    #[strum(serialize = "xml")]
    Xml,
}
use ron::extensions::Extensions as ex;

use crate::cli::FileFormat;
use crate::gui::components::list::providers::ListContentProvider::*;
use crate::gui::components::Component::*;
use crate::gui::windows::Window;

pub fn runner(mut args: std::env::Args) {
    use FileFormat::*;
    let window = &Window {
        content: Some(List {
            content: Some(Applications),
            show_icons: None,
            exit_after_selection: None,
            run_command: None,
            extra_arguments: vec![],
            bridge_stdin: None,
            bridge_stdout: None,
            bridge_stderr: None,
        }),
        ..Default::default()
    };

    // TODO: look at ~/.config and . to determine most likely preferred format
    let next = FileFormat::try_from(args.next().unwrap_or(String::from("ron")).as_ref())
        .expect("Format should be one of `ron`, `kdl`.");

    (match next {
        Infer => unreachable!(),
        Ron => ron,
        Kdl => kdl,
        Json => json,
        Yaml => yaml,
        Toml => toml,
        SExpression => sexpression,
        Url => url,
        Xml => xml,
        // Csv => csv,
    })(window);
}

fn ron(window: &Window) {
    let runner_string = ron::Options::default()
        .with_default_extension(ex::IMPLICIT_SOME)
        .to_string_pretty(
            window,
            ron::ser::PrettyConfig::new().indentor(String::from("    ")),
        )
        .unwrap();

    println!("{}", runner_string);
}

#[allow(unused_variables)]
fn kdl(window: &Window) {
    todo!("KDL support has not yet been implemented for this example");
}

fn json(window: &Window) {
    let runner_string = serde_json::to_string_pretty(window).unwrap();
    println!("{}", runner_string);
}

fn yaml(window: &Window) {
    let runner_string = serde_yml::to_string(window).unwrap();
    println!("{}", runner_string);
}

fn toml(window: &Window) {
    let string = toml::to_string(window).unwrap();
    println!("{}", string);
}

fn sexpression(window: &Window) {
    let string = serde_lexpr::to_string(window).unwrap();
    println!("{}", string);
}

fn url(window: &Window) {
    let string = serde_qs::to_string(window).unwrap();
    println!("{}", string);
}

// fn csv(window: &Window) {
//     let mut wtr = csv::Writer::from_writer(std::io::stdout());
//     wtr.serialize(window).unwrap();
// }

fn xml(window: &Window) {
    let string = crate::utils::format_xml(serde_xml_rs::to_string(window).unwrap());
    println!("{}", string);
}
pub mod frame_picker;
pub mod full;
pub mod runner;

pub fn example(mut args: std::env::Args) {
    let example = args.next().unwrap_or_else(|| String::from("runner"));
    match example.as_str() {
        "runner" => runner::runner(args),
        "full" => full::full(args),
        "frame_picker" => frame_picker::frame_picker(args),
        _ => eprintln!(
            "Unrecognized example {}. See `erun help` for more details.",
            example
        ),
    }
    std::process::exit(0)
}
use ron::extensions::Extensions as ex;

use crate::cli::FileFormat;
use crate::gui::windows::Window;

pub fn frame_picker(mut args: std::env::Args) {
    use FileFormat::*;
    let window = &ron::Options::default()
        .with_default_extension(ex::IMPLICIT_SOME)
        .from_str::<Window>(include_str!("../../../../test_cfg/frame_picker.ron"))
        .unwrap();

    //TODO: Implement the smart, see runner.rs
    (match FileFormat::try_from(args.next().unwrap_or(String::from("ron")).as_ref())
        .expect("File format should be one of `ron`, `kdl`")
    {
        Infer => unreachable!(),
        Ron => ron,
        Kdl => kdl,
        Json => json,
        Yaml => yaml,
        Toml => toml,
        SExpression => sexpression,
        Url => url,
        Xml => xml,
        // Csv => csv,
    })(window);
}

fn ron(window: &Window) {
    let runner_string = ron::Options::default()
        .with_default_extension(ex::IMPLICIT_SOME)
        .to_string_pretty(
            window,
            ron::ser::PrettyConfig::new().indentor(String::from("    ")),
        )
        .unwrap();

    println!("{}", runner_string);
}

#[allow(unused_variables)]
fn kdl(window: &Window) {
    todo!("KDL support has not yet been implemented for this example");
}

fn json(window: &Window) {
    let runner_string = serde_json::to_string_pretty(window).unwrap();
    println!("{}", runner_string);
}

fn yaml(window: &Window) {
    let runner_string = serde_yml::to_string(window).unwrap();
    println!("{}", runner_string);
}

fn toml(window: &Window) {
    let string = toml::to_string(window).unwrap();
    println!("{}", string);
}

fn sexpression(window: &Window) {
    let string = serde_lexpr::to_string(window).unwrap();
    println!("{}", string);
}

fn url(window: &Window) {
    let string = serde_qs::to_string(window).unwrap();
    println!("{}", string);
}

// fn csv(window: &Window) {
//     let mut wtr = csv::Writer::from_writer(std::io::stdout());
//     wtr.serialize(window).unwrap();
// }

fn xml(window: &Window) {
    let string = crate::utils::format_xml(serde_xml_rs::to_string(window).unwrap());
    println!("{}", string);
}
use ron::extensions::Extensions as ex;

use crate::cli::FileFormat;
use crate::gui::config::Style;
use crate::gui::windows::Window;

pub fn full(mut args: std::env::Args) {
    use FileFormat::*;
    let window = &Window {
        style: Some(Style {
            extreme_bg_color: Some(egui::Color32::RED),
            ..Style::default()
        }),
        ..Default::default()
    };

    //TODO: Implement the smart, see runner.rs
    (match FileFormat::try_from(args.next().unwrap_or(String::from("ron")).as_ref())
        .expect("File format should be one of `ron`, `kdl`")
    {
        Infer => unreachable!(),
        Ron => ron,
        Kdl => kdl,
        Json => json,
        Yaml => yaml,
        Toml => toml,
        SExpression => sexpression,
        Url => url,
        Xml => xml,
        // Csv => csv,
    })(window);
}

fn ron(window: &Window) {
    let full_string = ron::Options::default()
        .with_default_extension(ex::IMPLICIT_SOME)
        .to_string_pretty(
            window,
            ron::ser::PrettyConfig::new().indentor(String::from("    ")),
        )
        .unwrap();

    println!("{}", full_string);
}

#[allow(unused_variables)]
fn kdl(window: &Window) {
    todo!("Kdl support has not yet been implemented for this example.");
}

fn json(window: &Window) {
    let full_string = serde_json::to_string_pretty(window).unwrap();
    println!("{}", full_string);
}

fn yaml(window: &Window) {
    let full_string = serde_yml::to_string(window).unwrap();
    println!("{}", full_string);
}

fn toml(window: &Window) {
    let string = toml::to_string(window).unwrap();
    println!("{}", string);
}

fn sexpression(window: &Window) {
    let string = serde_lexpr::to_string(window).unwrap();
    println!("{}", string);
}

fn url(window: &Window) {
    let string = serde_qs::to_string(window).unwrap();
    println!("{}", string);
}
//
// fn csv(window: &Window) {
//     let mut wtr = csv::Writer::from_writer(std::io::stdout());
//     wtr.serialize(window).unwrap();
// }

fn xml(window: &Window) {
    let string = crate::utils::format_xml(serde_xml_rs::to_string(window).unwrap());
    println!("{}", string);
}
pub mod example;
pub mod help;
pub mod open;
pub use example::*;
pub use help::*;
pub use open::*;
pub fn help(mut args: std::env::Args) {
    if let Some(command) = args.next() {
        match command.as_str() {
            "open" => println!(include_str!("../../../text/help_open")),
            "help" => println!(include_str!("../../../text/help_help")),
            "example" => println!(include_str!("../../../text/help_example")),
            err => println!("Unrecognized command {}.", err),
        }
    } else {
        println!(include_str!("../../../text/help_main"));
    }
}
use ron::extensions::Extensions as ex;

use crate::app::App;
use crate::gui::windows::Window;

use crate::cli::FileFormat;

pub fn open(mut args: std::env::Args) {
    use FileFormat::*;
    let window_file = args
        .next()
        .expect("Expected configuration file path. See `erun help` for more details.");

    let mut format = FileFormat::try_from(args.next().unwrap_or(String::from("infer")).as_ref())
        .expect("Expected valid file format. See `erun help open` for a list of valid formats.");

    if matches!(format, Infer) {
        // TODO: real inference

        format = Ron;
    }

    let window = match format {
        Infer => unreachable!(),
        Ron => ron,
        Kdl => kdl,
        Json => json,
        Yaml => yaml,
        Toml => toml,
        SExpression => sexpression,
        Url => url,
        Xml => xml,
        // Csv => csv,
    }(window_file);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            fullscreen: window.fullscreen,
            maximized: window.maximized,
            resizable: window.resizable,
            transparent: window.transparent,
            decorations: window.decorations,
            title_shown: window.title_shown,
            titlebar_buttons_shown: window.titlebar_buttons_shown,
            titlebar_shown: window.titlebar_shown,
            drag_and_drop: window.drag_and_drop,
            taskbar: window.taskbar,
            close_button: window.close_button,
            minimize_button: window.minimize_button,
            maximize_button: window.maximize_button,
            mouse_passthrough: window.mouse_passthrough,
            active: window.active,
            visible: window.visible,
            fullsize_content_view: window.fullsize_content_view,
            ..Default::default()
        },
        // initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native("erun", options, Box::new(|cc| App::setup(cc, window)))
        .expect("App should not crash");
}

fn ron(window_file: String) -> Window {
    ron::Options::default()
        .with_default_extension(ex::IMPLICIT_SOME)
        .from_str(&std::fs::read_to_string(window_file).expect("File should be readable"))
        .expect("Config file should be valid")
}

#[allow(unused_variables)]
fn kdl(window_file: String) -> Window {
    todo!("KDL Support is not yet implemented.");
}

fn json(window_file: String) -> Window {
    serde_json::from_str(&std::fs::read_to_string(window_file).expect("File should be readable"))
        .expect("Config file should be valid")
}

fn yaml(window_file: String) -> Window {
    serde_yml::from_str(&std::fs::read_to_string(window_file).expect("File should be readable"))
        .expect("Config file should be valid")
}

fn toml(window_file: String) -> Window {
    toml::from_str(&std::fs::read_to_string(window_file).expect("File should be readable"))
        .expect("Config file should be valid")
}

fn sexpression(window_file: String) -> Window {
    serde_lexpr::from_str(&std::fs::read_to_string(window_file).expect("File should be readable"))
        .expect("Config file should be valid")
}

fn url(window_file: String) -> Window {
    serde_qs::from_str(&std::fs::read_to_string(window_file).expect("File should be readable"))
        .expect("Config file should be valid")
}

// fn csv(window_file: String) -> Window {
//     csv::Reader::from_path(window_file)
//         .expect("File should be readable")
//         .deserialize()
//         .next()
//         .expect("Should contain an entry")
//         .expect("Config file should be valid")
// }

fn xml(window_file: String) -> Window {
    serde_xml_rs::from_str(&std::fs::read_to_string(window_file).expect("file should be readable"))
        .expect("Config file should be valid")
}
use serde::{Deserialize, Serialize};

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

        // TODO: implement configurable widgets
        let widgets = egui::style::Widgets::default();

        visuals.widgets = widgets;

        let style = egui::Style {
            visuals,
            ..Default::default()
        };
        style
    }
}
pub mod style;
pub use style::*;
pub mod dimensions;
pub use dimensions::*;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum UPos {
    Pixels(f64), // TO BE IMPLEMENTED lmao
    Points(f64),
}

pub struct UPosVisitor;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct UVector2(pub UPos, pub UPos);

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct UVector4(pub UPos, pub UPos, pub UPos, pub UPos);

impl UPos {
    pub fn get(&self) -> f64 {
        use UPos::*;
        match self {
            Pixels(_m) => todo!(),
            Points(m) => {
                if m < &0f64 {
                    panic!("UPos cannot be less than 0")
                }
                m.clone()
            }
        }
    }
}

impl Serialize for UPos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use UPos::*;
        match self {
            Pixels(p) => serializer.serialize_str(&format!("{}px", p)),
            Points(p) => serializer.serialize_str(&format!("{}pt", p)),
        }
    }
}

impl<'de> Deserialize<'de> for UPos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UPosVisitor)
    }
}

impl<'de> Visitor<'de> for UPosVisitor {
    type Value = UPos;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a positive 64-bit floating point number with pixel or point suffix")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        let (value, suffix) = value.split_at(value.len() - 2);
        let float_value = f64::from_str(value).expect("Float should be valid");
        Ok(match suffix {
            "px" => UPos::Pixels(float_value),
            "pt" => UPos::Points(float_value),
            e => panic!("Suffix on Pos value should be `px` or `pt`, not `{}`", e),
        })
    }
}
pub mod signed;
pub mod unsigned;

pub use signed::*;
pub use unsigned::*;

use serde::{Deserialize, Serialize};

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
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum Pos {
    Pixels(f64), // TO BE IMPLEMENTED lmao
    Points(f64),
}

struct PosVisitor;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Vector2(pub Pos, pub Pos);

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Vector4(pub Pos, pub Pos, pub Pos, pub Pos);

impl Pos {}

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

impl<'de> Visitor<'de> for PosVisitor {
    type Value = Pos;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 64-bit floating point number with pixel or point suffix")
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
use egui::Ui;

use crate::gui::components::Component;
use crate::model::context::Context;

pub fn render(c: &Component, app_ctx: &mut Context, ui: &mut Ui) {
    let Component::Frame {
        padding,
        margin,
        content,
    } = c
    else {
        unreachable!()
    };

    egui::Frame::none()
        .inner_margin(padding.clone())
        .outer_margin(margin.clone())
        .show(ui, |ui| {
            content.render(app_ctx, ui);
        });
}
pub mod render;
use egui::Ui;

use crate::gui::components::Component;
use crate::model::context::Context;

#[allow(unused_variables)]
pub fn render(c: &Component, app_ctx: &mut Context, ui: &mut Ui) {
    todo!("Images have not yet been implemented.");
}
pub mod render;
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
use egui::ScrollArea;
use egui::Ui;

use crate::gui::components::list::providers::ListContentProvider;
use crate::gui::components::Component;
use crate::model::context::Context;

pub fn render(c: &Component, app_ctx: &mut Context, ui: &mut Ui) {
    let Component::List {
        show_icons,
        exit_after_selection,
        run_command,
        extra_arguments,
        bridge_stdin,
        bridge_stdout,
        bridge_stderr,
        content,
        // see TODO in mod.rs
        // update_on,
    } = c
    else {
        unreachable!()
    };

    let mut search_key = String::new();

    let _searchbox = ui.add(egui::TextEdit::singleline(&mut search_key));

    ScrollArea::vertical().show(ui, |ui| {
        let mut content = content.clone().unwrap_or(ListContentProvider::Applications);
        let entries = content.provide_contents(
            app_ctx,
            show_icons.unwrap_or(false),
            exit_after_selection.unwrap_or(true),
            run_command.clone().unwrap_or(String::from("open")),
            extra_arguments.clone(),
            bridge_stdin.unwrap_or(false),
            bridge_stdout.unwrap_or(false),
            bridge_stderr.unwrap_or(false),
        );

        let _responses = entries
            .iter()
            .filter(|i| i.matches_filter(&search_key))
            .map(|i| ui.add((*i).clone()))
            .collect::<Vec<_>>();
    });
}

/*
ListContent=> {
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

*/
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::sync::Mutex;

use crate::gui::components::list::providers::ListContentProvider;
use crate::gui::components::list::ListEntry;
use crate::model::context::Context;
use crate::utils::load_icon_to_uri;

pub fn provide_contents<'a>(
    p: &mut ListContentProvider,
    app_ctx: &mut Context,
    show_icons: bool,
    exit_after_selection: bool,
    run_command: String,
    extra_arguments: Vec<String>,
    bridge_stdin: bool,
    bridge_stdout: bool,
    bridge_stderr: bool,
) -> Vec<ListEntry<'a>> {
    match p {
        ListContentProvider::Applications => (),
        _ => unreachable!(),
    }

    app_ctx
        .apps
        .iter()
        .map(|app| {
            let mut cmd = Command::new(&run_command);
            cmd.arg(app.app_desktop_path.clone())
                .args(extra_arguments.clone());

            if !bridge_stdin {
                cmd.stdin(Stdio::null());
            }
            if !bridge_stdout {
                cmd.stdout(Stdio::null());
            }
            if !bridge_stderr {
                cmd.stderr(Stdio::null());
            }

            let mut icon = None;

            if show_icons {
                icon = app
                    .icon_path
                    .clone()
                    .map(|i| Some(egui::Image::from_uri(load_icon_to_uri(i).ok()?)))
                    .flatten();
            }

            ListEntry {
                display_title: app.name.clone(),
                filter_string: None,
                icon,
                exit_after_selection,
                omnipresent: false,
                click_callback: Rc::new(Mutex::new(cmd)),
            }
        })
        .collect()
}

pub mod applications;
pub mod command;

use serde::{Deserialize, Serialize};

use crate::gui::components::list::ListEntry;
use crate::model::context::Context;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub enum ListContentProvider {
    #[default]
    Applications,
    Command {
        // TODO: Implement
        #[serde(default)]
        pass_filter_content: Option<bool>,
        #[serde(default)]
        filter_results: Option<bool>,
    },
}

impl ListContentProvider {
    pub fn provide_contents(
        &mut self,
        app_ctx: &mut Context,
        show_icons: bool,
        exit_after_selection: bool,
        run_command: String,
        extra_arguments: Vec<String>,
        bridge_stdin: bool,
        bridge_stdout: bool,
        bridge_stderr: bool,
    ) -> Vec<ListEntry> {
        use ListContentProvider::*;
        match self {
            Applications => applications::provide_contents(
                self,
                app_ctx,
                show_icons,
                exit_after_selection,
                run_command,
                extra_arguments,
                bridge_stdin,
                bridge_stdout,
                bridge_stderr,
            ),
            Command { .. } => todo!("Implement ListContentProvider::Command"),
        }
    }

    pub fn update(&mut self) {}
}
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
pub mod components;
pub mod config;
pub mod primitives;
pub mod windows;
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
pub mod context;
use applications::{App, AppInfo, AppInfoContext};

pub struct Context {
    pub apps: Vec<App>,
}

impl Context {
    pub fn new() -> Self {
        let mut ctx = AppInfoContext::new();
        ctx.refresh_apps().unwrap();
        let apps = ctx.get_all_apps();
        Self { apps }
    }

    pub fn update(&mut self) -> Result<(), ()> {
        Ok(())
    }
}
pub mod app;
pub mod cli;
pub mod gui;
pub mod model;
pub mod utils;

use cli::*;

fn main() -> Result<(), eframe::Error> {
    let mut args = std::env::args();
    args.next().expect("This should not crash here...");

    let Some(command) = args.next() else {
        help(args);
        return Ok(());
    };

    match command.as_str() {
        "open" => open(args),
        "help" => help(args),
        "example" => example(args),
        err => print!("Unrecognized command {}.", err),
    }

    Ok(())
}
use eframe::CreationContext;

use crate::gui::windows::Window;
use crate::model::context::Context;

/// The main application struct for `erun`. 
///
/// * `ctx`: The application context. 
/// * `layout`: The loaded Window layout currently being displayed in this window. 
pub struct App {
    ctx: Context,
    layout: Window,
}

impl App {
    /// Constructor. 
    ///
    /// * `layout`: The layout of the window to construct the App with. 
    pub fn new(layout: Window) -> Self {
        Self {
            ctx: Context::new(),
            layout,
        }
    }

    /// Setup function. Called in main and passed to eframe::run_native. 
    ///
    /// * `cc`: 
    /// * `window`: 
    pub fn setup(cc: &CreationContext, window: Window) -> Box<dyn eframe::App> {
        if let Some(style) = &window.style {
            let estyle: egui::Style = style.clone().into();
            if let Some(zoom) = &window.zoom_factor {
                cc.egui_ctx.set_zoom_factor(*zoom);
            }
            cc.egui_ctx.set_style(std::sync::Arc::new(estyle));
        }

        Box::new(Self::new(window))
    }
}

impl eframe::App for App {
    /// Implementation of update. 
    ///
    /// * `ctx`: egui Context. 
    /// * `_frame`: eframe Frame. Unused. 
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let app_ctx = &mut self.ctx;

        app_ctx.update().expect("Update should not fail");

        if let Some(title) = &self.layout.title {
            egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
                ui.label(title);
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.layout
                .content
                .clone()
                .expect("Root window should have content")
                .render(app_ctx, ui);
        });
    }
}
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use icns::IconFamily;

#[allow(unreachable_code, unused_variables)]
pub fn load_icon_to_uri(path: PathBuf) -> Result<String, anyhow::Error> {
    return Ok("".into());
    if path.extension().is_some_and(|x| x == "icns") {
        let family = IconFamily::read(BufReader::new(File::open(path)?))?;
        let icon = family.get_icon_with_type(family.available_icons()[0])?;
        Ok(format!(
            "bytes://{}",
            icon.into_data()
                .iter()
                .map(|i| i.to_string())
                .collect::<String>()
        ))
    } else {
        Ok(format!(
            "file://{}",
            path.into_os_string()
                .into_string()
                .expect("Path should be valid")
        ))
    }
}

use xml::{reader::ParserConfig, writer::EmitterConfig};

pub fn format_xml(src: String) -> String {
    let mut dest = Vec::new();
    let reader = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(false)
        .create_reader(src.as_bytes());
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .normalize_empty_elements(false)
        .autopad_comments(false)
        .create_writer(&mut dest);
    for event in reader {
        if let Some(event) = event.unwrap().as_writer_event() {
            writer.write(event).unwrap();
        }
    }
    String::from_utf8(dest).unwrap()
}
