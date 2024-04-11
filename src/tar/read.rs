use std::{error::Error, io::Read};

use crate::tar::read_binary_value;

use super::{
    models::{TarArchive, TarFile},
    read_utf8_octal_value, read_utf8_string_value,
};

impl TarArchive {
    pub fn from_reader<T: Read>(mut buffer: T, from_stdin: bool) -> Result<Self, Box<dyn Error>> {
        let mut files = Vec::new();

        loop {
            let mut leftover: u32 = 512;
            let file_name = read_utf8_string_value(&mut buffer, 100)?;
            leftover -= 100;

            if file_name.is_empty() {
                break;
            }

            let file_mode = read_utf8_octal_value(&mut buffer, 8)?;
            leftover -= 8;
            let owner_uid = read_utf8_octal_value(&mut buffer, 8)?;
            leftover -= 8;
            let group_uid = read_utf8_octal_value(&mut buffer, 8)?;
            leftover -= 8;
            let file_size = read_utf8_octal_value(&mut buffer, 12)?;
            leftover -= 12;
            let last_mod_time = read_utf8_octal_value(&mut buffer, 12)?;
            leftover -= 12;
            let checksum = read_utf8_string_value(&mut buffer, 8)?;
            leftover -= 8;
            let link_indicator = read_utf8_string_value(&mut buffer, 1)?;
            leftover -= 1;
            let linked_file_nbame = read_utf8_string_value(&mut buffer, 100)?;
            leftover -= 100;
            let ustar = read_utf8_string_value(&mut buffer, 6)?;
            leftover -= 6;
            let ustar_version = read_utf8_string_value(&mut buffer, 2)?;
            leftover -= 2;
            let owner_name = read_utf8_string_value(&mut buffer, 32)?;
            leftover -= 32;
            let group_name = read_utf8_string_value(&mut buffer, 32)?;
            leftover -= 32;
            let device_major = read_utf8_string_value(&mut buffer, 8)?;
            leftover -= 8;
            let device_minor = read_utf8_string_value(&mut buffer, 8)?;
            leftover -= 8;
            let prefix = read_utf8_string_value(&mut buffer, 155)?;
            leftover -= 155;
            let _leftover = read_utf8_string_value(&mut buffer, leftover)?;

            let body = TarArchive::read_body(&mut buffer, file_size)?;
            files.push(TarFile {
                file_name,
                file_mode,
                owner_uid,
                group_uid,
                file_size,
                last_mod_time,
                checksum,
                link_indicator,
                linked_file_nbame,
                ustar,
                ustar_version,
                owner_name,
                group_name,
                device_major,
                device_minor,
                prefix,
                body,
            });

            if from_stdin {
                let _ = read_utf8_string_value(&mut buffer, 1)?;
            }
        }
        Ok(TarArchive { files })
    }

    fn read_body<T: Read>(mut buffer: T, size: u128) -> Result<Vec<u8>, Box<dyn Error>> {
        println!("Size -> {:?}", size);
        let mut iterations = size / 512;
        println!("Iterations -> {:?}", iterations);
        if size % 512 != 0 {
            iterations += 1;
        }
        println!("Iterations -> {:?}", iterations);
        let mut blocks = Vec::new();
        let mut it = 0;
        for _ in 0..iterations {
            it += 1;
            let block = read_binary_value(&mut buffer, 512)?;
            blocks.push(block);
        }

        println!("It -> {:?}", it);

        let result: Vec<u8> = blocks.into_iter().flat_map(|v| v.into_iter()).collect(); // blocks.join("");

        println!("Result -> {:?}", result.len());

        Ok(result)
    }
}
