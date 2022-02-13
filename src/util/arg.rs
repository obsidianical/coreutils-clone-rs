#[derive(Debug)]
pub struct Argument {
    pub arg_type: ArgType,
    pub value: String
}

#[derive(Debug, PartialEq)]
pub enum ArgType {
    Param,
    Group,
    Long
}

impl Argument {
    pub fn new(value: String) -> Argument {
        Argument {
            arg_type: Argument::detect_flag(&value),
            value
        }
    }

    fn detect_flag(arg: &str) -> ArgType {
        let mut arg_chars = arg.chars();

        if arg_chars.next().unwrap() == '-' && arg.len() > 1 {
            if arg_chars.next().unwrap() == '-' {
                return ArgType::Long
            } 
            return ArgType:: Group
        }
        ArgType::Param
    }

    pub fn check_flag(&self, short_form: &str, long_form: &str) -> bool {
        match self.arg_type {
            ArgType::Long => self.value.eq(long_form),
            ArgType::Group => self.value.contains(short_form),
            _ => false
        }
    }
}

