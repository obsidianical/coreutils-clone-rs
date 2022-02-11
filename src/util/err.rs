use std::process;

pub fn err_out(reason: String, exit_code: i32) -> ! {
    println!("Error: '{}'", reason);
    process::exit(exit_code);
}

