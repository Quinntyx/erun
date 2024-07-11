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
