use std::{
    error::Error,
    ffi::CStr,
    fs::{self, File},
    io::{Read, Write},
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
    time::SystemTime,
};

use libc::{getgrgid, getpwuid};

use super::models::TarArchive;

impl TarArchive {
    pub fn write_archive(tar_filename: &str, files: Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut tar_file = File::create(tar_filename)?;
        // let mut blocks: Vec<[u8; 512]> = Vec::new();
        let mut blocks: Vec<u8> = Vec::new();

        for file in files {
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

            let (prefix, name) = if file.len() > 100 {
                let (prefix, name) = file.split_at(file.len() - 100);
                (prefix, name)
            } else {
                ("", file.as_str())
            };

            let mut header = [0u8; 512];

            let mut cursor = 0;

            write!(&mut header[cursor..cursor + 100], "{:<100}", name)?; // Name
            cursor += 100;
            write!(
                &mut header[cursor..cursor + 8],
                "{:07o}",
                file_mode & 0o0777
            )?; // Mode
            cursor += 8;
            write!(&mut header[cursor..cursor + 8], "{:07o}", uid)?; // UID
            cursor += 8;
            write!(&mut header[cursor..cursor + 8], "{:07o}", gid)?; // GID
            cursor += 8;
            write!(&mut header[cursor..cursor + 12], "{:011o}", file_size_met)?; // Size
            cursor += 12;
            write!(&mut header[cursor..cursor + 12], "{:011o}", mtime)?; // MTime
            cursor += 12;
            write!(&mut header[cursor..cursor + 8], "{:8}", ' ')?; // Checksum
            cursor += 8;
            write!(&mut header[cursor..cursor + 1], "{}", type_flag)?; // Typeflag
            cursor += 1;
            write!(&mut header[cursor..cursor + 100], "{:<100}", linkname)?; // Linkname
            cursor += 100;
            write!(&mut header[cursor..cursor + 6], "ustar ")?; // Magic
            cursor += 6;
            write!(&mut header[cursor..cursor + 2], "{:<2}", " \0")?; // Version
            cursor += 2;
            write!(&mut header[cursor..cursor + 32], "{:<32}", username)?; // Uname
            cursor += 32;
            write!(&mut header[cursor..cursor + 32], "{:<32}", group_name)?; // Gname
            cursor += 32;
            write!(&mut header[cursor..cursor + 8], "{:07}", "\0")?; // Devmajor <- Apparently in the file created by the tar command these are 0 bytes
            cursor += 8;
            write!(&mut header[cursor..cursor + 8], "{:07}", "\0")?; // Devminor <- Apparently in the file created by the tar command these are 0 bytes
            cursor += 8;
            write!(&mut header[cursor..cursor + 155], "{:<155}", prefix)?; // Prefix

            for (i, element) in header.iter_mut().enumerate() {
                if *element == 32 && i != 155 && i != 262 && i != 263 {
                    *element = 0;
                }

                if i > 147 && i < 156 {
                    *element = b' ';
                }
            }

            let checksum: u32 = header.iter().map(|&b| u32::from(b)).sum();

            write!(&mut header[148..156], "{:<06o}\0", checksum)?; // Checksum

            blocks.extend(header);

            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;

            match buffer.len() {
                len if len < 512 => {
                    let padding = vec![0; 512 - buffer.len()];
                    buffer.extend(padding);
                }
                len if len > 512 => {
                    let padding = vec![0; 512 - (buffer.len() % 512)];
                    buffer.extend(padding);
                }
                _ => {
                    let padding = vec![0; 512];
                    buffer.extend(padding);
                }
            }

            for i in (0..buffer.len()).step_by(512) {
                let mut block = [0u8; 512];
                block.copy_from_slice(&buffer[i..i + 512]);
                blocks.extend(block);
            }
        }

        let padding_blocks = if blocks.len() < 20 * 512 {
            20 - (blocks.len() / 512)
        } else {
            2
        };

        for _ in 0..padding_blocks {
            blocks.extend([0u8; 512]);
        }

        tar_file.write_all(&blocks)?;

        Ok(())
    }
}
