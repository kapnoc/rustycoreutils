
use crate::arguments;

pub const VERSION_OPTION: arguments::CommandOption = arguments::CommandOption {
    short: 'v',
    long: "version",
    value: arguments::CommandOptionType::Boolean,
};

pub fn print_version_message(invoked_command_for_print: &String) {
    println!("{invoked_command_for_print} 0.1.0")
}
