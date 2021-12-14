use std::fs;

fn count(pairs: &[Vec<usize>], first: u8) -> usize {
    let mut counts = vec![0; 26];
    counts[first as usize] = 1;
    pairs.iter().for_each(|v| {
        v.iter().enumerate().for_each(|(i, val)| {
            counts[i] += val;
        })
    });
    counts.iter().max().unwrap() - counts.iter().filter(|&&v| v > 0).min().unwrap()
}

fn to_usize(c: char) -> u8 {
    c as u8 - 65
}

fn do_step(pairs: &[Vec<usize>], rules: &[Vec<u8>]) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = (0..26).map(|_| (0..26).map(|_| 0).collect()).collect();
    (0..26).for_each(|a| {
        (0..26).for_each(|b| {
            let c = rules[a][b] as usize;
            if c < 26 {
                result[a][c] += pairs[a][b];
                result[c][b] += pairs[a][b];
            } else {
                result[a][b] += pairs[a][b];
            }
        })
    });
    result
}

fn main() {
    let mut rules: Vec<Vec<u8>> = (0..26).map(|_| (0..26).map(|_| 26).collect()).collect();
    let read = fs::read_to_string("./input/14").unwrap();
    let mut iter = read.trim().lines();
    let starting: Vec<u8> = iter.next().unwrap().chars().map(to_usize).collect();
    let mut pairs: Vec<Vec<usize>> = (0..26).map(|_| (0..26).map(|_| 0).collect()).collect();
    (1..starting.len()).for_each(|i| pairs[starting[i - 1] as usize][starting[i] as usize] += 1);
    for line in iter {
        if line.is_empty() {
            continue;
        }
        let s: Vec<_> = line.split(" -> ").collect();
        let first_second: Vec<u8> = s[0].chars().map(to_usize).collect();
        let res = s[1].chars().next().unwrap();
        rules[first_second[0] as usize][first_second[1] as usize] = to_usize(res);
    }
    (0..40).for_each(|_| pairs = do_step(&pairs, &rules));
    println!("{}", count(&pairs, starting[0]));
}
