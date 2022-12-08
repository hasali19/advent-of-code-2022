use std::collections::HashMap;
use std::path::PathBuf;

use inpt::Inpt;

#[derive(Debug, Inpt)]
enum Line<'a> {
    #[inpt(regex = r"\$ (.+)")]
    Cmd(Cmd<'a>),
    #[inpt]
    Entry(Entry<'a>),
}

#[derive(Debug, Inpt)]
enum Cmd<'a> {
    #[inpt(regex = r"cd (.+)")]
    Cd(&'a str),
    #[inpt(regex = r"ls")]
    Ls,
}

#[derive(Debug, Inpt)]
enum Entry<'a> {
    #[inpt(regex = r"dir (.+)")]
    Dir(&'a str),
    #[inpt(regex = r"(\d+) (.+)")]
    File(usize, &'a str),
}

fn main() {
    let mut current_path = PathBuf::new();
    let mut dir_sizes = HashMap::new();

    for line in include_str!("input.txt")
        .lines()
        .map(|line| inpt::inpt::<Line>(line).unwrap())
    {
        match line {
            Line::Cmd(Cmd::Cd("..")) => {
                current_path.pop();
            }
            Line::Cmd(Cmd::Cd(path)) => {
                current_path.push(path);
                dir_sizes.insert(current_path.to_str().unwrap().to_owned(), 0);
            }
            Line::Cmd(Cmd::Ls) => {}
            Line::Entry(e) => {
                if let Entry::File(size, _) = e {
                    for path in current_path.ancestors() {
                        *dir_sizes.get_mut(path.to_str().unwrap()).unwrap() += size;
                    }
                }
            }
        }
    }

    let total: usize = dir_sizes.values().filter(|&&it| it < 100_000).sum();

    println!("a: {total}");

    let root_size = dir_sizes.get("/").unwrap();
    let required_size = 30_000_000 - (70_000_000 - root_size);

    let min_size = dir_sizes
        .values()
        .filter(|&&it| it >= required_size)
        .min()
        .unwrap();

    println!("b: {min_size}");
}
