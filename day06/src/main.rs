use std::fs;

fn main() {
    let mut ages: Vec<isize> = vec![0; 9];
    fs::read_to_string("./input/06")
        .unwrap()
        .trim()
        .split(',')
        .for_each(|s| {
            ages[s.parse::<usize>().unwrap()] += 1;
        });
    for _ in 0..256 {
        ages.rotate_left(1);
        ages[6] += ages[8];
    }
    println!("{}", ages.iter().sum::<isize>());
}
