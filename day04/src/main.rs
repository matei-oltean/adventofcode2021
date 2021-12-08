use std::collections::{HashMap, HashSet};
use std::fs;

fn bingo(input: &[String]) -> (usize, usize) {
    let mut first: usize = 0;
    let mut lines = input.iter();
    let mut boards: Vec<Vec<usize>> = Vec::new();
    let numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    lines.next();
    let mut current_board: Vec<usize> = Vec::new();
    let mut positions: Vec<HashMap<usize, usize>> = Vec::new();
    for line in lines {
        if line.is_empty() {
            positions.push(
                current_board
                    .iter()
                    .enumerate()
                    .map(|(i, val)| (*val, i))
                    .collect(),
            );
            boards.push(current_board);
            current_board = Vec::new();
            continue;
        }
        let mut current_line: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        current_board.append(&mut current_line);
    }
    let mut drawn: HashSet<usize> = HashSet::new();
    let mut to_ignore: HashSet<usize> = HashSet::new();
    for num in numbers {
        drawn.insert(num);
        for (i, board) in boards.iter().enumerate() {
            if to_ignore.contains(&i)
                || !positions[i].contains_key(&num)
                || !check_board(*positions[i].get(&num).unwrap(), board, &drawn)
            {
                continue;
            }
            if to_ignore.is_empty() || to_ignore.len() == boards.len() - 1 {
                let res = num * board.iter().filter(|v| !drawn.contains(v)).sum::<usize>();
                if first > 0 {
                    return (first, res);
                }
                first = res;
            }
            to_ignore.insert(i);
        }
    }
    (0, 0)
}

fn check_board(position: usize, board: &[usize], drawn: &HashSet<usize>) -> bool {
    (0..5).all(|s| drawn.contains(&board[position % 5 + 5 * s]))
        || (0..5).all(|s| drawn.contains(&board[(position / 5) * 5 + s]))
}

fn main() {
    let input: Vec<String> = fs::read_to_string("./input/04")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();
    let (first, last) = bingo(&input);
    println!("{} {}", first, last);
}
