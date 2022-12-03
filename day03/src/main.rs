use itertools::Itertools;

fn main() {
    part_a();
    part_b();
}

fn priority(c: char) -> u32 {
    let i = c as u32;
    if (97..=122).contains(&i) {
        i - 96
    } else {
        i - 38
    }
}

fn part_a() {
    let total: u32 = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (l, r) = line.split_at(line.len() / 2);
            let shared_item = l.chars().find(|c| r.contains(*c)).unwrap();
            priority(shared_item)
        })
        .sum();

    println!("a: {total}");
}

fn part_b() {
    let total: u32 = include_str!("input.txt")
        .lines()
        .tuples()
        .map(|(x, y, z)| {
            let shared_item = x
                .chars()
                .find(|c| y.contains(*c) && z.contains(*c))
                .unwrap();
            priority(shared_item)
        })
        .sum();

    println!("b: {total}");
}
