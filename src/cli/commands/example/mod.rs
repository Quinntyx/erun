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
