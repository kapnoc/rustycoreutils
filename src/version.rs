
use crate::commands;

pub const VERSION_OPTION: commands::CommandOption = commands::CommandOption {
    short: 'v',
    long: "version",
    value: commands::CommandOptionType::Boolean(None),
};

pub fn print_version_message(invoked_command_for_print: &String) {
    println!("{invoked_command_for_print} 0.1.0")
}
