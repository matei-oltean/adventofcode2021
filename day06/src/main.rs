use std::fs;

fn num_of_fish(ages: &mut Vec<isize>, iterations: isize) -> isize {
    for _ in 0..iterations {
        ages.rotate_left(1);
        ages[6] += ages[8];
    }
    ages.iter().sum()
}

fn main() {
    let mut ages: Vec<isize> = vec![0; 9];
    fs::read_to_string("./input/06")
        .unwrap()
        .trim()
        .split(',')
        .for_each(|s| {
            ages[s.parse::<usize>().unwrap()] += 1;
        });
    //println!("{}", num_of_fish(&mut ages, 80));
    println!("{}", num_of_fish(&mut ages, 256));
}
