use std::fs;

fn compute_gap(depths: &Vec<i32>, gap: usize) -> i32 {
    depths
        .iter()
        .enumerate()
        .map(|(i, val)| {
            if i < depths.len() - gap && val < &depths[i + gap] {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let depths: Vec<i32> = fs::read_to_string("./input/day01.input")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{}", compute_gap(&depths, 1));
    println!("{}", compute_gap(&depths, 3));
}
