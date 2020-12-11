use std::path::PathBuf;
use std::env;
use std::fs;

fn main() {
    let input = get_input();

    let missing_value = find_missing_value(&input, 25);
    println!("The first non-compliant value is {}",
             missing_value);

    let summing_set = find_continguous_set_summing_to(&input, missing_value);
    let encryption_weakness = summing_set.iter().min().unwrap() + summing_set.iter().max().unwrap();

    println!("The first and last value of the summing set sums to {}", encryption_weakness)
}

fn find_continguous_set_summing_to(all_values: &Vec<i64>, v: i64) -> &[i64] {
    let mut first_idx = 0;
    let mut last_idx;
    let mut sum;
    loop {
        sum = all_values[first_idx];
        last_idx = first_idx + 1;
        loop {
            sum += all_values[last_idx];
            if sum == v {
                return &all_values[first_idx..last_idx+1];
            }
            else if sum > v {
                break;
            }

            last_idx += 1;
            if last_idx > all_values.len()-1 {
                break;
            }
        }
        first_idx += 1;
        if first_idx > all_values.len()-1 {
            break;
        }
    }
    return &all_values[0..0];
}

fn find_missing_value(all_values: &Vec<i64>, preamble_len: usize) -> i64 {
    for i in preamble_len..all_values.len()-1 {
        if !get_all_two_addend_sums(&all_values[(i-preamble_len)..i])
            .iter()
            .any(|&x: &i64| x == all_values[i]) {
                return all_values[i];
            }
    }
    return 0;
}

fn get_input() -> Vec<i64> {
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push("src/bin/nine/input");
    return fs::read_to_string(path)
        .expect("Could not find file")
        .split("\n")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
}

fn get_all_two_addend_sums(addends: &[i64]) -> Vec<i64>{
    let mut sums: Vec<i64> = vec![];
    for i in 0..addends.len() {
        for j in (i+1)..addends.len() {
            sums.push(addends[i]+addends[j]);
        }
    }
    sums.sort();
    sums.dedup();
    return sums;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_two_addend_sums() {
        let input = vec![1,2,3,4,5];
        let expected_output = vec![3,4,5,6,7,8,9];
        assert_eq!(get_all_two_addend_sums(&input),expected_output);
    }

    #[test]
    fn test_find_missing_value() {
        let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        assert_eq!(find_missing_value(&input, 5), 127);
    }

    #[test]
    fn test_find_continguous_set_summing_to() {
        let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        assert_eq!(find_continguous_set_summing_to(&input, 127), &[15,25,47,40]);
    }
}