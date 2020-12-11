use std::path::PathBuf;
use std::env;
use std::fs;

fn main() {
    let mut adapters: Vec<u32> = get_input();
    adapters = add_source_and_built_in_adapter(adapters);
    let task1 = calculate_product_of_diffs(&mut adapters);
    println!("Product of diffs of 1 and diffs of 3: {}", task1);
}

fn count_number_of_paths(adapters: &mut Vec<u32>) -> u64 {
    let mut count: u64 = 0;
    adapters.sort_unstable();
    for idx in 0..adapters.len() {
        count += adapters[idx..].iter().fold(0, |acc, x| {
            if x <= &(adapters[idx] + 3) {
                return 1;
            }
            return 0;
        });
    }
    return count;
}

fn iterate_over_paths(adapters: &[u32], count: &mut u64) {

    match adapters.split_first() {
        Some((x, xs)) => {
            for valid_paths in xs.iter().filter(|y| x + 3 <= y) {
                iterate_over_paths(&valid_paths, count);
            }
        },
        None => *count += 1
    }
}

fn calculate_product_of_diffs(adapters: &mut Vec<u32>) -> u32{
    adapters.sort_unstable();
    let one_diffs = count_differences_in_sorted_array(&adapters, 1);
    let three_diffs = count_differences_in_sorted_array(&adapters, 3);
    return one_diffs * three_diffs;
}

fn get_input() -> Vec<u32> {
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push("src/bin/ten/input");
    return fs::read_to_string(path)
        .expect("Could not find file")
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
}

fn add_source_and_built_in_adapter(mut adapters: Vec<u32>) -> Vec<u32> {
    adapters.push(0);
    adapters.push(*adapters.iter().max().unwrap()+3);
    return adapters;
}

fn count_differences_in_sorted_array(a: &[u32], diff: u32) -> u32 {
    let mut count = 0;
    for idx in 1..a.len() {
        if a[idx] - a[idx-1] == diff {
            count += 1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_differences_in_sorted_array() {
        let mut input: Vec<u32> = vec![0, 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 22];
        input.sort_unstable();
        assert_eq!(count_differences_in_sorted_array(&input, 1), 7);
        assert_eq!(count_differences_in_sorted_array(&input, 3), 5);
    }

    #[test]
    fn test_add_source_and_built_in_adapter() {
        let input = vec![1, 2];
        assert_eq!(add_source_and_built_in_adapter(input), vec![1,2,0,5]);
    }

    #[test]
    fn test_calculate_product_of_diffs() {
        let mut input: Vec<u32> = vec![0, 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 22];
        assert_eq!(calculate_product_of_diffs(&mut input), 7*5);
    }

    #[test]
    fn test_count_number_of_paths() {
        let mut input: Vec<u32> = vec![0, 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 22];
        assert_eq!(count_number_of_paths(&mut input), 8);
    }
}