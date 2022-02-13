use std::env;

mod util;
use util::command::Command;

mod commands;
use commands::*;

fn main() {
	let args: Vec<String> = env::args().collect();

    let command = Command::parse_from(args);

    match command.name.as_str() {
        "cat" => cat::cat(command),
        _ => println!("Command not found: {}", command.name)
    }
}

