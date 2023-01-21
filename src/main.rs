/*
 * Program Notes
 * ----------------
 * Rust implementation of programming problem 2.24 from the OS textbook. The problem requests the
 * reader to implement their own version of the copy command using the POSIX API. However, we are
 * just doing this to get better with Rust.
 *
 */

use std::io::{self, Read, Write};
use std::path;
use std::{env, fs, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Usage: rust_copy source target");
        eprintln!("config build error: {}", error);
        eprintln!("env args: {:?}", env::args());
        process::exit(1);
    });
    action(config).unwrap_or_else(|error| {
        eprintln!("IO error occurred during execution: {:?}", error);
        process::exit(1);
    });
}

fn action(config: Config) -> io::Result<()> {
    let mut contents: Vec<u8> = vec![];
    fs::File::open(config.source)?.read_to_end(&mut contents)?;
    fs::File::create(config.target)?.write(&contents)?;
    Ok(())
}

struct Config {
    source: path::PathBuf,
    target: path::PathBuf,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        args.next();
        let source = match args.next() {
            Some(str) => match path::PathBuf::from(str).canonicalize() {
                Ok(val) => val,
                Err(e) => return Err(e.to_string()),
            },
            None => return Err(String::from("missing required argument `source`")),
        };
        let target = match args.next() {
            Some(str) => path::PathBuf::from(str),
            None => return Err(String::from("missing required argument `target`")),
        };



        Ok(Self { source, target })
    }
}
