use std::process;

#[derive(Debug, Clone)]
pub enum CommandOptionType {
    Boolean(Option<bool>),
    Argument(Option<String>),
}

#[derive(Debug, Clone)]
pub struct CommandOption {
    pub short: char,
    pub long: &'static str,
    pub value: CommandOptionType,
}

pub const HELP_OPTION: CommandOption = CommandOption {
    short: 'h',
    long: "help",
    value: CommandOptionType::Boolean(None),
};

pub fn parse_args(invoked_command_for_print: &String, args: &Vec<String>, options: &[CommandOption]) -> (Vec<CommandOption>, Vec<String>) {
    let mut found_options: Vec<CommandOption> = Vec::new();
    let mut positional_arguments: Vec<String> = Vec::new();
    let mut current_multi_arg_option: Option<CommandOption> = None;
    for (fake_i, arg) in args[1..].iter().enumerate() {
        let i = fake_i + 1;
        match current_multi_arg_option {
            None => {},
            Some(ref current_multi_arg_option_unwrapped) => {
                match current_multi_arg_option_unwrapped.value {
                    CommandOptionType::Argument(_) => {
                        found_options.push(CommandOption {
                            value: CommandOptionType::Argument(Some(arg.clone())),
                            ..*current_multi_arg_option_unwrapped
                        });
                        current_multi_arg_option = None;
                    },
                    _ => {},
                };
                continue;
            },
        };
        if arg == "--" {
            positional_arguments.extend_from_slice(&args[i+1..]);
            break;
        }
        let found_option_for_arg = options.iter().find(|option| *arg == format!("-{}", option.short) || *arg == format!("--{}", option.long));
        match found_option_for_arg {
            Some(found_option_for_arg_unwrapped) => {
                match found_option_for_arg_unwrapped.value {
                    CommandOptionType::Boolean(_) => {
                        found_options.push(CommandOption {
                            value: CommandOptionType::Boolean(Some(true)),
                            ..*found_option_for_arg_unwrapped
                        });
                    },
                    CommandOptionType::Argument(_) => {
                        current_multi_arg_option = Some(CommandOption {
                            value: CommandOptionType::Argument(None),
                            ..*found_option_for_arg_unwrapped
                        });
                    },
                };
            },
            None => {
                positional_arguments.push(arg.clone())
            },
        };
    }

    match current_multi_arg_option {
        None => {},
        Some(option_missing_value) => {
            eprintln!("{invoked_command_for_print}: Missing value for argument -{}/--{}", option_missing_value.short, option_missing_value.long);
            process::exit(1);

        }
    }

    return (found_options.clone(), positional_arguments.clone())
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: &'static str,
    pub function: fn (invoked_command_for_print: &String, options: Vec<CommandOption>, positional_arguments: Vec<String>) -> i32,
    pub options: &'static [CommandOption],
}
