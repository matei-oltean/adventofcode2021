use std::cmp::Ordering;

fn do_shot(vx: isize, vy: isize, (x0, x1): (isize, isize), (y0, y1): (isize, isize)) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut vx = vx;
    let mut vy = vy;
    while y >= y0 {
        x += vx;
        y += vy;
        if (x0..=x1).contains(&x) && (y0..=y1).contains(&y) {
            return true;
        }
        vy -= 1;
        vx += match vx.cmp(&0) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };
    }
    false
}

fn possibles((x0, x1): (isize, isize), (y0, y1): (isize, isize)) -> usize {
    (y0..=-y0)
        .map(|vy| {
            (1..=x1)
                .map(|vx| do_shot(vx, vy, (x0, x1), (y0, y1)) as usize)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    //let (x_test, y_test) = ((20, 30), (-10, -5));
    let (x, y) = ((88, 125), (-157, -103));
    println!("{}", (y.0) * (y.0 + 1) / 2);
    println!("{}", possibles(x, y));
}
