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
