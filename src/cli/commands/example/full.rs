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
