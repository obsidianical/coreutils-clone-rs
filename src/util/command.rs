use super::arg::{ Argument, ArgType };
use super::err::err_out;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<Argument>,
    pub executable_path: String,
}

impl Command {
    pub fn parse_from(args: Vec<String>) -> Command {
        Command {
            name: Command::find_name(&args),
            args: Command::parse_args(&args),
            executable_path: args.get(0).unwrap().to_string()
        }
    }

    fn find_name(args: &[String]) -> String {
        let r = String::from(match args.get(1) {
            Some(v) => v,
            None => err_out(String::from("No command provided!"), 1),
        });

        r
    }

    fn parse_args(args: &[String]) -> Vec<Argument> {
        let mut r: Vec<Argument> = vec![];

        for arg in &args[2..] {
            r.push(Argument::new(arg.to_string()));
        }

        r
    }

    pub fn flag_passed(&self, short_form: &str, long_form: &str) -> bool {
        let r = self.args.iter().any(|arg| {
            if arg.arg_type == ArgType::Param {
                return false;
            }

            arg.check_flag(short_form, long_form)
        });

        r
    }

    pub fn get_args(&self, filter: ArgType) -> Vec<&Argument> {
        let mut r: Vec<&Argument> = vec![];

        for arg in &self.args {
            if arg.arg_type == filter {
                r.push(arg);
            }
        }

        r
    }
}
