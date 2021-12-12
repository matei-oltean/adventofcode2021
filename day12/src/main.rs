use std::collections::{HashMap, HashSet};
use std::fs;

fn num_path(
    node: String,
    graph: &HashMap<String, Vec<String>>,
    small: HashSet<String>,
    twice_allowed: bool,
) -> usize {
    graph
        .get(&node)
        .unwrap()
        .iter()
        .map(|next| {
            if next == "end" {
                return 1;
            }
            if next == "start" {
                return 0;
            }
            let mut next_small = small.clone();
            let mut next_allowed = twice_allowed;
            if next.chars().next().unwrap().is_lowercase() {
                if next_small.contains(next) {
                    if !next_allowed {
                        return 0;
                    }
                    next_allowed = false;
                }
                next_small.insert(next.to_string());
            }
            num_path(next.to_string(), graph, next_small, next_allowed)
        })
        .sum()
}

fn main() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    fs::read_to_string("./input/12")
        .unwrap()
        .trim()
        .lines()
        .for_each(|x| {
            let s: Vec<_> = x.split('-').collect();
            let (start, end) = (s[0], s[1]);
            graph.entry(start.to_string()).or_insert_with(Vec::new);
            graph.entry(end.to_string()).or_insert_with(Vec::new);
            graph.get_mut(start).unwrap().push(end.to_string());
            if start != "start" && end != "end" {
                graph.get_mut(end).unwrap().push(start.to_string());
            }
        });
    println!(
        "{}",
        num_path(String::from("start"), &graph, HashSet::new(), false)
    );
    println!(
        "{}",
        num_path(String::from("start"), &graph, HashSet::new(), true)
    );
}
