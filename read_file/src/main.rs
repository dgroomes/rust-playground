use std::{env, process};

use read_file::Config;

/*
  Mostly taken from https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html
  Also, I don't know how to write idiomatic comments in Rust!
*/
fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Counting lines in file {}", config.filename);

    if let Err(e) = read_file::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
