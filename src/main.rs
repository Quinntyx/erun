pub mod app;
pub mod gui;
pub mod model;
pub mod utils;

use app::App;
use gui::definitions::Window;
use ron::de;
use ron::extensions::Extensions as ex;

fn main() -> Result<(), eframe::Error> {
    let mut args = std::env::args();
    args.next().expect("This should not crash here...");

    let Some(command) = args.next() else { help(None) };

    match command.as_str() {
        "open" => open(args.next().expect("Expected configuration file path. See `erun help` for more details.")),
        "help" => help(args.next()),
        "example" => example(args.next().unwrap_or_else(|| String::from("runner"))),
        err => print!("Unrecognized command {}.", err),
    }

    Ok(())
}

fn help(command: Option<String>) -> ! {
    let main_string = 
" \
Widget-Based Launcher, Menu, and Bar

Usage: erun [COMMAND] [ARGS]

Commands:
    open [FILE]       Opens a window by loading the specified configuration file
    help [COMMAND?]   Displays the help message for a specific command or this message if unspecified
    example [TYPE?]    Prints an example Window config. Currently available: runner (default)

See `erun help [COMMAND]` for more information on a specific command.
";

    let open_string = "Help string for `open` has not yet been added.";
    let help_string = "Help string for `help` has not yet been added.";
    let example_string = "Help string for `example` has not yet been added.";

    if let Some(command) = command { 
        match command.as_str() {
            "open" => print!("{}", open_string),
            "help" => print!("{}", help_string),
            "example" => print!("{}", example_string),
            err => print!("Unrecognized command {}.", err),
        }
    } else {
        print!("{}", main_string);
    }

    std::process::exit(0)
}

fn example(example: String) {
    let runner_string = "\
title: \"erun\",
content: Frame {
    padding: Even(Unit(5.0)),
    margin: Even(Unit(5.0)),
    subcomponent: AppList {
        show_icons: true,
        exit_after_selection: true,
    },
},
";

    match example.as_str() {
        "runner" => eprintln!("{}", runner_string),
        _ => eprintln!(
            "Unrecognized example {}. See `erun help` for more details.",
            runner_string
        ),
    }
    std::process::exit(0)
}

fn open(window_file: String) {
    let window = ron::Options::default()
        .with_default_extension(ex::IMPLICIT_SOME)
        .from_str(
            &std::fs::read_to_string(window_file).expect("File should be readable"),
        ).expect("Config file should be valid");

    let options = eframe::NativeOptions {
        // initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native("erun", options, Box::new(|cc| App::setup(cc, window))).expect("App should not crash");
}
