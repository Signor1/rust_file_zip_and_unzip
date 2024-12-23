# Rust Zip/Unzip Utility

This project is a Rust-based command-line utility for compressing files into a ZIP archive and extracting files from a ZIP archive. It demonstrates how to use the [zip](https://crates.io/crates/zip) crate for handling ZIP files in Rust.

## Features
- **Zip Files**: Compress multiple files and directories into a ZIP archive.
- **Unzip Files**: Extract files from an existing ZIP archive.
- **Handles Nested Directories**: Automatically includes subdirectories during compression.
- **Cross-Platform**: Compatible with Linux, macOS, and Windows.

## Prerequisites
- [Rust](https://www.rust-lang.org/) installed on your machine.

## Installation
1. Clone this repository:
   ```bash
   git clone [rust_file_zip_and_unzip](https://github.com/Signor1/rust_file_zip_and_unzip.git)
   cd rust_file_zip_and_unzip
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage
### Unzipping a File
To extract files from a ZIP archive:
```bash
cargo run -- unzip <filename>
```
Example:
```bash
cargo run -- unzip hello_cargo.zip
```
This will extract the contents of `hello_cargo.zip` into the current directory.

### Zipping Files
To create a ZIP archive from files and directories:
```bash
cargo run -- zip <output_zipfile> <files_to_zip>...
```
Example:
```bash
cargo run -- zip my_archive.zip file1.txt folder1/
```
This will create a ZIP file named `my_archive.zip` containing `file1.txt` and the contents of `folder1/`.

## Project Structure
- **`src/main.rs`**: Contains the main logic for zipping and unzipping files.
- **Dependencies**: Uses the [zip](https://crates.io/crates/zip) crate for handling ZIP files.

## How It Works
### Unzip Functionality
1. Opens the specified ZIP file.
2. Iterates through the archive contents.
3. Extracts files and directories, preserving structure.
4. Optionally restores file permissions (on Unix-based systems).

### Zip Functionality
1. Creates a new ZIP archive.
2. Adds specified files to the archive.
3. Recursively adds contents of directories to the archive.

## Example
Given the following directory structure:
```
work/
  hello_cargo.zip
  file1.txt
  folder1/
    file2.txt
```
- To extract `hello_cargo.zip`:
  ```bash
  cargo run -- unzip hello_cargo.zip
  ```
- To create a new archive:
  ```bash
  cargo run -- zip new_archive.zip file1.txt folder1/
  ```


## Contributions
Contributions are welcome! Feel free to open issues or submit pull requests.


