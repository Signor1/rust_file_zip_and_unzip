use std::fs;
use std::io::{self, Write};
use std::path::Path;
use zip::{write::FileOptions, ZipArchive, ZipWriter};

fn main() {
    std::process::exit(logic_main());
}

fn logic_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        println!(
            "Usage: {} <zip|unzip> <filename> [<files_to_zip>...]",
            args[0]
        );
        return 1;
    };

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "unzip" => unzip_file(filename),
        "zip" => {
            if args.len() < 4 {
                println!("Usage: {} zip <output_zipfile> <files_to_zip>...", args[0]);
                return 1;
            }
            zip_file(filename, &args[3..])
        }
        _ => {
            println!("Invalid command: {}. Use 'zip' or 'unzip'.", command);
            1
        }
    }
}
