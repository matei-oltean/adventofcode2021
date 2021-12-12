use std::collections::HashMap;
use std::fs;

fn num_path(
    node: String,
    graph: &HashMap<String, Vec<String>>,
    small: HashMap<String, usize>,
    twice_allowed: bool,
    second: Option<String>,
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
            let mut next_clone = small.clone();
            let mut next_second = second.clone();
            if next.chars().next().unwrap().is_lowercase() {
                if !next_clone.contains_key(next) {
                    next_clone.insert(next.to_string(), 0);
                }
                let val = next_clone.get_mut(next).unwrap();
                if val == &mut 2 {
                    return 0;
                }
                if val == &mut 1 {
                    if !twice_allowed {
                        return 0;
                    }
                    if let Some(x) = &second {
                        if x != next {
                            return 0;
                        }
                    } else {
                        next_second = Some(next.to_string());
                    }
                }
                *val += 1;
            }
            num_path(
                next.to_string(),
                graph,
                next_clone,
                twice_allowed,
                next_second,
            )
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
            graph
                .get_mut(&start.to_string())
                .unwrap()
                .push(end.to_string());
            graph
                .get_mut(&end.to_string())
                .unwrap()
                .push(start.to_string());
        });
    println!(
        "{}",
        num_path("start".to_string(), &graph, HashMap::new(), true, None)
    );
}
