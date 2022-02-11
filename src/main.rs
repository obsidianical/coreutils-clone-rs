use std::env;
use std::fs;
use std::io;

mod util;
use util::command::Command;
use util::arg::ArgType;
use util::err::err_out;

fn main() {
	let args: Vec<String> = env::args().collect();

    let command = Command::parse_from(args);

    match command.name.as_str() {
        "cat" => cat(command),
        _ => println!("Command not found: {}", command.name)
    }
}

fn cat(cmd: Command) {
    let paths = cmd.get_args(ArgType::Param);

    if paths.len() == 0 {
        stdin_to_stdout_loop();
    }

    for path in paths {
        if path.value.as_str() == "-" {
            stdin_to_stdout_loop();
        }

        let file = match fs::read_to_string(path.value.as_str()) {
            Ok(v) => v,
            Err(e) => err_out(format!("Error while reading file: {}", e), 1)
        };

        print!("{}", file);
    }
}

fn stdin_to_stdout_loop() -> ! {
	let stdin = io::stdin();

    loop {
        let mut inpt: String = String::new();

        match stdin.read_line(&mut inpt) {
            Ok(v) => v,
            Err(e) => panic!("Error reading stdin: {:?}", e),
        };

        print!("{}", inpt);
    }
}

