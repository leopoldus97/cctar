use std::{
    error::Error,
    fs::File,
    io::{self, Read, Write},
};

use cctar::{tar::models::TarArchive, utils::arg_parser::setup_parser};

fn main() -> Result<(), Box<dyn Error>> {
    let args = setup_parser()?;

    if args.list {
        // List files
        let archive = load_archive(args.file)?;
        for file in archive.files {
            println!("{}", file.file_name);
        }
    } else if args.extract {
        // TODO: Add metadata to the extracted files!
        // Extract files
        let archive = load_archive(args.file)?;
        for file in archive.files {
            let mut f = File::create(file.file_name)?;
            f.write_all(&file.body)?;
        }
    } else if args.create {
        // Create archive
        if let Some(tar_filename) = args.file {
            TarArchive::write_archive(&tar_filename, args.input_files)?;
        }
    }

    Ok(())
}

fn load_file(file_path: Option<String>) -> Result<Box<dyn Read>, Box<dyn Error>> {
    if let Some(file) = file_path {
        Ok(Box::new(File::open(file)?))
    } else {
        Ok(Box::new(io::stdin()))
    }
}

fn load_archive(file_name: Option<String>) -> Result<TarArchive, Box<dyn Error>> {
    let from_stdin = file_name.is_none();
    let buffer = load_file(file_name)?;

    let archive = TarArchive::from_reader(buffer, from_stdin)?;
    Ok(archive)
}
