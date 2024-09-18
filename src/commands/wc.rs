use std::fs;
use std::io;

use crate::arguments;
use crate::commands;
use crate::version;

pub const WC_CMD: commands::Command = commands::Command {
    name: "wc",
    function: wc_main,
    options: &[
        commands::HELP_OPTION,
        version::VERSION_OPTION,
        arguments::CommandOption {
            short: 'c',
            long: "bytes",
            value: arguments::CommandOptionType::Boolean,
        },
        arguments::CommandOption {
            short: 'l',
            long: "lines",
            value: arguments::CommandOptionType::Boolean,
        },
        arguments::CommandOption {
            short: 'm',
            long: "chars",
            value: arguments::CommandOptionType::Boolean,
        },
        arguments::CommandOption {
            short: 'w',
            long: "words",
            value: arguments::CommandOptionType::Boolean,
        },
    ]
};

fn read_from_source_and_count(source: &mut dyn io::Read, file_path: Option<String>, results_vec: &mut Vec<WcResults>) {
    let mut count_results = WcResults {
        file_path,
        bytes: 0,
        lines: 0,
        chars: 0,
        words: 0,
    };
    let mut buffer: [u8; 32 * 1024] = [0; 32 * 1024];
    let mut previous_buffer_leftover_bytes: Vec<u8> = Vec::new();
    let mut is_in_word = false;
    loop {
        let read_result = source.read(&mut buffer);
        match read_result {
            Ok(0) => break,
            Ok(n) => {
                count_results.bytes += n;
                let read_bytes = buffer[..n].to_vec();
                let bytes_to_convert = [previous_buffer_leftover_bytes.to_owned(), read_bytes.to_owned()].concat();
                let read_unicode_chars: &str;
                match std::str::from_utf8(&bytes_to_convert) {
                    Ok(chars) => {
                        previous_buffer_leftover_bytes = Vec::new();
                        read_unicode_chars = chars;
                    },
                    Err(error) => {
                        previous_buffer_leftover_bytes = bytes_to_convert[error.valid_up_to()..].to_vec();
                        unsafe {
                            read_unicode_chars = std::str::from_utf8_unchecked(&bytes_to_convert[..error.valid_up_to()]);
                        }
                    },
                }
                for c in read_unicode_chars.chars() {
                    count_results.chars += 1;
                    if c == '\n' {
                        count_results.lines += 1;
                    }
                    if c.is_whitespace() {
                        if is_in_word {
                            count_results.words += 1;
                        }
                        is_in_word = false;
                    } else {
                        is_in_word = true;
                    }
                }
            },
            Err(error) => eprintln!("{}", error),
        }
    }

    if is_in_word {
        count_results.words += 1;
    }
    results_vec.push(count_results)
}

#[derive(Debug, Clone)]
struct WcResults {
    file_path: Option<String>,
    bytes: usize,
    lines: usize,
    chars: usize,
    words: usize,
}

fn wc_main(invoked_command_for_print: &String, options: Vec<arguments::CommandOption>, positional_arguments: Vec<String>) -> i32 {
    if options.iter().any(|ref x| x.long == "version") {
        version::print_version_message(invoked_command_for_print);
        return 0;
    }
    if options.iter().any(|ref x| x.long == "help") {
        println!("{invoked_command_for_print}: print byte, newline and word count for file(s)");
        return 0;
    }

    let bytes_option = options.iter().any(|ref x| x.long == "bytes");
    let lines_option = options.iter().any(|ref x| x.long == "lines");
    let chars_option = options.iter().any(|ref x| x.long == "chars");
    let words_option = options.iter().any(|ref x| x.long == "words");
    let mut results: Vec<WcResults> = Vec::new();

    if positional_arguments.len() == 0 {
        let mut stdin_lock = io::stdin().lock();
        read_from_source_and_count(&mut stdin_lock, None, &mut results);
    } else {
        for file_path in positional_arguments {
            let mut file = fs::File::open(&file_path);
            match file {
                Ok(ref mut open_file) => {
                    read_from_source_and_count(open_file, Some(file_path.clone()), &mut results)
                },
                Err(error) => eprintln!("{}", error),
            }
        }
    }

    for result in results {
        let counts_as_string = [
            if lines_option {&format!("{}", result.lines)} else {""},
            if words_option {&format!("{}", result.words)} else {""},
            if chars_option {&format!("{}", result.chars)} else {""},
            if bytes_option {&format!("{}", result.bytes)} else {""},
        ];
        let filtered_counts: Vec<&&str> = counts_as_string.iter().filter(|s| s.chars().count() != 0).collect();
        let counts_to_print = match filtered_counts.clone().len() {
            0 => {
                format!(
                    " {} {} {} {}",
                    result.lines,
                    result.words,
                    result.chars,
                    result.bytes
                )
            },
            1 => {
                filtered_counts.iter().fold("".to_string(), |a, b| a + b )
            },
            _ => {
                filtered_counts.iter().fold("".to_string(), |a, b| format!("{a} {b}"))
            },
        };
        match result.file_path {
            Some(file_path) => {
                println!("{counts_to_print} {file_path}");
            },
            None => {
                println!("{counts_to_print}");
            }
        }
    }

    return 0;
}
