use std::collections::HashSet;

use inpt::Inpt;

fn main() {
    println!("a: {}", count_tail_positions(2));
    println!("b: {}", count_tail_positions(10));
}

#[derive(Inpt)]
enum Dir {
    #[inpt(regex = "U")]
    Up,
    #[inpt(regex = "D")]
    Down,
    #[inpt(regex = "L")]
    Left,
    #[inpt(regex = "R")]
    Right,
}

#[derive(Inpt)]
#[inpt(regex = r"(\w) (\d+)")]
struct Step {
    dir: Dir,
    count: usize,
}

fn move_tail(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
        (
            tail.0 + (head.0 - tail.0).signum(),
            tail.1 + (head.1 - tail.1).signum(),
        )
    } else {
        tail
    }
}

fn count_tail_positions(knots: usize) -> usize {
    let mut knots = vec![(0, 0); knots];
    let mut positions = HashSet::new();

    for step in include_str!("input.txt")
        .lines()
        .map(|line| inpt::inpt::<Step>(line).unwrap())
    {
        let (dx, dy) = match step.dir {
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };

        for _ in 0..step.count {
            knots[0] = (knots[0].0 + dx, knots[0].1 + dy);

            for i in 1..knots.len() {
                knots[i] = move_tail(knots[i], knots[i - 1]);
            }

            positions.insert(*knots.last().unwrap());
        }
    }

    positions.len()
}
