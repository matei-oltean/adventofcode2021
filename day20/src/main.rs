use std::cmp::max;
use std::collections::HashSet;
use std::fs;

fn parse_input(input: &[String]) -> (HashSet<(isize, isize)>, Vec<bool>, isize, isize) {
    let mut lines = input.iter();
    let mut i = 0;
    let mut i_max = 0;
    let mut j_max = 0_isize;
    let mut image = HashSet::new();
    let enhance: Vec<bool> = lines.next().unwrap().chars().map(|v| v == '#').collect();
    lines.next();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                image.insert((i, j as isize));
                i_max = max(i, i_max);
                j_max = max(j as isize, j_max);
            }
        });
        i += 1;
    }
    (image, enhance, i_max, j_max)
}

fn do_iter(
    image: &HashSet<(isize, isize)>,
    enhance: &[bool],
    iter: isize,
    i_max: isize,
    j_max: isize,
) -> HashSet<(isize, isize)> {
    let mut next_image = HashSet::new();
    let ((i_min, i_max), (j_min, j_max)) = ((-1 * iter, i_max + iter), (-1 * iter, j_max + iter));
    (i_min - 1..i_max + 2).for_each(|i| {
        (j_min - 1..j_max + 2).for_each(|j| {
            if enhance[get_bin(
                (i, j),
                image,
                enhance,
                iter,
                ((i_min, i_max), (j_min, j_max)),
            )] {
                next_image.insert((i, j));
            }
        })
    });
    next_image
}

fn get_bin(
    (i, j): (isize, isize),
    image: &HashSet<(isize, isize)>,
    enhance: &[bool],
    iter: isize,
    ((i_min, i_max), (j_min, j_max)): ((isize, isize), (isize, isize)),
) -> usize {
    let s: String = (i - 1..=i + 1)
        .flat_map(|ii| {
            (j - 1..=j + 1).map(move |jj| {
                if image.contains(&(ii, jj)) {
                    '1'
                } else if ((i_min..=i_max).contains(&ii) && (j_min..=j_max).contains(&jj))
                    || !enhance[0]
                    || iter % 2 == 0
                {
                    '0'
                } else {
                    '1'
                }
            })
        })
        .collect();
    usize::from_str_radix(s.as_str(), 2).unwrap()
}

fn main() {
    let input: Vec<String> = fs::read_to_string("./input/20")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect();
    let (mut image, enhance, i_max, j_max) = parse_input(&input);
    (0..50).for_each(|iter| image = do_iter(&image, &enhance, iter, i_max, j_max));
    println!("{}", image.len());
}
