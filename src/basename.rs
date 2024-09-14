
pub fn basename_cmd(error_command: String, args: Vec<String>) -> i32 {
    if args.len() < 2 {
        eprintln!("{error_command}: Please provide a path");
        return 1;
    }

    let path = &args[1];
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

    let basename_without_suffix = if args.len() == 3 && args[2].len() != basename.len() {
        let suffix = &args[2];
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
