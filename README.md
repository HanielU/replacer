# Replacer CLI Tool

This is a command-line interface (CLI) tool written in Rust that allows you to replace file extensions, replace occurrences of a string in a filename, and replace a string in a file or all files in a directory.

## Prerequisites

- Rust programming language installed on your machine.

## How to Use

1. Build the project using the Rust compiler by running `cargo build --release`. The executable will be located in the `target/release` directory.

2. Run the tool with the desired options. The general usage is:

```sh
./replacer [OPTIONS]
```

### Options

- `--dir <directory> <old_extension> <new_extension>`: Replace the extension of all files in the specified directory and its subdirectories. `<old_extension>` is the current extension and `<new_extension>` is the new extension you want to change to.

- `--file <file_path> <pattern> <new_string>`: Replace all occurrences of the regular expression `<pattern>` with `<new_string>` in the file at `<file_path>`.

- `--dirfile <directory> <pattern> <new_string>`: Replace all occurrences of the regular expression `<pattern>` with `<new_string>` in all files in the specified directory and its subdirectories.

- `--filename <directory> <old_string> <new_string>`: Replace all occurrences of `<old_string>` with `<new_string>` in the filenames in the specified directory and its subdirectories.

### Examples

- To replace the extension of all `.txt` files to `.md` in the `documents` directory, run:

```sh
./replacer --dir documents txt md
```

- To replace all occurrences of the regular expression `foo\d+` with `bar` in `file.txt`, run:

```sh
./replacer --file file.txt "foo\\d+" bar
```

- To replace all occurrences of the regular expression `foo\d+` with `bar` in all files in the `src` directory, run:

```sh
./replacer --dirfile src "foo\\d+" bar
```

- To replace all occurrences of `foo` with `bar` in filenames in the `src` directory, run:

```sh
./replacer --filename src foo bar
```

## Note

This tool will recursively search through directories. Be careful when specifying directories, as this tool will modify files in place. Always make sure you have a backup of your files before running this tool.
