/*
 * Program Notes
 * ----------------
 * Rust implementation of programming problem 2.24 from the OS textbook. The problem requests the
 * reader to implement their own version of the copy command using the POSIX API. However, we are
 * just doing this to get better with Rust.
 *
 */
use rusty_unix_cp;
use std::{env, io, process};

fn main() {
    let config = rusty_unix_cp::Config::build(env::args().collect()).unwrap_or_else(|error| {
        eprintln!("Usage: rusty_unix_cp source_a [source_b, ...] target");
        eprintln!("config build error: {}", error);
        eprintln!("env args: {:?}", env::args());
        process::exit(1);
    });

    run(config).unwrap_or_else(|error| {
        eprintln!("IO error occurred during execution: {:?}", error);
        process::exit(1);
    });
}

fn run(mut config: rusty_unix_cp::Config) -> io::Result<()> {
    let mut sources = config
        .sources
        .take()
        .expect("sources was `None`")
        .into_iter();
    let target = config.target.take().expect("target was `None`");
    match config.target_isdir {
        true => rusty_unix_cp::recursive_copy(sources, target),
        false => {
            let source_file = match sources.next().expect("expected filename") {
                Ok(val) => val,
                Err(e) => panic!("shouldn't be able to reach here: {:?}", e.to_string()),
            };
            rusty_unix_cp::copy(source_file, target)
        }
    }
}
