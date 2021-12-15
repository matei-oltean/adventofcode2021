use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs;

fn risk(risk_map: &[Vec<u32>]) -> u32 {
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut paths: HashMap<(isize, isize), u32> = HashMap::new();
    let mut weights: Vec<Vec<u32>> = vec![vec![u32::MAX; risk_map.len()]; risk_map.len()];
    paths.insert((0, 0), 0);
    weights[0][0] = 0;
    while seen.len() < (risk_map.len()) * (risk_map.len()) {
        let ((i, j), _) = paths.iter().min_by_key(|(_, val)| *val).unwrap();
        let (i, j) = (*i, *j);
        seen.insert((i, j));
        paths.remove(&(i, j));
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (ii, jj) = (i + di, j + dj);
            if 0 <= ii
                && ii < risk_map.len().try_into().unwrap()
                && 0 <= jj
                && jj < risk_map.len().try_into().unwrap()
                && !seen.contains(&(ii, jj))
            {
                let (x, y): (usize, usize) = (ii.try_into().unwrap(), jj.try_into().unwrap());
                let new_val = min(
                    weights[i as usize][j as usize] + risk_map[x][y],
                    weights[x][y],
                );
                weights[x][y] = new_val;
                paths.insert((ii, jj), new_val);
            }
        }
    }
    weights[risk_map.len() - 1][risk_map.len() - 1]
}

fn enlarge(risk_map: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut new_map = vec![vec![0; 5 * risk_map.len()]; 5 * risk_map.len()];
    (0..risk_map.len()).for_each(|i| {
        (0..risk_map.len()).for_each(|j| {
            new_map[i][j] = risk_map[i][j];
        });
    });
    (0..risk_map.len()).for_each(|i| {
        (1..5).for_each(|k| {
            (0..risk_map.len()).for_each(|j| {
                let mut new_val = (1 + new_map[i][j + (k - 1) * risk_map.len()]) % 10;
                if new_val == 0 {
                    new_val = 1;
                }
                new_map[i][j + k * risk_map.len()] = new_val;
            });
        });
    });
    (risk_map.len()..5 * risk_map.len()).for_each(|i| {
        (1..5).for_each(|_| {
            (0..5 * risk_map.len()).for_each(|j| {
                let mut new_val = (1 + new_map[i - risk_map.len()][j]) % 10;
                if new_val == 0 {
                    new_val = 1;
                }
                new_map[i][j] = new_val;
            });
        });
    });
    new_map
}

fn main() {
    let mut risk_map: Vec<Vec<u32>> = fs::read_to_string("./input/15")
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    risk_map = enlarge(&risk_map);
    println!("{}", risk(&risk_map));
}
