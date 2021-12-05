use std::cmp::max;
use std::collections::HashMap;
use std::fs;

fn num_intersect(positions: &Vec<String>, with_diag: bool) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    positions.iter().fold(0, |s, line| {
        let split: Vec<i32> = line
            .split(" -> ")
            .flat_map(|x| x.split(','))
            .map(|x| x.parse().unwrap())
            .collect();
        let (x0, y0, x1, y1) = (split[0], split[1], split[2], split[3]);
        if x0 != x1 && y0 != y1 && !with_diag {
            return s;
        }
        let dist = max((x0 - x1).abs(), (y0 - y1).abs());
        let (dx, dy) = ((x1 - x0) / dist, (y1 - y0) / dist);
        s + (0..=dist).fold(0, |acc, i| {
            let pos = (x0 + i * dx, y0 + i * dy);
            let count = map.get_mut(&pos);
            return match count {
                Some(c) => {
                    *c += 1;
                    acc + (c == &1) as i32
                }
                None => {
                    map.insert(pos, 0);
                    acc
                }
            };
        })
    })
}

fn main() {
    let input: Vec<String> = fs::read_to_string("./input/05")
        .unwrap()
        .trim()
        .lines()
        .map(|x| x.to_string())
        .collect();
    println!("{}", num_intersect(&input, false));
    println!("{}", num_intersect(&input, true));
}
