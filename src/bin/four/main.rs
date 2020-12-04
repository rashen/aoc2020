use std::env;
use std::path::PathBuf;
use regex::Regex;
use std::fs;

fn read_until_empty_line(filename: &PathBuf) -> Vec<String> {
    return fs::read_to_string(filename)
        .expect("Could not find file")
        .split("\n\n")
        .map(|x| String::from(x))
        .collect();
}

fn main() {
    let filename = "src/bin/four/input";
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push(filename);

    let count: i32 = count_passports_with_valid_keys(&read_until_empty_line(&path));
    println!("Number of passports with matching keys: {}", count);

    let count: i32 = count_valid_passports(&read_until_empty_line(&path));
    println!("Number of valid passports: {}", count);
}

fn count_passports_with_valid_keys(passports: &Vec<String>) -> i32 {
    let re = get_key_pattern_except_cid();
    return passports.iter().fold(0, |acc, x|
        acc + match check_all_keys_present(&x, &re) {
            true => 1,
            false => 0
    });
}

fn get_valid_key_passport_indices(passports: &Vec<String>) -> Vec<usize> {
    let re = get_value_pattern();
    let mut valid_indices: Vec<usize> = vec![];

    for i in 0..passports.len() {
        if check_all_keys_present(&passports[i], &re) {
            valid_indices.push(i);
        }
    }
    return valid_indices;
}

fn count_valid_passports(passports: &Vec<String>) -> i32 {
    let valid_indices = get_valid_passport_indices(passports);
    let mut passports_with_valid_keys: Vec<String> = vec![];

    for i in valid_indices {
        passports_with_valid_keys.push(String::from(&passports[i]));
    }

    return passports_with_valid_keys.iter().fold(0, |acc, x|
        acc + match check_all_passport_values(x) {
            true => 1,
            false => 0
        });
}

fn get_valid_passport_indices(passports: &Vec<String>) -> Vec<usize> {
    let valid_key_indices = get_valid_key_passport_indices(passports);
    return valid_key_indices;
}

fn check_all_passport_values(passport: &str) -> bool {
    let re = get_value_pattern();
    return re.captures_iter(passport).fold(true, |acc, x| acc && check_value(x.get(0).unwrap().as_str()));
}

fn check_value(field: &str) -> bool {
    if field.starts_with("pid") {
        return check_passport_id(&field[4..field.len()]);
    }
    else if field.starts_with("hgt") {
        return check_height(&field[4..field.len()]);
    }
    else if field.starts_with("ecl") {
        return check_eye_colour(&field[4..field.len()]);
    }
    else if field.starts_with("iyr") {
        return check_issue_year(&field[4..field.len()]);
    }
    else if field.starts_with("eyr") {
        return check_expiration_year(&field[4..field.len()]);
    }
    else if field.starts_with("byr") {
        return check_birth_year(&field[4..field.len()]);
    }
    else if field.starts_with("hcl") {
        return check_hair_color(&field[4..field.len()]);
    }
    return false;
}

fn check_birth_year(byr: &str) -> bool {
    let birth_year = byr.parse::<i32>().unwrap();
    return birth_year >= 1920 && birth_year <= 2002;
}

fn check_issue_year(iyr: &str) -> bool {
    let issue_year = iyr.parse::<i32>().unwrap();
    return issue_year >= 2010 && issue_year <= 2020;
}

fn check_expiration_year(eyr: &str) -> bool {
    let expiration_year = eyr.parse::<i32>().unwrap();
    return expiration_year >= 2020 && expiration_year <= 2030;
}

fn check_height(hgt: &str) -> bool {
    if hgt.ends_with("in") {
        let height = hgt[0..hgt.len()-2].parse::<i32>().unwrap();
        return height >= 59 && height <= 76;
    }
    else if hgt.ends_with("cm") {
        let height = hgt[0..hgt.len()-2].parse::<i32>().unwrap();
        return height >= 150 && height <= 193;
    }
    return false;
}

fn check_hair_color(hcl: &str) -> bool {
    let re = get_hair_colour_pattern();
    return match re.captures(hcl) {
        Some(_) => true,
        None => false
    };
}

fn check_eye_colour(ecl: &str) -> bool {
    let valid_eye_colours: Vec<&str> = vec!["amb","blu","brn","gry","grn","hzl","oth"];
    return valid_eye_colours.iter().any(|&x| x == ecl);
}

fn check_passport_id(pid: &str) -> bool {
    let re = get_passport_id_pattern();
    return match re.captures(pid) {
        Some(_) => true,
        None => false
    };
}

fn get_value_pattern() -> Regex {
    return Regex::new(r"(ecl:\S+)|(pid:\d+)|(eyr:\d+)|(hcl:\#\S+)|(byr:\d+)|(iyr:\d+)|(hgt:\d+cm|hgt:\d+in)").unwrap();
}

fn get_key_pattern_except_cid() -> Regex {
    return Regex::new(r"(?P<ecl>ecl)|(?P<pid>pid)|(?P<eyr>eyr)|(?P<hcl>hcl)|(?P<byr>byr)|(?P<iyr>iyr)|(?P<hgt>hgt)").unwrap();
}

fn get_hair_colour_pattern() -> Regex {
    return Regex::new(r"^\#[a-f0-9]{6}$").unwrap();
}

fn get_passport_id_pattern() -> Regex {
    return Regex::new(r"^[0-9]{9}$").unwrap();
}

fn check_all_keys_present(s: &str, re: &Regex) -> bool {
    let key_count = 7;
    return re.captures_iter(s).count() >= key_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_value() {
        let re = get_value_pattern();
        let test_string = "ecl:gry";
        let caps = re.captures(test_string).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "ecl:gry");
    }

    #[test]
    fn test_all_keys_present() {
        let re = get_key_pattern_except_cid();
        let test_string = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
        assert_eq!(check_all_keys_present(&test_string, &re), true);
    }

    #[test]
    fn test_missing_keys() {
        let re = get_key_pattern_except_cid();
        let test_string = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929";
        assert_eq!(check_all_keys_present(&test_string, &re), false);
    }

    #[test]
    fn test_count_passports_with_valid_keys() {
        let test_passport = vec![String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm")];
        assert_eq!(count_passports_with_valid_keys(&test_passport), 1);
    }

    #[test]
    fn test_get_valid_key_passport_indices() {
        {
            let test_passport = vec![String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929")];
            let indices = get_valid_key_passport_indices(&test_passport);
            assert!(indices.is_empty());
        }
        {
            let test_passport = vec![String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm")];
            let indices = get_valid_key_passport_indices(&test_passport);
            assert!(!indices.is_empty());
            assert_eq!(indices[0], 0);
        }
    }

    #[test]
    fn test_check_birth_year() {
        assert_eq!(check_birth_year("1900"), false);
        assert_eq!(check_birth_year("2022"), false);
        assert_eq!(check_birth_year("1991"), true);
    }

    #[test]
    fn test_check_height() {
        assert_eq!(check_height("60in"), true);
        assert_eq!(check_height("190cm"), true);
        assert_eq!(check_height("190in"), false);
        assert_eq!(check_height("190"), false);
    }

    #[test]
    fn test_check_hair_colour() {
        assert_eq!(check_hair_color("#123abc"), true);
        assert_eq!(check_hair_color("#123abz"), false);
        assert_eq!(check_hair_color("123abc"), false);
    }

    #[test]
    fn test_check_eye_colour() {
        assert_eq!(check_eye_colour("brn"), true);
        assert_eq!(check_eye_colour("wat"), false);
    }

    #[test]
    fn test_passport_id() {
        assert_eq!(check_passport_id("000000001"), true);
        assert_eq!(check_passport_id("0123456789"), false);
    }

    #[test]
    fn test_check_all_passport_values() {
        assert_eq!(check_all_passport_values("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"), true);
        assert_eq!(check_all_passport_values("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"), true);
        assert_eq!(check_all_passport_values("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"), true);
        assert_eq!(check_all_passport_values("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"), true);
        assert_eq!(check_all_passport_values("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"), false);
        assert_eq!(check_all_passport_values("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"), false);
        assert_eq!(check_all_passport_values("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"), false);
    }

}
