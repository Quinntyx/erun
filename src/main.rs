pub mod app;
pub mod gui;
pub mod model;
pub mod utils;

use app::App;
use gui::config::Style;
use gui::windows::Window;
use ron::extensions::Extensions as ex;

fn main() -> Result<(), eframe::Error> {
    let mut args = std::env::args();
    args.next().expect("This should not crash here...");

    let Some(command) = args.next() else {
        help(None)
    };

    match command.as_str() {
        "open" => open(
            args.next()
                .expect("Expected configuration file path. See `erun help` for more details."),
        ),
        "help" => help(args.next()),
        "example" => example(args.next().unwrap_or_else(|| String::from("runner"))),
        err => print!("Unrecognized command {}.", err),
    }

    Ok(())
}

fn help(command: Option<String>) -> ! {
    if let Some(command) = command {
        match command.as_str() {
            "open" => println!(include_str!("../text/help_open")),
            "help" => println!(include_str!("../text/help_help")),
            "example" => println!(include_str!("../text/help_example")),
            err => println!("Unrecognized command {}.", err),
        }
    } else {
        println!(include_str!("../text/help_main"));
    }
    std::process::exit(0)
}

fn example(example: String) {
    use crate::gui::components::Component::*;
    use crate::gui::components::list::providers::ListContentProvider::*;
    let runner_string = "\
Window(
    title: \"erun\",
    content: Frame(
        padding: Even(Unit(5.0)),
        margin: Even(Unit(5.0)),
        subcomponent: AppList(
            show_icons: true,
            exit_after_selection: true,
        ),
    ),
)
";

    let runner_string = ron::Options::default()
        .with_default_extension(ex::IMPLICIT_SOME)
        .to_string_pretty(
            &Window {
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
            },
            ron::ser::PrettyConfig::new().indentor(String::from("    ")),
        ).unwrap();

    let full_string = ron::Options::default()
        .with_default_extension(ex::IMPLICIT_SOME)
        .to_string_pretty(
            &Window {
                style: Some(Style {
                    extreme_bg_color: Some(egui::Color32::RED),
                    ..Style::default()
                }),
                ..Default::default()
            },
            ron::ser::PrettyConfig::new().indentor(String::from("    ")),
        )
        .unwrap();

    match example.as_str() {
        "runner" => print!("{}", runner_string),
        "full" => print!("{}", full_string),
        _ => eprintln!(
            "Unrecognized example {}. See `erun help` for more details.",
            runner_string
        ),
    }
    std::process::exit(0)
}

fn open(window_file: String) {
    let window: Window = ron::Options::default()
        .with_default_extension(ex::IMPLICIT_SOME)
        .from_str(&std::fs::read_to_string(window_file).expect("File should be readable"))
        .expect("Config file should be valid");

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
