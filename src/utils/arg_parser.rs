use std::error::Error;

use clap::{Arg, Command};

pub struct Arguments {
    pub list: bool,
    pub file: Option<String>,
    pub extract: bool,
    pub create: bool,
    pub input_files: Vec<String>,
}

pub fn setup_parser() -> Result<Arguments, Box<dyn Error>> {
    let matches = Command::new("cctar")
        .about("Simple tar-like utility written in Rust")
        .version("0.1.0")
        .args([
            Arg::new("list")
                .short('t')
                .long("list")
                .num_args(0)
                .help("Lists the contents of an archive")
                .conflicts_with_all(["extract", "create"]),
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .num_args(1)
                .help("Reads the archive from the specified file. If not specified, tries to read from stdin"),
            Arg::new("extract")
                .short('x')
                .long("extract")
                .alias("get")
                .num_args(0)
                .help("Extracts the contents of an archive")
                .conflicts_with_all(["list", "create"]),
            Arg::new("create")
                .short('c')
                .long("create")
                .num_args(0)
                .help("Creates a new archive")
                .conflicts_with_all(["extract", "list"])
                .requires("file"),
            Arg::new("input_files")
                .index(1)
                .num_args(1..)
                .required(false)
                .help("Input files to be added to the archive")
                .conflicts_with_all(["list", "extract"]),
        ])
        .get_matches();

    let list = matches.get_one::<bool>("list").copied().unwrap_or(false);
    let file = matches
        .get_one::<String>("file")
        .map(|file| file.to_string());
    let extract = matches.get_one::<bool>("extract").copied().unwrap_or(false);
    let create = matches.get_one::<bool>("create").copied().unwrap_or(false);
    let input_files = matches
        .get_many::<String>("input_files")
        .map(|values| {
            values
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    Ok(Arguments {
        list,
        file,
        extract,
        create,
        input_files,
    })
}
