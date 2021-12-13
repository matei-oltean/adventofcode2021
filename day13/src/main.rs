use std::collections::HashSet;
use std::fs;

fn fold(x: usize, y: usize, coord: char, delta: usize) -> (usize, usize) {
    let sym = |z: usize| 2 * delta - z;
    let new_x = if coord == 'x' && x > delta { sym(x) } else { x };
    let new_y = if coord == 'y' && y > delta { sym(y) } else { y };
    (new_x, new_y)
}

fn do_fold(
    (coord, delta): (char, usize),
    points: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    points
        .iter()
        .filter_map(|&(x, y)| {
            if (coord == 'x' && x == delta) || (coord == 'y' && y == delta) {
                return None;
            }
            Some(fold(x, y, coord, delta))
        })
        .collect()
}

fn print(points: &HashSet<(usize, usize)>) {
    let &(m_x, _) = points.iter().max_by_key(|(x, _)| x).unwrap();
    let &(_, m_y) = points.iter().max_by_key(|(_, y)| y).unwrap();
    println!();
    (0..=m_y).for_each(|y| {
        (0..=m_x).for_each(|x| {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
    });
    println!();
}

fn main() {
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut folds: Vec<(char, usize)> = Vec::new();
    fs::read_to_string("./input/13")
        .unwrap()
        .trim()
        .lines()
        .for_each(|x| {
            if x.is_empty() {
                return;
            }
            if x.starts_with('f') {
                let s: Vec<_> = x.split('=').collect();
                return folds.push((s[0].chars().last().unwrap(), s[1].parse().unwrap()));
            }
            let s: Vec<_> = x.split(',').collect();
            points.insert((s[0].parse().unwrap(), s[1].parse().unwrap()));
        });
    folds.iter().for_each(|&f| {
        points = do_fold(f, &points);
        println!("{}", points.len());
    });
    print(&points);
}
