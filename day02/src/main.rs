use std::fs;

fn get_position(commands: &Vec<String>, with_aim: bool) -> i32 {
    let pos = commands.iter().fold((0, 0, 0), |(x, y, a), s| {
        let command: Vec<String> = s.split(' ').map(|s| s.to_string()).collect();
        let d: i32 = command[1].parse().unwrap();
        match command[0].as_str() {
            "forward" => (x + d, y + d * a, a),
            "up" => (x, y, a - d),
            "down" | _ => (x, y, a + d),
        }
    });
    if with_aim {
        pos.0 * pos.1
    } else {
        pos.0 * pos.2
    }
}

fn main() {
    let commands: Vec<String> = fs::read_to_string("./input/day02.input")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();
    println!("{}", get_position(&commands, false));
    println!("{}", get_position(&commands, true));
}
