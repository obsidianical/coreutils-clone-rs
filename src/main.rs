use std::env;
use std::process;
use std::fs;
use std::io;

fn main() {
	let args: Vec<String> = env::args().collect();

    let command = Command::parse_from(args);

	//match args.get(1).unwrap().as_str() {
		//"cat" => cat(args),
		//_ => println!("Command not found.")
	//}
}

fn cat(args: Vec<String>) {
	let path = match args.get(2) {
		Some(v) => v,
		None => stdin_to_stdout_loop()
	};

    if path == "-" {
        stdin_to_stdout_loop();
    }
	
	println!("{}", match fs::read_to_string(path) {
		Ok(v) => v,
		Err(v) => {
			println!("Error when reading file: {}", v); 
			process::exit(1);
		},
	});
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
            if arg.chars().nth(0).unwrap() == '-' {
                if arg.chars().nth(1).unwrap() == '-' {
                    command.args.push(CommandParam { 
                        value: String::from(&arg[2..arg.len()]),
                        is_flag: true
                    });
                } else {
                    command.args.push(CommandParam { 
                        value: String::from(&arg[1..arg.len()]),
                        is_flag: true
                    });
                }
            } else {
                command.args.push(CommandParam {
                    value: arg.to_string(),
                    is_flag: false
                });
            }
        }
        println!("{:?}", command);
        command
    }
}

