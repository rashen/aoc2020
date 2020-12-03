use std::convert::TryInto;
use std::env;
use std::path::PathBuf;

#[path = "../../file_reader.rs"] mod file_reader;

fn main() {
    let filename = "src/bin/three/input";
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push(filename);

    let map = file_reader::strings_from_file(path);

    let step_lens: Vec<(i32, i32)> = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];
    let mut product_of_trees: i64 = 1;
    for s in step_lens {
        let tree_count: i64 = count_step_and_wrap(&map, s) as i64;
        product_of_trees *= tree_count;
        println!("We will hit {} trees using step length ({}, {})", tree_count, s.0, s.1);
    }
    println!("The product of all trees hit is {}", product_of_trees);
}

fn count_step_and_wrap(map: &Vec<String>, step_length: (i32, i32)) -> i32 {

    let mut horizontal_pos: i32 = 0;
    let mut tree_count: i32 = 0;

    let mut map_iter = map.iter().step_by(step_length.1 as usize);
    map_iter.next(); // Step over first
    loop {
        let line = match map_iter.next() {
            Some(val) => val,
            None => break,
        };

        horizontal_pos = modulo(horizontal_pos + step_length.0, line.len() as i32);
        if line.chars().nth(horizontal_pos.try_into().unwrap()).unwrap() == '#' {
            tree_count += 1;
        }
    }
    return tree_count;
}

fn modulo(a: i32, b: i32) -> i32 {
    return ((a % b) + b) % b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_one_tree() {
        let input = vec![String::from("...."),
                         String::from("...#")];
        let step_length: (i32, i32) = (3,1);
        assert_eq!(count_step_and_wrap(&input, step_length), 1);
    }

    #[test]
    fn test_count_two_trees() {
        let input = vec![String::from("........"),
                         String::from("...#...."),
                         String::from("......#.")];
        let step_length: (i32, i32) = (3,1);
        assert_eq!(count_step_and_wrap(&input, step_length), 2);
    }

    #[test]
    fn test_count_one_tree_wrapping() {
        let input = vec![String::from("...."),
                         String::from("...."),
                         String::from("..#.")];
        let step_length: (i32, i32) = (3,1);
        assert_eq!(count_step_and_wrap(&input, step_length), 1);
    }

    #[test]
    fn test_modulo() {
        let a = 6;
        let b = 4;
        let expected = 2;
        assert_eq!(modulo(a,b), expected);
    }
}