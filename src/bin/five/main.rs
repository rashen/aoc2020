use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let filename = "src/bin/five/input";
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push(filename);

    let boarding_passes: Vec<String> = get_lines_from_file(&path);

    let mut sorted_ids: Vec<i32> = boarding_passes.iter().map(|x| to_id(&x))
        .collect::<Vec<i32>>();
    sorted_ids.sort_unstable();

    println!("Highest id is: {}", sorted_ids.last().unwrap());

    print!("Missing values are: ");
    for e in find_missing_elements(&sorted_ids).iter() {
        print!("{}", e);
    }
    println!(); // Empty line for nicer printout
}

fn get_lines_from_file(filename: &PathBuf) -> Vec<String> {
    return fs::read_to_string(filename)
        .expect("Could not find file")
        .split("\n")
        .map(|x| String::from(x))
        .collect();
}

fn to_id(seat: &str) -> i32 {
    let (row, column) = get_row_and_column(seat);
    return (row as i32)*8 + (column as i32);
}


fn get_row_and_column(seat: &str) -> (u8, u8) {
    let in_binary = seat.chars().map(|x| to_binary(&x).unwrap()).collect::<Vec<u8>>();

    assert_eq!(in_binary.len(), 10);
    let (row, column) = in_binary.split_at(7);

    return (to_decimal(&row), to_decimal(&column));
}

fn to_decimal(bin: &[u8]) -> u8 {
    let mut exp = 0;
    let mut sum = 0;
    for b in bin.iter().rev() {
        sum += b * 2u8.pow(exp);
        exp += 1;
    }

    return sum;
}

fn to_binary(c: &char) -> Option<u8> {
    return match c {
        'F' => Some(0),
        'L' => Some(0),
        'B' => Some(1),
        'R' => Some(1),
        _  => None
    }
}

fn find_missing_elements(ids: &Vec<i32>) -> Vec<i32> {
    let mut prev_val: i32 = ids[0] - 1;
    let mut output: Vec<i32> = vec![];
    for e in ids.iter() {
        let diff = e - prev_val;
        if diff > 1 {
            output.push(e-1)
        }
        prev_val = *e;
    }
    return output;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_id() {
        assert_eq!(to_id("FBFBBFFRLR"), 357);
        assert_eq!(to_id("BFFFBBFRRR"), 567);
        assert_eq!(to_id("FFFBBBFRRR"), 119);
        assert_eq!(to_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_get_row_and_column() {
        assert_eq!(get_row_and_column("FBFBBFFRLR"), (44, 5));
        assert_eq!(get_row_and_column("BFFFBBFRRR"), (70, 7));
        assert_eq!(get_row_and_column("FFFBBBFRRR"), (14, 7));
        assert_eq!(get_row_and_column("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn test_to_binary() {
        assert_eq!(to_binary(&'F'), Some(0));
        assert_eq!(to_binary(&'B'), Some(1));
        assert_eq!(to_binary(&'L'), Some(0));
        assert_eq!(to_binary(&'R'), Some(1));
    }

    #[test]
    fn test_to_decimal() {
        assert_eq!(to_decimal(&vec![0, 1, 0, 1, 1, 0, 0]), 44);
        assert_eq!(to_decimal(&vec![1, 0, 1]), 5);
    }

    #[test]
    fn test_find_missing_elements() {
        assert_eq!(find_missing_elements(&vec![1,2,3,5]), vec![4]);
        assert_eq!(find_missing_elements(&vec![1,3,5]), vec![2,4]);
    }
}
