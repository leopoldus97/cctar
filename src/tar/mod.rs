use std::{error::Error, io::Read};

pub mod models;

fn read_utf8_string_value<T: Read>(mut buffer: T, length: u32) -> Result<String, Box<dyn Error>> {
    let mut value = vec![0; length as usize];
    let num_of_bytes = buffer.read(&mut value)?;
    //println!("num_of_bytes: {:?}", num_of_bytes);
    assert_eq!(num_of_bytes, length as usize, "Failed to read value");
    let value = value
        .split(|&x| x == 0)
        .next()
        .expect("Failed to trim value");
    String::from_utf8(value.to_vec()).map_err(|e| e.into())
}

fn read_utf8_octal_value<T: Read>(mut buffer: T, length: u32) -> Result<u128, Box<dyn Error>> {
    let mut value = vec![0; length as usize];
    let num_of_bytes = buffer.read(&mut value)?;
    //println!("num_of_bytes: {:?}", num_of_bytes);
    assert_eq!(num_of_bytes, length as usize, "Failed to read value");
    let value = value
        .split(|&x| x == 0)
        .next()
        .expect("Failed to trim value");
    let string_value =
        String::from_utf8(value.to_vec()).expect("Failed to convert value to string");
    u128::from_str_radix(&string_value, 8).map_err(|e| e.into())
}
