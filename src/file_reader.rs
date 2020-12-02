use std::fs;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

pub fn strings_from_file(filename: PathBuf) -> Vec<String> {
    let file = fs::File::open(filename)
        .expect("Could not find file");
    let mut output: Vec<String> = Vec::new();
    let buf = BufReader::new(file);
    for line in buf.lines() {
        let val = line.unwrap();
        output.push(val);
    }
    return output;
}

pub fn ints_from_file(filename: PathBuf) -> Vec<i32> {
    let lines = strings_from_file(filename);
    let mut output: Vec<i32> = Vec::new();
    for line in lines {
        output.push(line.parse::<i32>().unwrap());
    }
    return output;
}
