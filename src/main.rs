use std::env;
use std::process;

pub mod basename;
pub mod dirname;

struct Command {
    name: &'static str,
    function: fn (error_command: String, args: Vec<String>) -> i32,
}

fn main() {
    let implemented_commands = [
        Command { name: "basename", function: basename::basename_cmd },
        Command { name: "dirname", function: dirname::dirname_cmd },
    ];

    let args: Vec<String> = env::args().collect();
    let mut invoked_command: String = basename::get_filename(&args[0]).unwrap().to_string();
    let mut error_command: String = invoked_command.clone();
    let mut args_for_command: Vec<String> = args.clone();
    if invoked_command == "rustycoreutils" {
        if args.len() > 1 {
            invoked_command = args[1].clone();
            error_command = format!("{}: {}", &args[0], &args[1]);
            args_for_command = args[1..].to_vec();
        } else {
            for command in implemented_commands.iter() {
                println!("{}", command.name);
            }
        }
    }
    for command in implemented_commands.iter() {
        if command.name == invoked_command {
            let status = (command.function)(error_command, args_for_command);
            process::exit(status);
        }
    }
}
