use std::env;
use std::process;
use std::path::Path;

mod commands;
mod version;
mod arguments;

pub fn get_filename(path_str: &str) -> Option<&str> {
    let path = Path::new(path_str);
    let filename_str = match path.file_name() {
        None => None,
        Some(filename_str) => filename_str.to_str()
    };
    return filename_str;
}

fn rustycoreutils_no_command(invoked_command_for_print: &String, args: &Vec<String>) -> i32 {
    let (options, positional_arguments) = arguments::parse_args(
        &invoked_command_for_print,
        &args,
        &[
            commands::HELP_OPTION,
            version::VERSION_OPTION,
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

    for command in commands::IMPLEMENTED_COMMANDS.iter() {
        println!("{}", command.name);
    }

    if positional_arguments.len() > 0 {
        return 1
    }
    return 0;
}

fn main() {

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
            process::exit(rustycoreutils_no_command(&invoked_command_for_print, &args_for_command));
        }
    }
    for command in commands::IMPLEMENTED_COMMANDS.iter() {
        if command.name == invoked_command {
            let (options, positional_arguments) = arguments::parse_args(
                &invoked_command_for_print,
                &args_for_command,
                command.options
            );
            let status = (command.function)(&invoked_command_for_print, options, positional_arguments);
            process::exit(status);
        }
    }
    process::exit(rustycoreutils_no_command(&args[0], &args));
}
