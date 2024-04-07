use std::os::unix::fs::MetadataExt;
use std::{
    error::Error,
    ffi::CStr,
    fs::{self, File},
    io::{self, Read, Write},
    os::unix::fs::{FileTypeExt, PermissionsExt},
    time::SystemTime,
};

use libc::{getgrgid, getpwuid};

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

    // List files
    if args.list {
        let archive = load_archive(args.file)?;
        for file in archive.files {
            println!("{}", file.file_name);
        }
    } else if args.extract {
        // Extract files
        let archive = load_archive(args.file)?;
        for file in archive.files {
            let mut f = File::create(file.file_name)?;
            f.write(file.body.as_bytes())?;
        }
    } else if args.create {
        // Create archive
        if let Some(tar_filename) = args.file {
            let mut tar_file = File::create(tar_filename)?;
            // let mut blocks: Vec<[u8; 512]> = Vec::new();
            let mut blocks: Vec<u8> = Vec::new();

            for file in args.input_files {
                let mut f = File::open(&file)?;
                let metadata = f.metadata()?;
                let file_mode = metadata.permissions().mode();
                let file_size_met = metadata.len();
                let mtime = metadata
                    .modified()?
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs();

                let uid = metadata.uid();
                let gid = metadata.gid();
                let file_type = metadata.file_type();
                let type_flag = match file_type {
                    _ if file_type.is_file() => Ok('0'),
                    _ if file_type.is_dir() => Ok('5'),
                    _ if file_type.is_symlink() => Ok('2'),
                    _ if file_type.is_block_device() => Ok('4'),
                    _ if file_type.is_char_device() => Ok('3'),
                    _ if file_type.is_fifo() => Ok('6'),
                    _ => Err("Type flag is not supported!"),
                }?;

                let linkname = if file_type.is_symlink() {
                    fs::read_link(&file)?.to_string_lossy().into_owned()
                } else {
                    String::new()
                };

                let mut username = String::new();
                let mut group_name = String::new();

                unsafe {
                    let user = getpwuid(uid);
                    if !user.is_null() {
                        let name = CStr::from_ptr((*user).pw_name);
                        let usr = name.to_str().map_err(|_| "Couldn't get user name!")?;
                        username = usr.to_owned();
                    }

                    let group = getgrgid(gid);
                    if !group.is_null() {
                        let name = CStr::from_ptr((*group).gr_name);
                        let grp = name.to_str().map_err(|_| "Couldn't get group name!")?;
                        group_name = grp.to_owned();
                    }
                }

                // let dev = metadata.dev();
                // let devmajor = (dev >> 8) as u8;
                // let devminor = (dev & 0xFF) as u8;

                let (prefix, name) = if file.len() > 100 {
                    let (prefix, name) = file.split_at(file.len() - 100);
                    (prefix, name)
                } else {
                    ("", file.as_str())
                };
        
                let mut header = [0u8; 512];

                let mut cursor = 0;

                write!(&mut header[cursor..cursor+100], "{:<100}", name)?; // Name
                cursor += 100;
                write!(&mut header[cursor..cursor+8], "{:07o}", file_mode & 0o0777)?; // Mode
                cursor += 8;
                write!(&mut header[cursor..cursor+8], "{:07o}", uid)?; // UID
                cursor += 8;
                write!(&mut header[cursor..cursor+8], "{:07o}", gid)?; // GID
                cursor += 8;
                write!(&mut header[cursor..cursor+12], "{:011o}", file_size_met)?; // Size
                cursor += 12;
                write!(&mut header[cursor..cursor+12], "{:011o}", mtime)?; // MTime
                cursor += 12;
                write!(&mut header[cursor..cursor+8], "{:8}", " ")?; // Checksum
                cursor += 8;
                write!(&mut header[cursor..cursor+1], "{}", type_flag)?; // Typeflag
                cursor += 1;
                write!(&mut header[cursor..cursor+100], "{:<100}", linkname)?; // Linkname
                cursor += 100;
                write!(&mut header[cursor..cursor+6], "ustar ")?; // Magic
                cursor += 6;
                write!(&mut header[cursor..cursor+2], "{:<2}", " \0")?; // Version
                cursor += 2;
                write!(&mut header[cursor..cursor+32], "{:<32}", username)?; // Uname
                cursor += 32;
                write!(&mut header[cursor..cursor+32], "{:<32}", group_name)?; // Gname
                cursor += 32;
                // write!(&mut header[cursor..cursor+8], "{:07o}", devmajor)?; // Devmajor
                write!(&mut header[cursor..cursor+8], "{:07}", "\0")?; // Devmajor <- Apparently in the file created by the tar command these are 0 bytes
                cursor += 8;
                //write!(&mut header[cursor..cursor+8], "{:07o}", devminor)?; // Devminor
                write!(&mut header[cursor..cursor+8], "{:07}", "\0")?; // Devminor <- Apparently in the file created by the tar command these are 0 bytes
                cursor += 8;
                write!(&mut header[cursor..cursor+155], "{:<155}", prefix)?; // Prefix

                for i in 0..512 {
                    if header[i] == 32 && i != 155 && i != 262 && i != 263 {
                        header[i] = 0;
                    }
                }

                let checksum: u32 = header.iter().map(|&b| u32::from(b)).sum();

                write!(&mut header[148..156], "{:<06o}\0", checksum)?; // Checksum

                blocks.extend(header);

                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;

                // TODO: Make this work with files that are not a multiple of 512 bytes, but bigger than 512 bytes
                if buffer.len() < 512 {
                    let padding = vec![0; 512 - buffer.len()];
                    buffer.extend(padding);
                } else if buffer.len() > 512 {
                    let padding = vec![0; 512 - (buffer.len() % 512)];
                    buffer.extend(padding);
                }

                for i in (0..buffer.len()).step_by(512) {
                    let mut block = [0u8; 512];
                    block.copy_from_slice(&buffer[i..i+512]);
                    blocks.extend(block);
                }

                /* println!("File size: {:?}", tar_file.metadata()?.len());

                println!("{:?}", header);

                // FIX: For some reason when it is writing the header for the second file it only gets to the point where the filename ends
                if let Ok(bytes_written) = tar_file.write(&header) {
                    if bytes_written != 512 {
                        return Err("Couldn't write header to tar file!".into());
                    }

                    println!("{:?}", buffer);
                    
                    if let Ok(bytes_written) = tar_file.write(&buffer) {
                        if bytes_written != buffer.len() {
                            return Err("Couldn't write file to tar file!".into());
                        }
                    }
                }

                println!("File size: {:?}", tar_file.metadata()?.len()); */
            }

            println!("Buffer length pre write: {:?}", blocks.len());

            let padding_blocks = if blocks.len() < 20 * 512 {
                20 - (blocks.len() / 512)
            } else {
                2
            };

            for _ in 0..padding_blocks {
                blocks.extend([0u8; 512]);
            }

            println!("Buffer length post write: {:?}", blocks.len());

            tar_file.write_all(&blocks)?;
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
