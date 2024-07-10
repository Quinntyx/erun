use ron::extensions::Extensions as ex;

use crate::gui::components::list::providers::ListContentProvider::*;
use crate::gui::components::Component::*;
use crate::gui::config::Style;
use crate::gui::windows::Window;

pub fn example(mut args: std::env::Args) {
    let example = args.next().unwrap_or_else(|| String::from("runner"));
    match example.as_str() {
        "runner" => runner(),
        "full" => full(),
        _ => eprintln!(
            "Unrecognized example {}. See `erun help` for more details.",
            example
        ),
    }
    std::process::exit(0)
}

fn runner() {
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
        )
        .unwrap();

    println!("{}", runner_string);
}

fn full() {
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

    println!("{}", full_string);
}
