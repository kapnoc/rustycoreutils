
use crate::commands;
use crate::version;

pub const BASENAME_CMD: commands::Command = commands::Command {
    name: "basename",
    function: basename_main,
    options: &[
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
};

fn basename_main(invoked_command_for_print: &String, options: Vec<commands::CommandOption>, positional_arguments: Vec<String>) -> i32 {
    if options.iter().any(|ref x| x.long == "version") {
        version::print_version_message(invoked_command_for_print);
        return 0;
    }
    if options.iter().any(|ref x| x.long == "help") {
        todo!("No help here :D");
        return 0;
    }

    if positional_arguments.len() < 1 {
        eprintln!("{invoked_command_for_print}: Please provide a path");
        return 1;
    }

    let path = &positional_arguments[0];
    if path.len() == 0 {
        println!("");
        return 0;
    }

    let mut end_index = 0;
    for (i, c) in path.chars().enumerate() {
        if c != '/' {
            end_index = i + 1;
        }
    }
    if end_index == 0 {
        println!("/");
        return 0;
    }

    let path_without_trailing_slash = &path[..end_index];
    let basename_start_index: usize = if path_without_trailing_slash.find("/").is_some() {
        let mut last_slash_index_before_basename: usize = 0;
        for (i, c) in path_without_trailing_slash.chars().enumerate() {
            if c == '/' {
                last_slash_index_before_basename = i;
            }
        }
        last_slash_index_before_basename + 1
    } else {
        0
    };
    let basename = &path_without_trailing_slash[basename_start_index..];

    let basename_without_suffix = if positional_arguments.len() == 2 && positional_arguments[1].len() != basename.len() {
        let suffix = &positional_arguments[1];
        let suffix_matches: Vec<_> = basename.rmatch_indices(suffix).collect();
        if suffix_matches.len() > 0 && suffix_matches[0].0 == basename.len() - suffix.len() {
            &basename[..suffix_matches[0].0]
        } else {
            basename
        }
    } else {
        basename
    };

    println!("{basename_without_suffix}");
    return 0;
}
