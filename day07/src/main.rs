use std::fs;

fn min_fuel(
    depths: &Vec<isize>,
    targets: impl Iterator<Item = isize>,
    dist: fn(isize, isize) -> isize,
) -> isize {
    targets
        .map(|m| depths.iter().map(|&crab| dist(m, crab)).sum())
        .min()
        .unwrap()
}

fn main() {
    let mut depths: Vec<isize> = fs::read_to_string("./input/07")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    depths.sort();
    println!(
        "{}",
        min_fuel(
            &depths,
            [depths[depths.len() / 2 - 1], depths[depths.len() / 2]].into_iter(),
            |x, y| (x - y).abs()
        )
    );
    println!(
        "{}",
        min_fuel(&depths, depths[0]..=depths[depths.len() - 1], |x, y| {
            let dist = (x - y).abs();
            dist * (dist + 1) / 2
        })
    );
}
