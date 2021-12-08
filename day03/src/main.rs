use std::{fs, isize};

fn val_from_string(bin: &[isize], cmp: fn(isize) -> bool) -> isize {
    isize::from_str_radix(
        bin.iter()
            .map(|&x| if cmp(x) { '1' } else { '0' })
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap()
}

fn get_majority(report: &[Vec<isize>]) -> Vec<isize> {
    let mut majorities: Vec<isize> = vec![0; report[0].len()];
    report.iter().for_each(|line| {
        line.iter()
            .enumerate()
            .for_each(|(i, val)| majorities[i] += 2 * val - 1)
    });
    majorities
}

fn gamma_epsilon(report: &[Vec<isize>]) -> isize {
    let majority = get_majority(report);
    val_from_string(&majority, |x| x >= 0) * val_from_string(&majority, isize::is_negative)
}

fn highlander(report: &[Vec<isize>], cmp: fn(isize) -> bool) -> isize {
    let mut cur: Vec<usize> = (0..report.len()).collect();
    let mut i = 0;
    while cur.len() > 1 {
        let maj = cmp(cur.iter().map(|&i| &report[i]).map(|v| 2 * v[i] - 1).sum()) as isize;
        cur = cur
            .into_iter()
            .filter(|&idx| report[idx][i] == maj)
            .collect();
        i += 1;
    }
    val_from_string(&report[cur[0]], |x| x == 1)
}

fn main() {
    let report: Vec<Vec<isize>> = fs::read_to_string("./input/03")
        .unwrap()
        .trim()
        .lines()
        .map(|s| {
            s.to_string()
                .split("")
                .filter_map(|x| x.parse().ok())
                .collect()
        })
        .collect();
    println!("{}", gamma_epsilon(&report));
    println!(
        "{}",
        highlander(&report, |x| x >= 0) * highlander(&report, isize::is_negative)
    );
}
