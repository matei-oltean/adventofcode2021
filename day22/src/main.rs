use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;

fn op<T: std::cmp::Eq + std::hash::Hash>(op: &str, pt: T, set: &mut HashSet<T>) {
    if op == "on" {
        set.insert(pt);
    } else {
        set.remove(&pt);
    }
}

fn get_box_1(instructions: &[String]) -> usize {
    let regex = Regex::new(r"^(on|off) x=(-?[0-9]*)\.\.(-?[0-9]*),y=(-?[0-9]*)\.\.(-?[0-9]*),z=(-?[0-9]*)\.\.(-?[0-9]*)$").unwrap();
    let mut res = HashSet::new();
    for instruction in instructions {
        let captures = regex.captures(instruction).unwrap();
        let points: Vec<i32> = (2..=7).map(|i| captures[i].parse().unwrap()).collect();
        (max(points[0], -50)..=min(points[1], 50)).for_each(|x| {
            (max(points[2], -50)..=min(points[3], 50)).for_each(|y| {
                (max(points[4], -50)..=min(points[5], 50)).for_each(|z| {
                    op(&captures[1], (x, y, z), &mut res);
                });
            });
        });
    }
    res.len()
}

fn main() {
    let input: Vec<String> = fs::read_to_string("./input/22")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();
    println!("{}", get_box_1(&input));
}
