#![allow(non_snake_case)]
mod instruction;
mod parse;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");

    let reader = BufReader::new(file);
    for line in reader.lines() {

        let line = line.expect("Failed to read line");
        //TODO: better comment truncate
        if line.starts_with('#') {
            println!("Ignore Comment: {}", line);
            continue;
        }

        

    }
}

