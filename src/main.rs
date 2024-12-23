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

fn unzip_file(filename: &str) -> i32 {
    let fname = Path::new(filename);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}

fn zip_file(output_file: &str, files: &[String]) -> i32 {
    let path = Path::new(output_file);
    let file = fs::File::create(&path).unwrap();
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for file_path in files {
        let path = Path::new(file_path);
        if path.is_file() {
            let mut file = fs::File::open(&path).unwrap();
            zip.start_file(path.file_name().unwrap().to_string_lossy(), options)
                .unwrap();
            io::copy(&mut file, &mut zip).unwrap();
        } else if path.is_dir() {
            zip_dir(path, &mut zip, options).unwrap();
        }
    }

    zip.finish().unwrap();
    println!("Created zip file: {}", output_file);
    0
}
