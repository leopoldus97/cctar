use std::error::Error;

use clap::{Arg, Command};

pub struct Arguments {
    pub list: bool,
    pub file: Option<String>,
}

pub fn setup_parser() -> Result<Arguments, Box<dyn Error>> {
    let matches = Command::new("cctar")
        .version("0.1.0")
        .args([
            Arg::new("list")
                .short('t')
                .long("list")
                .num_args(0)
                .help("Lists the contents of an archive")
                .conflicts_with_all(&["extract", "create"]),
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .num_args(1)
                .help("Reads the archive from the specified file. If not specified, tries to read from stdin"),
        ])
        .get_matches();

    let list = matches
        .get_one::<bool>("list")
        .map(|list| *list)
        .unwrap_or_else(|| false);
    let file = matches
        .get_one::<String>("file")
        .map(|file| file.to_string());

    Ok(Arguments {
        list,
        file,
    })
}
