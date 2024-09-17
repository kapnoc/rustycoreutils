
use crate::arguments;

pub mod basename;
pub mod cat;
pub mod dirname;

pub const HELP_OPTION: arguments::CommandOption = arguments::CommandOption {
    short: 'h',
    long: "help",
    value: arguments::CommandOptionType::Boolean,
};

#[derive(Debug, Clone)]
pub struct Command {
    pub name: &'static str,
    pub function: fn (invoked_command_for_print: &String, options: Vec<arguments::CommandOption>, positional_arguments: Vec<String>) -> i32,
    pub options: &'static [arguments::CommandOption],
}

pub const IMPLEMENTED_COMMANDS: [Command; 3] = [
    basename::BASENAME_CMD,
    cat::CAT_CMD,
    dirname::DIRNAME_CMD,
];
