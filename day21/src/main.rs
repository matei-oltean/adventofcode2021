fn roll_dice(i: &mut usize, score: &mut usize, pos: &mut usize) {
    (0..3).for_each(|_| {
        *pos += *i;
        *i += 1;
        if *i > 100 {
            *i -= 100;
        }
    });
    *pos %= 10;
    if 0 == *pos {
        *pos = 10;
    }
    *score += *pos;
}

fn roll((start_1, start_2): (usize, usize)) -> usize {
    let (mut score_1, mut score_2) = (0, 0);
    let (mut pos_1, mut pos_2) = (start_1, start_2);
    let mut i = 1;
    let mut num_rolls = 0;
    while score_1 < 1000 && score_2 < 1000 {
        roll_dice(&mut i, &mut score_1, &mut pos_1);
        num_rolls += 3;
        if score_1 >= 1000 {
            return score_2 * num_rolls;
        }
        roll_dice(&mut i, &mut score_2, &mut pos_2);
        num_rolls += 3;
        if score_2 >= 1000 {
            return score_1 * num_rolls;
        }
    }
    0
}

fn main() {
    //let start = (4, 8);
    let start = (3, 5);
    println!("{}", roll(start));
}
