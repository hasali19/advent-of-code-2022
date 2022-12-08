use itertools::Itertools;

const STEPS: &[(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let width = grid[0].len();
    let height = grid.len();
    let mut count = 0;
    let mut max_score = 0;

    for x in 0..width {
        for y in 0..height {
            let mut visible = false;
            let mut score = 1;
            for (dx, dy) in STEPS {
                let mut cx = x as isize + dx;
                let mut cy = y as isize + dy;
                let mut hidden = false;
                let mut distance = 0;
                while cx >= 0 && cy >= 0 && cx < width as isize && cy < height as isize {
                    distance += 1;
                    if grid[cx as usize][cy as usize] >= grid[x][y] {
                        hidden = true;
                        break;
                    }
                    cx += dx;
                    cy += dy;
                }
                score *= distance;
                if !hidden {
                    visible = true;
                }
            }
            if score > max_score {
                max_score = score;
            }
            if visible {
                count += 1;
            }
        }
    }

    println!("a: {count}");
    println!("b: {max_score}");
}
