use std::path::PathBuf;
use std::env;
use std::fs;

// struct BusAndOffset {
//     id: i32,
//     offset: i32,
// }

fn main() {
    let input = get_input();
    let timestamp = input[0].parse::<i32>().unwrap();
    let schedule = parse_schedule_ints(&input[1]);

    let (time_to_wait, bus_id) = find_earliest_departure(timestamp, &schedule);
    println!("Time to wait times bus id: {}", time_to_wait*bus_id);

    let schedule = parse_schedule(&input[1]);

    // for i in schedule.iter() {
    //     println!("{}, {}", i.0, i.1);
    // }

    println!("Contiguous time schedule: ");
    for i in find_contiguous_departure_schedule(&schedule, 100000000000000) {
        print!("{}, ", i);
    }
}

fn parse_schedule(input: &str) -> Vec<(u32, u32)> {
    let mut output = vec![];
    for (i, id) in input.split(',').enumerate() {
        if id != "x" {
            output.push((id.parse::<u32>().unwrap(), i as u32));
        }
    }
    return output;
}

// All inputs are primes
// I need a number that can be divided by the bus id

fn find_contiguous_departure_schedule(schedule: &Vec<(u32, u32)>, init_guess: u64) -> Vec<u64> {
    // let (step_size, start_value) = get_step_size_and_start_value(schedule, init_guess);

    // println!("Step size: {}, start value: {}", step_size, start_value);
    // for i in (start_value..).step_by(step_size as usize) {
    for i in init_guess.. {
        let mut it = schedule.iter().peekable();
        while it.peek() != None {
            let (bus, offset) = it.next().unwrap();
            let bus = u64::from(*bus);
            let offset = u64::from(*offset);
            // println!("Id: {}, offset: {}, rem: {}", bus, offset, (i + offset) % bus);
            if (i + offset) % bus != 0 {
                break;
            }
            else if it.peek() == None {
                // println!("DONE!");
                return schedule
                    .iter()
                    .map(|(_, offset)| i + u64::from(*offset))
                    .collect();

            }
        }
        if i % 1000000000000 == 0 {
            println!("Iteration: {}", i);
        }
        // if i > 1068781 {
        //     break;
        // }
    }
    return vec![];
}

// fn get_step_size_and_start_value(schedule: &Vec<(u32, u32)>, seed: u64) -> (u64, u64) {
//     let step_size: u64 = schedule
//         .iter()
//         .max_by(|(x,_), (y,_)| x.cmp(y))
//         .unwrap()
//         .0
//         .into();
//     let first_id: u64 = schedule[0].0.into();
//     let mut start_value: u64 = first_id;
//     for i in seed.. {
//         // std::println!("{}", i);
//         if i % first_id == 0 {
//             start_value = i - u64::from(schedule[0].1);
//             break;
//         }
//     }

//     return (step_size, start_value);
// }


fn step_until_over_limit(step: i32, limit: i32) -> i32 {
    let mut x = step;
    while x < limit {
        x += step;
    }
    return x;
}

fn find_earliest_departure(timestamp: i32, schedule: &Vec<i32>) -> (i32, i32) {
    let mut closest_departure = i32::MAX;
    let mut bus_id = 0;
    for t in schedule.iter() {
        let time_to_wait = step_until_over_limit(*t, timestamp);
        if time_to_wait < closest_departure {
            closest_departure = time_to_wait;
            bus_id = *t;
        }
    }
    return (closest_departure - timestamp, bus_id);
}

fn parse_schedule_ints(schedule: &str) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];
    for s in schedule.split(',') {
        if s != "x" {
            output.push(s.parse::<i32>().unwrap());
        }
    }
    return output;
}

fn get_input() -> Vec<String> {
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push("src/bin/thirteen/input");
    return fs::read_to_string(path)
        .expect("Could not find file")
        .split('\n')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_schedule_ints() {
        let input = "13,x,41,x,x,12";
        assert_eq!(parse_schedule_ints(&input), vec![13,41,12]);
    }

    #[test]
    fn test_find_earliest_departure() {
        let schedule = vec![7,13,59,31,19];
        assert_eq!(find_earliest_departure(939, &schedule), (5, 59));
    }

    #[test]
    fn test_step_until_over_limit() {
        let x = 59;
        let limit = 939;
        assert_eq!(step_until_over_limit(x, limit), 944);
    }

    #[test]
    fn test_find_contiguous_departure_schedule() {
        let input = vec![(7,0), (13, 1), (59,4), (31,6), (19,7)];
        let expected_output = vec![1068781, 1068782, 1068785, 1068787, 1068788];
        assert_eq!(find_contiguous_departure_schedule(&input, 1000000), expected_output);
        assert_eq!(find_contiguous_departure_schedule(&input, 1068781), expected_output);
    }

    #[test]
    fn test_parse_schedule() {
        let input = "7,13,x,x,59,x,31,19";
        let expected_output = vec![(7,0), (13, 1), (59,4), (31,6), (19,7)];
        assert_eq!(parse_schedule(input), expected_output);
    }

    // #[test]
    // fn test_get_step_size_and_start_value() {
    //     let input = vec![(7,0), (13, 1), (59,4), (31,6), (19,7)];
    //     assert_eq!(get_step_size_and_start_value(&input, 0), (59, 0));
    //     assert_eq!(get_step_size_and_start_value(&input, 4), (59, 7));
    //     assert_eq!(get_step_size_and_start_value(&input, 10), (59, 14));
    // }

}
