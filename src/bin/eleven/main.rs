use std::convert::TryInto;
use std::path::PathBuf;
use std::env;
use std::fs;

// Rules:
// * If there are no occupied seats adjacents, take the seat
// * If a seat is occupied and four or more seats adjacent to
//   it are also occupied, the seat becomes empty
// * Other seats don't change

fn main() {
    let mut seats = get_input();
    // for row in seats.iter() {
    //     for s in row.iter() {
    //         print!("{}", s);
    //     }
    //     println!("");
    // }



}

fn get_input() -> Vec<Vec<char>> {
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push("src/bin/eleven/input");
    return fs::read_to_string(path)
        .expect("Could not find file")
        .split("\n")
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
}

fn get_adjacent_indices(idx: &(usize, usize), size: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = vec![];

    let x = idx.0 as i32;
    let y = idx.1 as i32;

    let max_height = size.0 as i32;
    let max_width = size.1 as i32;

    for i in [-1, 0, 1].iter() {
        for j in [-1, 0, 1].iter() {
            if (i,j) == (&x,&y) || i >= &max_height || j >= &max_width {
                continue;
            }

            match ((x + i).try_into(), (y + j).try_into()) {
                (Ok(adj_x), Ok(adj_y)) => output.push((adj_x, adj_y)),
                (Err(_), _) => {},
                (_, Err(_)) => {}
            }
        }
    }

    return output;
}

fn count_adjacent_occupied_seats(seats: &Vec<Vec<char>>, idx: &(usize, usize)) -> u8 {
    // let empty_seat = 'L';
    // let floor = '.';
    let occupied_seat = '#';

    let size = (seats.len(), seats[0].len());
    let n_occupied_seats = get_adjacent_indices(idx, &size).iter().fold(0, |acc, x| {
        println!("{}, {}", x.0, x.1);
        if seats[x.0][x.1] == occupied_seat {
            return acc + 1;
        }
        return acc + 0
    });


    return n_occupied_seats;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_adjacent_occupied_seats() {
        let seats = vec![vec!['.', 'L', 'L', 'L'],
                         vec!['L', 'L', 'L', 'L'],
                         vec!['#', 'L', 'L', 'L']];
        assert_eq!(count_adjacent_occupied_seats(&seats, &(1,1)), 1);
        assert_eq!(count_adjacent_occupied_seats(&seats, &(1,2)), 0);
    }

    #[test]
    fn test_get_adjacent_indices() {
        assert_eq!(get_adjacent_indices(&(1,1), &(3,3)).sort(),
                   vec![(0, 0), (0, 1), (0, 2), (1, 0), (1,2), (2, 0), (2,1), (2,2)].sort());
        assert_eq!(get_adjacent_indices(&(2,2), &(3,3)).sort(),
                   vec![(1,1), (1,2), (2,1)].sort());
    }
}

