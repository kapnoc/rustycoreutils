use std::fs;
use std::io;
use std::io::Write;

use crate::arguments;
use crate::commands;
use crate::version;

pub const CAT_CMD: commands::Command = commands::Command {
    name: "cat",
    function: cat_main,
    options: &[
        commands::HELP_OPTION,
        version::VERSION_OPTION,
    ]
};

fn read_from_source_and_print(source: &mut dyn io::Read) {
    let mut stdout_lock = std::io::stdout().lock();
    let mut buffer: [u8; 32 * 1024] = [0; 32 * 1024];
    loop {
        let read_result = source.read(&mut buffer);
        match read_result {
            Ok(0) => break,
            Ok(n) => {
                let write_result = stdout_lock.write(&buffer[..n]);
                match write_result {
                    Err(error) => eprintln!("{}", error),
                    _ => continue,
                };
            },
            Err(error) => eprintln!("{}", error),
        }
    }
}

fn cat_main(invoked_command_for_print: &String, options: Vec<arguments::CommandOption>, positional_arguments: Vec<String>) -> i32 {
    if options.iter().any(|ref x| x.long == "version") {
        version::print_version_message(invoked_command_for_print);
        return 0;
    }
    if options.iter().any(|ref x| x.long == "help") {
        todo!("No help here :D");
        // return 0;
    }

    if positional_arguments.len() == 0 {
        let mut stdin_lock = io::stdin().lock();
        read_from_source_and_print(&mut stdin_lock);
    } else {
        for file_path in positional_arguments {
            let mut file = fs::File::open(file_path);
            match file {
                Ok(ref mut open_file) => {
                    read_from_source_and_print(open_file)
                },
                Err(error) => eprintln!("{}", error),
            }
        }
    }

    return 0;
}
