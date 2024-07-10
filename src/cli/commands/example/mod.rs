pub mod full;
pub mod runner;



pub fn example(mut args: std::env::Args) {
    let example = args.next().unwrap_or_else(|| String::from("runner"));
    match example.as_str() {
        "runner" => runner::runner(args),
        "full" => full::full(args),
        _ => eprintln!(
            "Unrecognized example {}. See `erun help` for more details.",
            example
        ),
    }
    std::process::exit(0)
}


