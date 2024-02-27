use std::env;
use std::fs;
use std::path::Path;

fn replace_extension(dir: &str, old_ext: &str, new_ext: &str) {
    let path = Path::new(dir);
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if entry_path.is_dir() {
                replace_extension(
                    entry_path.to_str().unwrap(),
                    old_ext,
                    new_ext,
                );
            } else {
                let file_name =
                    entry_path.file_name().unwrap().to_str().unwrap();
                if file_name.ends_with(old_ext) {
                    let new_file_name = file_name.replace(old_ext, new_ext);
                    let new_path = entry_path.with_file_name(new_file_name);
                    fs::rename(entry_path, new_path).unwrap();
                }
            }
        }
    }
}

fn replace_occurence_in_filename(
    dir: &str,
    old_string: &str,
    new_string: &str,
) {
    let path = Path::new(dir);
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if entry_path.is_dir() {
                replace_occurence_in_filename(
                    entry_path.to_str().unwrap(),
                    old_string,
                    new_string,
                );
            } else {
                let file_name =
                    entry_path.file_name().unwrap().to_str().unwrap();
                if file_name.contains(old_string) {
                    let new_file_name =
                        file_name.replace(old_string, new_string);
                    let new_path = entry_path.with_file_name(new_file_name);
                    fs::rename(entry_path, new_path).unwrap();
                }
            }
        }
    }
}

fn replace_string_in_file(file_path: &str, old_string: &str, new_string: &str) {
    let mut file = fs::OpenOptions::new().write(true).open(file_path).unwrap();
    let file_content = fs::read_to_string(file_path).unwrap();
    let new_content = file_content.replace(old_string, new_string);
    file.set_len(0).unwrap();
    std::io::Write::write_all(&mut file, new_content.as_bytes()).unwrap();
}

fn replace_string_in_dir(dir: &str, old_string: &str, new_string: &str) {
    let path = Path::new(dir);
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if entry_path.is_dir() {
                replace_string_in_dir(
                    entry_path.to_str().unwrap(),
                    old_string,
                    new_string,
                );
            } else {
                replace_string_in_file(
                    entry_path.to_str().unwrap(),
                    old_string,
                    new_string,
                );
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} --dir <directory> <old_extension> <new_extension> --file <file_path> <old_string> <new_string> --filename <directory> <old_string> <new_string> --dirfile <directory> <old_string> <new_string>", args[0]);
        return;
    }

    for i in 1..args.len() {
        if args[i] == "--dir" {
            let dir = &args[i + 1];
            let old_ext = &args[i + 2];
            let new_ext = &args[i + 3];
            replace_extension(dir, old_ext, new_ext);
        } else if args[i] == "--file" {
            let file_path = &args[i + 1];
            let old_string = &args[i + 2];
            let new_string = &args[i + 3];
            replace_string_in_file(file_path, old_string, new_string);
        } else if args[i] == "--dirfile" {
            let dir = &args[i + 1];
            let old_string = &args[i + 2];
            let new_string = &args[i + 3];
            replace_string_in_dir(dir, old_string, new_string);
        } else if args[i] == "--filename" {
            let dir = &args[i + 1];
            let old_string = &args[i + 2];
            let new_string = &args[i + 3];
            replace_occurence_in_filename(dir, old_string, new_string);
        }
    }
}
