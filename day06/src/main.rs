use std::fs;

fn num_of_fish(starting: &mut Vec<isize>, iterations: isize) -> isize {
    for _ in 0..iterations {
        let mut cur = vec![0; 9];
        starting.iter().enumerate().for_each(|(age, count)| {
            if age == 0 {
                cur[6] += count;
                cur[8] += count
            } else {
                cur[age - 1] += count;
            }
        });
        *starting = cur;
    }
    starting.iter().sum()
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
