use std::collections::{HashMap, HashSet};
use std::fs;

fn count_unique(nums: &Vec<Vec<String>>) -> isize {
    nums.iter()
        .map(|patterns| {
            patterns.iter().fold(0, |s, x| match x.len() {
                2 | 3 | 4 | 7 => s + 1,
                _ => s,
            })
        })
        .sum()
}

fn decode(inputs: &HashSet<String>) -> HashMap<String, char> {
    let mut str_to_int: HashMap<String, char> = HashMap::new();
    let mut int_to_str: HashMap<char, HashSet<char>> = HashMap::new();
    inputs.iter().for_each(|x| match x.len() {
        2 => {
            str_to_int.insert(x.to_string(), '1');
            int_to_str.insert('1', x.chars().collect());
        }
        3 => {
            str_to_int.insert(x.to_string(), '7');
        }
        4 => {
            str_to_int.insert(x.to_string(), '4');
            int_to_str.insert('4', x.chars().collect());
        }
        7 => {
            str_to_int.insert(x.to_string(), '8');
        }
        _ => (),
    });
    inputs.iter().for_each(|x| {
        if x.len() != 6 && x.len() != 5 {
            return;
        }
        let num: HashSet<char> = x.chars().collect();
        if num.is_superset(&int_to_str[&'4']) {
            str_to_int.insert(x.to_string(), '9');
            return;
        }
        let four_diff_one: HashSet<char> = int_to_str[&'4']
            .difference(&int_to_str[&'1'])
            .copied()
            .collect();
        if four_diff_one.intersection(&num).collect::<Vec<_>>().len() == 2 {
            str_to_int.insert(x.to_string(), if x.len() == 6 { '6' } else { '5' });
            return;
        }
        if x.len() == 6 {
            str_to_int.insert(x.to_string(), '0');
        } else {
            if int_to_str[&'1']
                .intersection(&num)
                .collect::<Vec<_>>()
                .len()
                == 2
            {
                str_to_int.insert(x.to_string(), '3');
            } else {
                str_to_int.insert(x.to_string(), '2');
            }
        }
    });
    str_to_int
}

fn find_res(inputs: &Vec<HashSet<String>>, outputs: &Vec<Vec<String>>) -> isize {
    outputs
        .iter()
        .zip(inputs.iter())
        .map(|(out, inp)| {
            let dec = decode(&inp);
            String::from_iter(out.iter().map(|s| dec[s]))
                .parse::<isize>()
                .unwrap()
        })
        .sum()
}

fn sort_and_return_string(string: &str) -> String {
    let mut chars: Vec<_> = string.chars().collect();
    chars.sort();
    String::from_iter(chars)
}

fn main() {
    let (mut patterns, mut output): (Vec<HashSet<String>>, Vec<Vec<String>>) =
        (Vec::new(), Vec::new());
    fs::read_to_string("./input/08")
        .unwrap()
        .trim()
        .lines()
        .for_each(|s| {
            let spl: Vec<_> = s.split(" | ").collect();
            patterns.push(spl[0].split(' ').map(sort_and_return_string).collect());
            output.push(spl[1].split(' ').map(sort_and_return_string).collect());
        });
    println!("{}", count_unique(&output));
    println!("{}", find_res(&patterns, &output));
}
