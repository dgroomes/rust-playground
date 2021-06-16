use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.filename)?;
    let line_count = count_lines(file);

    println!("Line count: {}", line_count);

    Ok(())
}

fn count_lines(file: File) -> u32 {
    let reader = BufReader::new(file);

    let mut line_count: u32 = 0;
    for _line in reader.lines() {
        line_count = line_count + 1;
    }

    line_count
}
