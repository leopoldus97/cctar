use std::{
    error::Error,
    fs::File,
    io::{self, Read},
};

use tar::models::TarArchive;

pub mod tar;
pub mod utils;

pub fn load_archive(file_name: Option<String>) -> Result<TarArchive, Box<dyn Error>> {
    let from_stdin = file_name.is_none();
    let buffer = load_file(file_name)?;

    let archive = TarArchive::from_reader(buffer, from_stdin)?;
    Ok(archive)
}

fn load_file(file_path: Option<String>) -> Result<Box<dyn Read>, Box<dyn Error>> {
    if let Some(file) = file_path {
        Ok(Box::new(File::open(file)?))
    } else {
        Ok(Box::new(io::stdin()))
    }
}
