use std::env;
use std::process;
use std::fs;
use std::io;

fn main() {
	let args: Vec<String> = env::args().collect();
	//let command: String = args.get(1).unwrap().clone();
	match args.get(1).unwrap().as_str() {
		"cat" => cat(args),
		_ => println!("Command not found.")
	}
	//println!("{:?}", command);
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
	let mut inpt: String = String::new();
	
    loop {
        match stdin.read_line(&mut inpt) {
            Ok(v) => v,
            Err(e) => panic!("Error reading stdin: {:?}", e),
        };
        print!("{}", inpt);
    }
}
