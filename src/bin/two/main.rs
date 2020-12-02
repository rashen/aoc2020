use std::env;
use std::path::PathBuf;
use std::convert::TryInto;

#[path = "../../file_reader.rs"] mod file_reader;

fn main() {
    let filename = "src/bin/two/input";
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push(filename);

    let mut count_sled = 0;
    let mut count_toboggan = 0;
    for l in file_reader::strings_from_file(path) {
        let (min, max, c, pw) = split_string(&l);
        if is_password_valid_at_sled_rental(min, max, c, &pw) {
            count_sled += 1;
        }
        if is_password_valid_at_toboggan(min as usize, max as usize, c, &pw) {
            count_toboggan += 1;
        }
    }
    println!("Found {} valid passwords according to sled shop standard", count_sled);
    println!("Found {} valid passwords according to toboggan rental standard", count_toboggan);
}

fn split_string(input: &str) -> (i32, i32, char, String) {
    let s: Vec<&str> =
        input.split(|c: char| c.is_whitespace() || c == '-' || c == ':')
             .collect();

    return (s[0].parse::<i32>().unwrap(),
            s[1].parse::<i32>().unwrap(),
            s[2].parse::<char>().unwrap(),
            s[4].to_string());
}

fn is_password_valid_at_sled_rental(min: i32, max: i32, c: char, pw: &str) -> bool {
    return (pw.matches(c).count() >= min.try_into().unwrap())
        && (pw.matches(c).count() <= max.try_into().unwrap());
}

fn is_password_valid_at_toboggan(first: usize, second: usize, c: char, pw: &str) -> bool {
    return (pw.chars().nth(first-1).unwrap() == c) ^
            (pw.chars().nth(second-1).unwrap() == c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        let input = String::from("1-3 b: cdefg");
        let expected = (1, 3, 'b', String::from("cdefg"));
        assert_eq!(split_string(&input), expected);
    }

    #[test]
    fn is_password_valid_at_sled_rental_true() {
        let input = (1, 1, 'c', "abc");
        assert_eq!(is_password_valid_at_sled_rental(input.0, input.1, input.2, input.3), true);
    }

    #[test]
    fn is_password_valid_at_sled_rental_max() {
        let input = (1, 1, 'c', "abcc");
        assert_eq!(is_password_valid_at_sled_rental(input.0, input.1, input.2, input.3), false);
    }

    #[test]
    fn is_password_valid_at_sled_rental_min() {
        let input = (1, 1, 'c', "ab");
        assert_eq!(is_password_valid_at_sled_rental(input.0, input.1, input.2, input.3), false);
    }

    #[test]
    fn is_password_valid_at_toboggan_one_pos() {
        let input = (1, 3, 'a', "abcde");
        assert_eq!(is_password_valid_at_toboggan(input.0, input.1, input.2, input.3), true);
    }

    #[test]
    fn is_password_valid_at_toboggan_zero_pos() {
        let input = (1, 3, 'b', "cdefg");
        assert_eq!(is_password_valid_at_toboggan(input.0, input.1, input.2, input.3), false);
    }

    #[test]
    fn is_password_valid_at_toboggan_two_pos() {
        let input = (2, 9, 'c', "ccccccccc");
        assert_eq!(is_password_valid_at_toboggan(input.0, input.1, input.2, input.3), false);
    }

}