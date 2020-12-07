use regex::Regex;
use std::path::PathBuf;
use std::env;
use std::fs;

fn main() {
    let inputs: Vec<String> = get_inputs();

    for e in inputs.iter()  {
        println!("{}", e);
    }
}

fn get_inputs() -> Vec<String> {
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push("src/bin/seven/input");
    return fs::read_to_string(path)
        .expect("Could not find file")
        .split("\n")
        .map(|x| String::from(x))
        .collect();
}

fn parse_one_line(input: &str, general_pattern: &Regex, trailing_pattern: &Regex) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for e in general_pattern.captures_iter(input) {
        match e.get(1) {
            Some(val) => output.push(String::from(val.as_str())),
            None => println!("Parse error: could not find first bag")
        }
        match e.get(2) {
            Some(val) => output.append(&mut trailing_bag_parser(val.as_str(), &trailing_pattern)),
            None => println!("Parse error: could not find trailing bags")
        }
    }

    return output;
}

fn trailing_bag_parser(input: &str, re: &Regex) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    for e in re.captures_iter(input) {
        match e.get(1) {
            Some(val) => output.push(String::from(val.as_str())),
            None => {}
        };
    }
    return output;
}

fn get_general_pattern() -> Regex {
    return Regex::new(r"(\w+\s\w+)\sbags\scontain\s(.+).").unwrap();
}

fn get_trailing_pattern() -> Regex {
    return Regex::new(r"(\d+\s\w+\s\w+)(?:\sbag)").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_one_line() {
        let general_pattern = get_general_pattern();
        let trailing_pattern = get_trailing_pattern();
        assert_eq!(parse_one_line(&"faded blue bags contain no other bags.", &general_pattern, &trailing_pattern),
                   vec!["faded blue"]);
        assert_eq!(parse_one_line(&"light red bags contain 1 bright white bag, 2 muted yellow bags.", &general_pattern, &trailing_pattern),
                   vec!["light red", "1 bright white", "2 muted yellow"]);
        assert_eq!(parse_one_line(&"light aqua bags contain 2 muted plum bags, 5 mirrored bronze bags, 4 striped coral bags, 1 posh violet bag.", &general_pattern, &trailing_pattern),
                   vec!["light aqua", "2 muted plum", "5 mirrored bronze", "4 striped coral", "1 posh violet"]);
    }

    #[test]
    fn test_trailing_bag_parser() {
        let re = get_trailing_pattern();
        assert_eq!(trailing_bag_parser(&"1 bright white bag, 2 muted yellow bags.", &re),
                   vec!["1 bright white", "2 muted yellow"]);
        assert_eq!(trailing_bag_parser(&"2 muted plum bags, 5 mirrored bronze bags, 4 striped coral bags, 1 posh violet bag.", &re),
                   vec!["2 muted plum", "5 mirrored bronze", "4 striped coral", "1 posh violet"]);
        assert!(trailing_bag_parser(&"no other bags.", &re).is_empty());
    }

}