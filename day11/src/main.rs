use std::fs;

fn get_flashes(octopodes: &mut [u32], steps: u32) -> u32 {
    (0..steps).fold(0, |mut s, _| {
        let mut has_flashed = vec![false; octopodes.len()];
        (0..octopodes.len()).for_each(|i| octopodes[i] += 1);
        let mut has_stopped = false;
        while !has_stopped {
            has_stopped = true;
            (0..octopodes.len()).for_each(|i| {
                if octopodes[i] > 9 && !has_flashed[i] {
                    has_stopped = false;
                    has_flashed[i] = true;
                    s += 1;
                    let mut neighbours: Vec<isize> = Vec::new();
                    let ii: isize = i.try_into().unwrap();
                    (-1..=1).for_each(|di| {
                        (-1..=1).for_each(|dj| {
                            let new_idx = ii + 10 * di + dj;
                            if new_idx != ii
                                && new_idx / 10 == ii / 10 + di
                                && (0..100).contains(&new_idx)
                            {
                                neighbours.push(new_idx);
                            }
                        })
                    });
                    neighbours.iter().for_each(|&j| {
                        let jj: usize = j.try_into().unwrap();
                        octopodes[jj] += 1;
                    });
                }
            });
        }
        (0..octopodes.len()).for_each(|i| {
            if octopodes[i] > 9 {
                octopodes[i] = 0
            }
        });
        s
    })
}

fn get_simultaneous_flashes(octopodes: &mut [u32]) -> u32 {
    let mut iter = 1;
    loop {
        let mut has_flashed = vec![false; octopodes.len()];
        let mut n_flashes = 0;
        (0..octopodes.len()).for_each(|i| octopodes[i] += 1);
        let mut has_stopped = false;
        while !has_stopped {
            has_stopped = true;
            (0..octopodes.len()).for_each(|i| {
                if octopodes[i] > 9 && !has_flashed[i] {
                    has_stopped = false;
                    has_flashed[i] = true;
                    n_flashes += 1;
                    let mut neighbours: Vec<isize> = Vec::new();
                    let ii: isize = i.try_into().unwrap();
                    (-1..=1).for_each(|di| {
                        (-1..=1).for_each(|dj| {
                            let new_idx = ii + 10 * di + dj;
                            if new_idx != ii
                                && new_idx / 10 == ii / 10 + di
                                && (0..100).contains(&new_idx)
                            {
                                neighbours.push(new_idx);
                            }
                        })
                    });
                    neighbours.iter().for_each(|&j| {
                        let jj: usize = j.try_into().unwrap();
                        octopodes[jj] += 1;
                    });
                }
            });
        }
        if n_flashes == 100 {
            return iter;
        }
        (0..octopodes.len()).for_each(|i| {
            if octopodes[i] > 9 {
                octopodes[i] = 0
            }
        });
        iter += 1;
    }
}

fn main() {
    let mut octopodes: Vec<_> = fs::read_to_string("./input/11")
        .unwrap()
        .trim()
        .lines()
        .flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    //println!("{}", get_flashes(&mut octopodes, 100));
    println!("{}", get_simultaneous_flashes(&mut octopodes));
}
