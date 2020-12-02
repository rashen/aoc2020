use std::fs;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

pub fn ints_from_file(filename: PathBuf) -> Vec<i32> {
    let file = fs::File::open(filename)
        .expect("Could not find file");
    let mut output: Vec<i32> = Vec::new();
    let buf = BufReader::new(file);
    for line in buf.lines() {
        let val = line
            .unwrap()
            .parse::<i32>()
            .unwrap();
        output.push(val);
    }

    return output;
}
