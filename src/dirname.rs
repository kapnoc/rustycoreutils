use std::path::Path;

use crate::commands;
use crate::version;

pub const DIRNAME_CMD: commands::Command = commands::Command {
    name: "dirname",
    function: dirname_main,
    options: &[
        commands::HELP_OPTION,
        version::VERSION_OPTION,
    ]
};

fn dirname_main(invoked_command_for_print: &String, options: Vec<commands::CommandOption>, positional_arguments: Vec<String>) -> i32 {
    if options.iter().any(|ref x| x.long == "version") {
        version::print_version_message(invoked_command_for_print);
        return 0;
    }
    if options.iter().any(|ref x| x.long == "help") {
        todo!("No help here :D");
        return 0;
    }

    if positional_arguments.len() != 1 {
        eprintln!("{}: Please provide a path", invoked_command_for_print);
        return 1;
    }

    let path_str = &positional_arguments[0];
    let path = Path::new(path_str);
    let dirname_str = if path_str == "/" {
        "/"
    } else {
        match path.parent() {
            None => ".",
            Some(filename_str) => filename_str.to_str().unwrap(),
        }
    };

    if dirname_str.len() == 0 {
        println!(".");
        return 0;
    }

    println!("{}", dirname_str);
    return 0;
}

