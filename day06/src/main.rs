use itertools::Itertools;

fn main() {
    part_a();
    part_b();
}

fn find_unique_substr(chars: &[char], count: u32) -> usize {
    let position = chars
        .windows(count as usize)
        .position(|window| {
            let mut v = 0u32;
            for c in window {
                v |= 1 << (*c as u32 - 97);
            }
            v.count_ones() == count
        })
        .unwrap();
    position + count as usize
}

fn part_a() {
    let chars = include_str!("input.txt").chars().collect_vec();
    let pos = find_unique_substr(&chars, 4);
    println!("a: {pos}");
}

fn part_b() {
    let chars = include_str!("input.txt").chars().collect_vec();
    let pos = find_unique_substr(&chars, 14);
    println!("b: {pos}");
}
