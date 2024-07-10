pub fn help(mut args: std::env::Args) {
    if let Some(command) = args.next() {
        match command.as_str() {
            "open" => println!(include_str!("../../../text/help_open")),
            "help" => println!(include_str!("../../../text/help_help")),
            "example" => println!(include_str!("../../../text/help_example")),
            err => println!("Unrecognized command {}.", err),
        }
    } else {
        println!(include_str!("../../../text/help_main"));
    }
}
