use std::path::Path;

pub fn get_filename(path_str: &str) -> Option<&str> {
    let path = Path::new(path_str);
    let filename_str = match path.file_name() {
        None => None,
        Some(filename_str) => filename_str.to_str()
    };
    return filename_str;
}

pub fn basename_cmd(error_command: String, args: Vec<String>) -> i32 {
    if args.len() < 2 {
        eprintln!("{}: Please provide a path", error_command);
        return 1;
    }

    let filename = get_filename(&args[1]).unwrap();
    let mut shortened_filename = "";
    if args.len() >= 3 {
        let suffix = &args[2];
        match filename.strip_suffix(suffix) {
            None => shortened_filename = filename,
            Some(result) => shortened_filename = result,
        }
    }

    if shortened_filename.len() == 0 {
        shortened_filename = &filename;
    }

    println!("{}", shortened_filename);
    return 0;
}
