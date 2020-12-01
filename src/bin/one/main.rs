use std::fs;
use std::env;
use std::path::PathBuf;
use std::io::{BufReader, BufRead};

fn main() {
    let filename = "src/bin/one/input";
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push(filename);

    let contents = lines_from_file(path);

    println!("Product with two addends is {}",
             find_product_of_two_addends_to_sum(&contents, 2020));

    println!("Product with three addends is {}",
              find_product_of_three_addends_to_sum(&contents, 2020));
}

fn lines_from_file(filename: PathBuf) -> Vec<i32> {
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

fn find_product_of_two_addends_to_sum(seq: &Vec<i32>, sum: i32) -> i32 {

    for outer in seq.iter() {
        for inner in seq.iter() {
            if inner + outer == sum {
                return inner * outer;
            }
        }
    }

    return 0;
}

fn find_product_of_three_addends_to_sum(seq: &Vec<i32>, sum: i32) -> i32 {

    for outer in seq.iter() {
        for middle in seq.iter() {
            for inner in seq.iter() {
                if inner + middle + outer == sum {
                    return inner * middle * outer;
                }
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_product_of_two_addends_to_sum() {
        let sequence: Vec<i32> = vec![1, 3, 5, 7, 13];
        let value: i32 = 16;
        let expected: i32 = 3 * 13;
        assert_eq!(
            find_product_of_two_addends_to_sum(&sequence, value),
            expected
        );
    }

    #[test]
    fn test_find_product_of_two_addends_to_sum_empty() {
        let sequence: Vec<i32> = vec![1, 3, 5, 7, 13];
        let value: i32 = 1;
        let expected: i32 = 0;
        assert_eq!(
            find_product_of_two_addends_to_sum(&sequence, value),
            expected
        );
    }

    #[test]
    fn test_find_product_of_three_addends_to_sum() {
        let sequence: Vec<i32> = vec![1, 3, 5, 7, 13];
        let value: i32 = 23;
        let expected: i32 = 3 * 7 * 13;
        assert_eq!(
            find_product_of_three_addends_to_sum(&sequence, value),
            expected
        );
    }

    #[test]
    fn test_find_product_of_three_addends_to_sum_empty() {
        let sequence: Vec<i32> = vec![1, 3, 5, 7, 13];
        let value: i32 = 1;
        let expected: i32 = 0;
        assert_eq!(
            find_product_of_three_addends_to_sum(&sequence, value),
            expected
        );
    }

}