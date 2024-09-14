use std::path::Path;

pub fn dirname_cmd(error_command: String, args: Vec<String>) -> i32 {
    if args.len() != 2 {
        eprintln!("{}: Please provide a path", error_command);
        return 1;
    }

    let path_str = &args[1];
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

