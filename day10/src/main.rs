use std::fs;

fn score(c: char) -> usize {
    match c {
        ')' => 3,
        '}' => 1197,
        ']' => 57,
        _ => 25137,
    }
}

fn score_autocomplete(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        _ => 4,
    }
}

fn matching(c: char) -> char {
    match c {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        _ => '>',
    }
}

fn is_opener(c: char) -> bool {
    matches!(c, '(' | '{' | '[' | '<')
}

fn get_score(chunks: &[Vec<char>]) -> (usize, usize) {
    let mut first_score = 0;
    let mut autocomplete_scores: Vec<_> = chunks
        .iter()
        .filter_map(|chunk| {
            let mut closing: Vec<char> = Vec::new();
            for &c in chunk.iter() {
                if is_opener(c) {
                    closing.push(c);
                } else if matching(closing.pop().unwrap()) != c {
                    first_score += score(c);
                    return None;
                }
            }
            Some(
                closing
                    .iter()
                    .rev()
                    .fold(0, |score, &c| 5 * score + score_autocomplete(c)),
            )
        })
        .collect();
    autocomplete_scores.sort_unstable();
    (
        first_score,
        autocomplete_scores[autocomplete_scores.len() / 2],
    )
}

fn main() {
    let chunks: Vec<Vec<_>> = fs::read_to_string("./input/10")
        .unwrap()
        .trim()
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    println!("{:?}", get_score(&chunks));
}
