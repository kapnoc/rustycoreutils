use std::env;
use std::process;
use std::path::Path;

pub mod commands;
pub mod version;

pub mod basename;
pub mod dirname;

pub fn get_filename(path_str: &str) -> Option<&str> {
    let path = Path::new(path_str);
    let filename_str = match path.file_name() {
        None => None,
        Some(filename_str) => filename_str.to_str()
    };
    return filename_str;
}

fn rustycoreutils_no_command(invoked_command_for_print: &String, args: &Vec<String>, implemented_commands: &[commands::Command]) -> i32 {
    let (options, positional_arguments) = commands::parse_args(
        &invoked_command_for_print,
        &args,
        &[
            commands::CommandOption {
                short: 'h',
                long: "help",
                value: commands::CommandOptionType::Boolean(None),
            },
            commands::CommandOption {
                short: 'v',
                long: "version",
                value: commands::CommandOptionType::Boolean(None),
            }
        ]
    );

    if options.iter().any(|ref x| x.long == "version") {
        version::print_version_message(invoked_command_for_print);
        if options.len() == 1 {
            return 0
        }
    }

    if positional_arguments.len() > 0 {
        println!("unknown command: {}", positional_arguments.join(" "));
        println!("Available commands:");
    }

    if options.iter().any(|ref x| x.long == "help") {
        println!("Available commands:");
    }

    for command in implemented_commands.iter() {
        println!("{}", command.name);
    }

    if positional_arguments.len() > 0 {
        return 1
    }
    return 0;
}

fn main() {
    let implemented_commands = [
        basename::BASENAME_CMD,
        dirname::DIRNAME_CMD,
    ];

    let args: Vec<String> = env::args().collect();
    let mut invoked_command: String = get_filename(&args[0]).unwrap().to_string();
    let mut invoked_command_for_print: String = invoked_command.clone();
    let mut args_for_command: Vec<String> = args.clone();
    if invoked_command == "rustycoreutils" {
        if args.len() > 1 {
            invoked_command = args[1].clone();
            invoked_command_for_print = format!("{}: {}", &args[0], &args[1]);
            args_for_command = args[1..].to_vec();
        } else {
            process::exit(rustycoreutils_no_command(&invoked_command_for_print, &args_for_command, &implemented_commands));
        }
    }
    for command in implemented_commands.iter() {
        if command.name == invoked_command {
            let (options, positional_arguments) = commands::parse_args(
                &invoked_command_for_print,
                &args_for_command,
                command.options
            );
            let status = (command.function)(&invoked_command_for_print, options, positional_arguments);
            process::exit(status);
        }
    }
    process::exit(rustycoreutils_no_command(&args[0], &args, &implemented_commands));
}
