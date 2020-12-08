use std::path::PathBuf;
use std::env;
use std::fs;


#[derive(Eq, PartialEq, Debug)]
struct Instruction {
    op: String,
    val: i32
}

fn main() {
    let instruction_set = get_instruction_set();

    let last_value = emulate_until_loop_found(instruction_set);
    println!("Last value in acc before looping: {}", last_value);

}

fn emulate_until_loop_found(instructions: Vec<Instruction>) -> i32 {
    let mut acc: i32 = 0;
    let mut index: usize = 0;
    let mut visited_indices: Vec<usize> = vec![];
    loop {
        if index >= instructions.len() {
            break;
        }

        match instructions[index].op.as_str() {
            "acc" => {
                acc += instructions[index].val;
                index += 1;
            },
            "jmp" => {
                let new_index: i32 = index as i32 + instructions[index].val;
                index = new_index as usize;
            },
            "nop" => {
                index += 1;
            }
            &_ => {
                println!{"Could not find instruction"};
            }
        }

        if visited_indices.iter().any(|&x| x == index) {
            println!("Found loop at index {}", index);
            return acc;

        }
        visited_indices.push(index);
    }
    return acc
}

fn get_instruction_set() -> Vec<Instruction>{
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push("src/bin/eight/input");
    return fs::read_to_string(path)
        .expect("Could not find file")
        .split("\n")
        .map(|x| read_input(x))
        .collect::<Vec<Instruction>>();
}

fn read_input(input: &str) -> Instruction {
    let (op, val) = input.split_at(3);
    return Instruction{op: String::from(op),
                       val: val.trim().parse::<i32>().unwrap()};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        assert_eq!(read_input("acc +42"), Instruction{op: String::from("acc"), val: 42});
        assert_eq!(read_input("nop -99"), Instruction{op: String::from("nop"), val: -99});
    }

    #[test]
    fn test_emulate_until_loop_found() {
        assert_eq!(emulate_until_loop_found(vec![Instruction{op: String::from("nop"), val: 0},
                                                 Instruction{op: String::from("acc"), val: 1},
                                                 Instruction{op: String::from("jmp"), val: 4},
                                                 Instruction{op: String::from("acc"), val: 3},
                                                 Instruction{op: String::from("jmp"), val: -3},
                                                 Instruction{op: String::from("acc"), val: -99},
                                                 Instruction{op: String::from("acc"), val: 1},
                                                 Instruction{op: String::from("jmp"), val: -4},
                                                 Instruction{op: String::from("acc"), val: 6}]),
                                            5);
    }
}