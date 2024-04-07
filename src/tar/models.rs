use std::{error::Error, io::Read};

use super::{read_utf8_octal_value, read_utf8_string_value};

pub struct TarArchive {
    pub files: Vec<TarFile>,
}

pub struct TarFile {
    pub file_name: String,
    pub file_mode: u128,
    pub owner_uid: u128,
    pub group_uid: u128,
    pub file_size: u128,
    pub last_mod_time: u128,
    pub checksum: String,
    pub link_indicator: String,
    pub linked_file_nbame: String,
    pub ustar: String,
    pub ustar_version: String,
    pub owner_name: String,
    pub group_name: String,
    pub device_major: String,
    pub device_minor: String,
    pub prefix: String,
    pub body: String,
}

impl TarArchive {
    pub fn from_reader<T: Read>(mut buffer: T, from_stdin: bool) -> Result<Self, Box<dyn Error>> {
        let mut files = Vec::new();

        loop {
            let mut leftover: u32 = 512;
            let file_name = read_utf8_string_value(&mut buffer, 100)?;
            //println!("File name -> {:?}", file_name);
            leftover -= 100;

            if file_name.is_empty() {
                break;
            }

            let file_mode = read_utf8_octal_value(&mut buffer, 8)?;
            //println!("File mode -> {:?}", file_mode);
            leftover -= 8;
            let owner_uid = read_utf8_octal_value(&mut buffer, 8)?;
            //println!("Owner UID -> {:?}", owner_uid);
            leftover -= 8;
            let group_uid = read_utf8_octal_value(&mut buffer, 8)?;
            //println!("Group UID -> {:?}", group_uid);
            leftover -= 8;
            let file_size = read_utf8_octal_value(&mut buffer, 12)?;
            //println!("File size -> {:?}", file_size);
            leftover -= 12;
            let last_mod_time = read_utf8_octal_value(&mut buffer, 12)?;
            //println!("Last mod time -> {:?}", last_mod_time);
            leftover -= 12;
            let checksum = read_utf8_string_value(&mut buffer, 8)?;
            //println!("Checksum -> {:?}", checksum);
            leftover -= 8;
            let link_indicator = read_utf8_string_value(&mut buffer, 1)?;
            //println!("Link indicator -> {:?}", link_indicator);
            leftover -= 1;
            let linked_file_nbame = read_utf8_string_value(&mut buffer, 100)?;
            //println!("Linked file name -> {:?}", linked_file_nbame);
            leftover -= 100;
            let ustar = read_utf8_string_value(&mut buffer, 6)?;
            //println!("Ustar -> {:?}", ustar);
            leftover -= 6;
            let ustar_version = read_utf8_string_value(&mut buffer, 2)?;
            //println!("Ustar version -> {:?}", ustar_version);
            leftover -= 2;
            let owner_name = read_utf8_string_value(&mut buffer, 32)?;
            //println!("Owner name -> {:?}", owner_name);
            leftover -= 32;
            let group_name = read_utf8_string_value(&mut buffer, 32)?;
            //println!("Group name -> {:?}", group_name);
            leftover -= 32;
            let device_major = read_utf8_string_value(&mut buffer, 8)?;
            //println!("Device major -> {:?}", device_major);
            leftover -= 8;
            let device_minor = read_utf8_string_value(&mut buffer, 8)?;
            //println!("Device minor -> {:?}", device_minor);
            leftover -= 8;
            let prefix = read_utf8_string_value(&mut buffer, 155)?;
            //println!("Prefix -> {:?}", prefix);
            leftover -= 155;
            let _leftover = read_utf8_string_value(&mut buffer, leftover)?;
            //println!("Leftover -> {:?}", _leftover);

            let body = TarArchive::read_body(&mut buffer, 512)?;
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

    fn read_body<T: Read>(mut buffer: T, size: u128) -> Result<String, Box<dyn Error>> {
        let mut iterations = size / 512;
        if size % 512 != 0 {
            iterations += 1;
        }
        let mut blocks = Vec::new();
        for _ in 0..iterations {
            let block = read_utf8_string_value(&mut buffer, 512)?;
            blocks.push(block);
        }

        let result = blocks.join("");

        Ok(result)
    }
}
