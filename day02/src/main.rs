use inpt::{inpt, Inpt};

#[repr(i32)]
#[derive(Inpt)]
enum Shape {
    #[inpt(regex = "A|X")]
    Rock = 1,
    #[inpt(regex = "B|Y")]
    Paper = 2,
    #[inpt(regex = "C|Z")]
    Scissors = 3,
}

fn shape_score(shape: &str) -> i32 {
    inpt::<Shape>(shape).unwrap() as i32
}

#[repr(i32)]
#[derive(Inpt)]
enum Outcome {
    #[inpt(regex = "X")]
    Lose = 0,
    #[inpt(regex = "Y")]
    Draw = 3,
    #[inpt(regex = "Z")]
    Win = 6,
}

fn part_a() -> i32 {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(' ').unwrap();

            let their_score = shape_score(l);
            let my_score = shape_score(r);

            let outcome = if my_score == their_score {
                Outcome::Draw
            } else if (my_score - 1) == (their_score % 3) {
                Outcome::Win
            } else {
                Outcome::Lose
            };

            my_score + outcome as i32
        })
        .sum()
}

fn part_b() -> i32 {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(' ').unwrap();

            let their_score = shape_score(l);
            let outcome = inpt::<Outcome>(r).unwrap();

            let my_score = match outcome {
                Outcome::Lose => (their_score + 1) % 3 + 1,
                Outcome::Draw => their_score,
                Outcome::Win => their_score % 3 + 1,
            };

            my_score + outcome as i32
        })
        .sum()
}

fn main() {
    println!("a: {}", part_a());
    println!("b: {}", part_b());
}
