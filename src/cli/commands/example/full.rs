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
