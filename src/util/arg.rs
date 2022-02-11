#[derive(Debug)]
pub struct Argument {
    pub arg_type: ArgType,
    pub value: String
}

#[derive(Debug)]
#[derive(PartialEq)]
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

    fn detect_flag(arg: &String) -> ArgType {
        let mut arg_chars = arg.chars();

        if arg_chars.nth(0).unwrap() == '-' && arg.len() != 1 {
            if arg_chars.nth(1).unwrap() == '-' {
                return ArgType::Long
            } 
            return ArgType:: Group
        }

        return ArgType::Param
    }
}

