use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::new();
    let mut current_count = 0;
    for line in include_str!("input.txt").lines() {
        if line.is_empty() {
            set.insert(current_count);
            current_count = 0;
        } else {
            current_count += line.parse::<i32>().unwrap();
        }
    }

    let top_n = |n| set.iter().rev().take(n).sum::<i32>();

    println!("a: {}", top_n(1));
    println!("b: {}", top_n(3));
}
