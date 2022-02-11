use std::env;
use std::process;
use std::fs;
use std::io;

fn main() {
	let args: Vec<String> = env::args().collect();

    let command = Command::parse_from(args);

    match command.name.as_str() {
        "cat" => cat(command),
        _ => println!("Command not found.")
    }
}

fn cat(cmd: Command) {
    let paths = if cmd.get_params().len() == 0 {
        stdin_to_stdout_loop();
    } else {
        cmd.get_params()
    };

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

fn err_out(reason: String, exit_code: i32) -> ! {
    println!("Error: '{}'", reason);
    process::exit(exit_code);
}

#[derive(Debug)]
struct CommandParam {
    value: String,
    is_flag: bool
}

#[derive(Debug)]
struct Command {
    name: String,
    args: Vec<CommandParam>,
    executable_path: String,
}

impl Command {
    pub fn parse_from(arglist: Vec<String>) -> Command {
        let mut command = Command {
            name: match arglist.get(1) {
                Some(v) => v.to_string(),
                None => err_out(String::from("No command provided!"), 1),
            },
            args: vec![],
            executable_path: arglist.get(0).unwrap().to_string(),
        };

        for arg in &arglist[2..arglist.len()] {
            if arg.chars().nth(0).unwrap() == '-' && arg.len() != 1 {
                if arg.chars().nth(1).unwrap() == '-' {
                    command.args.push(CommandParam { 
                        value: String::from(&arg[2..arg.len()]),
                        is_flag: true
                    });
                } else {
                    for i in 1..arg.len() {
                        command.args.push(CommandParam {
                            value: arg.chars().nth(i).unwrap().to_string(),
                            is_flag: true
                        })
                    }
                }
            } else {
                command.args.push(CommandParam {
                    value: arg.to_string(),
                    is_flag: false
                });
            }
        }

        command
    }

    fn get_args(&self, get_flags: bool) -> Vec<&CommandParam> {
        let mut res: Vec<&CommandParam> = vec![];

        for arg in &self.args {
            if arg.is_flag == get_flags {
                res.push(arg);
            }
        }

        res
    }

    pub fn get_flags(&self) -> Vec<&CommandParam> {
        self.get_args(true)
    }
    pub fn get_params(&self) -> Vec<&CommandParam> {
        self.get_args(false)
    }
}

