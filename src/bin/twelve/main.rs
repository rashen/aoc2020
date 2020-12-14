use std::path::PathBuf;
use std::env;
use std::fs;

struct ShipState {
    x: i32, // east-west
    y: i32, // north-south
    dir: i32 // degrees anti-clockwise from east
}

#[derive(PartialEq,Eq,Debug)]
struct ShipAndWaypointState {
    x: i32,
    y: i32,
    x_wp: i32,
    y_wp: i32
}

fn main() {
    let input = get_input()
        .iter()
        .map(|x| parse(x))
        .collect::<Vec<(char, i32)>>();

    let last_pos = walk(&input);
    println!("The manhattan distance to last pos is: {}",
             get_manhattan_distance(last_pos.0, last_pos.1));

    let last_pos = walk_with_waypoint(&input);
    println!("If we instead follow the beacon our last pos will be: {}",
             get_manhattan_distance(last_pos.0, last_pos.1));
}

fn get_manhattan_distance(x: i32, y: i32) -> i32 {
    return x.abs() + y.abs();
}

fn walk(actions: &Vec<(char, i32)>) -> (i32, i32) {
    let mut state = ShipState {x: 0, y: 0, dir: 0};

    for a in actions.iter() {
        match a {
            ('E', val) => state.x += val,
            ('W', val) => state.x -= val,
            ('N', val) => state.y += val,
            ('S', val) => state.y -= val,
            ('F', val) => move_forward(&mut state, *val),
            ('L', val) => turn(&mut state, *val),
            ('R', val) => turn(&mut state, -(*val)),
            _ => {}
        };
    }
    return (state.x, state.y);
}

fn walk_with_waypoint(actions: &Vec<(char, i32)>) -> (i32, i32) {
    let mut state = ShipAndWaypointState {x: 0, y: 0, x_wp: 10, y_wp: 1};

    for a in actions.iter() {
        match a {
            ('E', val) => state.x_wp += val,
            ('W', val) => state.x_wp -= val,
            ('N', val) => state.y_wp += val,
            ('S', val) => state.y_wp -= val,
            ('F', val) => move_to_waypoint(&mut state, *val),
            ('L', val) => rotate_waypoint(&mut state, *val),
            ('R', val) => rotate_waypoint(&mut state, -(*val)),
            _ => {}
        }
    }

    return (state.x, state.y)
}

fn move_to_waypoint(state: &mut ShipAndWaypointState, dist: i32) {
    state.x += dist*state.x_wp;
    state.y += dist*state.y_wp;
}

fn move_forward(state: &mut ShipState, dist: i32) {
    match state.dir {
        0 => state.x += dist,
        90 => state.y += dist,
        180 => state.x -= dist,
        270 => state.y -= dist,
        _ => {}
    };
}

fn rotate_waypoint(state: &mut ShipAndWaypointState, deg: i32) {
    let input = (state.x_wp, state.y_wp);
    match deg {
        90 | -270 => {
            state.x_wp = -input.1;
            state.y_wp = input.0;
        },
        180 | -180 => {
            state.x_wp = -input.0;
            state.y_wp = -input.1;
        },
        270 | -90 => {
            state.x_wp = input.1;
            state.y_wp = -input.0;
        },
        _ => {}
    }
}

fn turn(state: &mut ShipState, deg: i32) {
    state.dir = modulo(state.dir + deg, 360);
}

fn modulo(a: i32, b: i32) -> i32 {
    return ((a % b) + b) % b;
}

fn parse(input: &str) -> (char, i32) {
    let (dir, dist) = input.split_at(1);
    return (dir.parse::<char>().unwrap(),
            dist.parse::<i32>().unwrap());
}

fn get_input() -> Vec<String> {
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push("src/bin/twelve/input");
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
    fn test_parse() {
        assert_eq!(parse("a2"), ('a', 2));
        assert_eq!(parse("N100"), ('N', 100));
        assert_eq!(parse("S9"), ('S', 9));
    }

    #[test]
    fn test_turn() {
        let mut state = ShipState{x: 0, y: 0, dir: 0};
        let expected_state = ShipState{x: 0, y: 0, dir: 90};
        turn(&mut state, 90);
        assert_eq!(state.dir, expected_state.dir);
        turn(&mut state, 360);
        assert_eq!(state.dir, expected_state.dir);
    }

    #[test]
    fn test_get_manhattan_distance() {
        assert_eq!(get_manhattan_distance(17,8), 25);
        assert_eq!(get_manhattan_distance(-4,-5), 9);
        assert_eq!(get_manhattan_distance(214,-72), 286);
    }

    #[test]
    fn test_walk_with_waypoint() {
        let actions = vec![('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)];
        assert_eq!(walk_with_waypoint(&actions), (214, -72));
    }

    #[test]
    fn test_walk_with_waypoints_rotations() {
        let actions = vec![('F', 10), ('R', 180), ('F', 20), ('L', 180), ('F', 20), ('R', 90),
                           ('R', 90), ('F', 20), ('L', 90), ('L', 90), ('F', 10)];
        assert_eq!(walk_with_waypoint(&actions), (0, 0));
    }

    #[test]
    fn test_rotate_waypoint() {
        let mut state = ShipAndWaypointState {x: 0, y: 0, x_wp: 1, y_wp: 0};
        let expected_state = ShipAndWaypointState {x: 0, y: 0, x_wp: 0, y_wp: 1};
        rotate_waypoint(&mut state, 90);
        assert_eq!(state, expected_state);
        rotate_waypoint(&mut state, 360);
        assert_eq!(state, expected_state);
    }

    #[test]
    fn test_rotate_waypoint_negative() {
        let mut state = ShipAndWaypointState {x: 170, y: 38, x_wp: 10, y_wp: 4};
        let expected_state = ShipAndWaypointState {x: 170, y: 38, x_wp: -10, y_wp: -4};
        rotate_waypoint(&mut state, -180);
        assert_eq!(state, expected_state);
    }

    #[test]
    fn test_move_to_waypoint() {
        let mut state = ShipAndWaypointState {x: 0, y: 0, x_wp: 10, y_wp: 1};
        let expected_state = ShipAndWaypointState{x: 100, y: 10, x_wp: 10, y_wp: 1};
        move_to_waypoint(&mut state, 10);
        assert_eq!(state, expected_state);
    }
}


