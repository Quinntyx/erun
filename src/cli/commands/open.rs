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
