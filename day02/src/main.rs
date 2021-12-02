use std::fs;

fn get_position(commands: &Vec<String>) -> i32 {
    let mut pos = (0, 0);
    commands.iter().for_each(|s| {
        let command: Vec<String> = s.split(' ').map(|s| s.to_string()).collect();
        let distance: i32 = command[1].parse().unwrap();
        match command[0].as_str() {
            "forward" => pos.0 += distance,
            "up" => pos.1 -= distance,
            "down" | _ => pos.1 += distance,
        }
    });

    pos.0 * pos.1
}

fn get_with_aim(commands: &Vec<String>) -> i32 {
    let mut pos = (0, 0, 0);
    commands.iter().for_each(|s| {
        let command: Vec<String> = s.split(' ').map(|s| s.to_string()).collect();
        let distance: i32 = command[1].parse().unwrap();
        match command[0].as_str() {
            "forward" => {
                pos.0 += distance;
                pos.1 += distance * pos.2
            }
            "up" => pos.2 -= distance,
            "down" | _ => pos.2 += distance,
        }
    });

    pos.0 * pos.1
}

fn main() {
    let commands: Vec<String> = fs::read_to_string("./input/day02.input")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();
    println!("{}", get_position(&commands));
    println!("{}", get_with_aim(&commands));
}
