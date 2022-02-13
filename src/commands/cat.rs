use std::io;
use std::fs;

use crate::util::command::Command;
use crate::util::arg::ArgType;
use crate::util::err::err_out;

pub fn cat(cmd: Command) {
    let paths = cmd.get_args(ArgType::Param);

    if paths.is_empty() {
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
