use std::fs;

fn risk(depths: &[Vec<u32>]) -> u32 {
    depths
        .iter()
        .enumerate()
        .map(|(i, val)| {
            val.iter()
                .enumerate()
                .filter_map(|(j, &depth)| {
                    if (j + 1 == val.len() || depth < val[j + 1])
                        && (j == 0 || depth < val[j - 1])
                        && (i == 0 || depth < depths[i - 1][j])
                        && (i + 1 == depths.len() || depth < depths[i + 1][j])
                    {
                        Some(depth + 1)
                    } else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

fn find_root(node: usize, precedents: &[usize]) -> usize {
    let mut cur = node;
    while precedents[cur] != cur {
        cur = precedents[cur];
    }
    cur
}

fn basin(depths: &[Vec<u32>]) -> u32 {
    let line_len = depths[0].len();
    let mut prec: Vec<_> = (0..depths.len() * line_len).collect();
    let mut sizes: Vec<_> = vec![0; depths.len() * line_len];
    depths.iter().enumerate().for_each(|(i, val)| {
        val.iter().enumerate().for_each(|(j, &depth)| {
            if depth == 9 {
                return;
            }
            let pos = i * line_len + j;
            let (x0, y0): (isize, isize) = (i.try_into().unwrap(), j.try_into().unwrap());
            [(x0, y0 + 1), (x0, y0 - 1), (x0 + 1, y0), (x0 - 1, y0)]
                .iter()
                .for_each(|(x, y)| {
                    if !(0..depths.len() as isize).contains(x)
                        || !(0..line_len as isize).contains(y)
                    {
                        return;
                    }
                    let (xx, yy): (usize, usize) =
                        ((*x).try_into().unwrap(), (*y).try_into().unwrap());
                    if depths[xx][yy] < depth {
                        let (root_1, root_2) =
                            (find_root(pos, &prec), find_root(xx * line_len + yy, &prec));
                        prec[root_1] = root_2;
                    }
                })
        });
    });
    prec.iter().for_each(|&p| sizes[find_root(p, &prec)] += 1);
    sizes.sort_by(|a, b| b.cmp(a));
    sizes[0] * sizes[1] * sizes[2]
}

fn main() {
    let depths: Vec<Vec<_>> = fs::read_to_string("./input/09")
        .unwrap()
        .trim()
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    println!("{}", risk(&depths));
    println!("{}", basin(&depths));
}
