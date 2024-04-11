use std::{
    error::Error,
    fs::File,
    io::{self, Read, Write},
};

use cctar::{tar::models::TarArchive, utils::arg_parser::setup_parser};

// Name: The name of the file or directory. Occupies the first 100 bytes of the header.
// Mode: The file permissions. Occupies the next 8 bytes.
// UID: The user ID of the owner. Occupies the next 8 bytes.
// GID: The group ID of the owner. Occupies the next 8 bytes.
// Size: The size of the file in bytes. Occupies the next 12 bytes.
// MTime: The last modification time of the file. Occupies the next 12 bytes.
// Checksum: The checksum for the header block. Occupies the next 8 bytes.
// Typeflag: The type of the file. Occupies the next 1 byte.
// Linkname: The name of the linked file for hard or symbolic links. Occupies the next 100 bytes.
// Magic: The magic string. Occupies the next 6 bytes.
// Version: The version of the tar file format. Occupies the next 2 bytes.
// Uname: The user name of the owner. Occupies the next 32 bytes.
// Gname: The group name of the owner. Occupies the next 32 bytes.
// Devmajor: The major number of the device for character or block device files. Occupies the next 8 bytes.
// Devminor: The minor number of the device for character or block device files. Occupies the next 8 bytes.
// Prefix: The prefix of the file name. Occupies the next 155 bytes.

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
