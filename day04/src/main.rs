fn main() {
    part_a();
    part_b();
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let (x, y) = line.split_once(',').unwrap();
    (parse_range(x), parse_range(y))
}

fn parse_range(value: &str) -> (i32, i32) {
    let (a, b) = value.split_once('-').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn part_a() {
    let count = include_str!("input.txt")
        .lines()
        .map(parse_line)
        .filter(|((x1, x2), (y1, y2))| x1 >= y1 && x2 <= y2 || y1 >= x1 && y2 <= x2)
        .count();
    println!("a: {count}")
}

fn part_b() {
    let count = include_str!("input.txt")
        .lines()
        .map(parse_line)
        .filter(|((x1, x2), (y1, y2))| x1 <= y2 && y1 <= x2)
        .count();
    println!("b: {count}")
}
