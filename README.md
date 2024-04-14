# Coding Challenge: `Tar`

This application was created as part of a coding challenge. The challenge details can be found [here](https://codingchallenges.fyi/challenges/challenge-tar).

The following resources were used for implementing and understanding the functionality of the `tar` command:
- [Wikipedia - tar (computing)](https://en.wikipedia.org/wiki/Tar_(computing))
- [GNU Tar Manual](https://www.gnu.org/software/tar/manual/tar.html)

## Description

A simple **t**ape **ar**chive-like cli tool created in Rust. This implementation mimics some, but not all of the functionalities of the original `tar` command.

## How to Run

To run the cctar command line tool, follow these steps:
1. Clone the repository.
2. Build the applikcation using `cargo build --release`. That will generate the executable in the `target/release` directory called `cctar`.
3. Run the application.

## Arguments

You can run the application with the following arguments:
- `cctar -c <archive_name> <file1> <file2> ...` to create an archive.
- `cctar -x <archive_name>` to extract an archive.
- `cctar -t <archive_name>` to list the contents of an archive.

## Notes

The application accepts file paths (using the `-f` flag) and piped in files (using `cat <file> |` or other similar tools).
