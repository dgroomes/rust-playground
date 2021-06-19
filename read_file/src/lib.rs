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

    println!("Number of lines: {}", line_count);

    Ok(())
}

fn count_lines(file: File) -> u32 {
    let reader = BufReader::new(file);

    let mut line_count: u32 = 0;
    for line in reader.lines() {
        match line {
            Ok(_) => {
                line_count = line_count + 1;
            }
            Err(_) => panic!("Found error while reading a line from the file")
        }
    }

    line_count
}
