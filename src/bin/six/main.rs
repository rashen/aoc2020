use std::collections::BTreeMap;
use std::path::PathBuf;
use std::env;
use std::fs;

fn main() {
    let answers_per_group = get_answers_per_group_from_file();

    let mut yes_anyone_count: u32 = 0;
    let mut yes_everyone_count: u32 = 0;
    let bit_map = get_bit_map();
    for group in answers_per_group.iter() {
        yes_anyone_count += count_yes_from_anyone_per_group(&group.split('\n').collect(), &bit_map);
        yes_everyone_count += count_yes_from_everyone_per_group(&group.split('\n').collect(), &bit_map);
    }
    std::println!("The total number of yes answers per group, from anyone, is {}", yes_anyone_count);
    std::println!("The total number of yes answers per group, from everyone, is {}", yes_everyone_count);
}

fn get_answers_per_group_from_file() -> Vec<String> {
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push("src/bin/six/input");
    return fs::read_to_string(path)
        .expect("Could not find file")
        .split("\n\n")
        .map(|x| String::from(x))
        .collect();
}

fn count_yes_from_anyone_per_group(answers: &Vec<&str>, bit_map: &BTreeMap<char, u32>) -> u32 {
    let mut merged_answers: u32 = 0;
    for a in answers.iter() {
        merged_answers |= to_bits(a, bit_map);
    }
    return merged_answers.count_ones();
}

fn count_yes_from_everyone_per_group(answers: &Vec<&str>, bit_map: &BTreeMap<char, u32>) -> u32 {
    let mut merged_answers: u32 = to_bits(answers[0], bit_map);
    for a in answers.iter() {
        merged_answers &= to_bits(a, bit_map);
    }
    return merged_answers.count_ones();
}

fn to_bits(input: &str, bit_map: &BTreeMap<char, u32>) -> u32 {
    let mut output: u32 = 0;

    for c in input.chars() {
        match bit_map.get_key_value(&c) {
            Some((_, val)) => output |= val,
            None => {},
        }
    }

    return output;
}

fn get_bit_map() -> BTreeMap<char, u32> {
    let alphabet: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut bit_map = BTreeMap::new();
    let mut power: u32 = 26;
    for c in alphabet.chars() {
        power -= 1;
        bit_map.insert(c, 2u32.pow(power));
    }
    return bit_map;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bits() {
        let all_yes: u32 = 0b0000_0011_1111_1111_1111_1111_1111_1111;
        println!("{}", 0b0000_0010_0000_0000_0000_0000_0000_0000);
        assert_eq!(to_bits("abcdefghijklmnopqrstuvwxyz", &get_bit_map()), all_yes);
    }

    #[test]
    fn test_get_bit_map() {
        let bit_map = get_bit_map();
        assert_eq!(bit_map.get_key_value(&'a'), Some((&'a', &2u32.pow(25))));
        assert_eq!(bit_map.get_key_value(&'z'), Some((&'z', &1u32)));
    }

    #[test]
    fn test_count_yes_from_anyone_per_group() {
        let bit_map = get_bit_map();
        let input = vec!["ab", "ac"];
        assert_eq!(count_yes_from_anyone_per_group(&input, &bit_map), 3);
    }

    #[test]
    fn test_count_yes_from_everyone_per_group() {
        let bit_map = get_bit_map();
        assert_eq!(count_yes_from_everyone_per_group(&vec!["abc"], &bit_map), 3);
        assert_eq!(count_yes_from_everyone_per_group(&vec!["a", "b", "c"], &bit_map), 0);
        assert_eq!(count_yes_from_everyone_per_group(&vec!["ab", "ac"], &bit_map), 1);
        assert_eq!(count_yes_from_everyone_per_group(&vec!["a", "a", "a"], &bit_map), 1);
        assert_eq!(count_yes_from_everyone_per_group(&vec!["b"], &bit_map), 1);
    }
}