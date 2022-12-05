use std::cell::RefCell;

use inpt::Inpt;
use itertools::Itertools;

fn main() {
    part_a();
    part_b();
}

#[derive(Inpt)]
#[inpt(regex = r"move (\d+) from (\d+) to (\d+)")]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn part_a() {
    let stacks = include_str!("stacks.txt")
        .lines()
        .map(|line| RefCell::new(line.chars().collect_vec()))
        .collect_vec();

    for instruction in include_str!("instructions.txt")
        .lines()
        .map(|line| inpt::inpt::<Instruction>(line).unwrap())
    {
        let mut from = stacks[instruction.from - 1].borrow_mut();
        let mut to = stacks[instruction.to - 1].borrow_mut();

        for _ in 0..instruction.count {
            to.push(from.pop().unwrap());
        }
    }

    print!("a: ");

    for stack in &stacks {
        print!("{}", stack.borrow().last().unwrap());
    }

    println!();
}

fn part_b() {
    let stacks = include_str!("stacks.txt")
        .lines()
        .map(|line| RefCell::new(line.chars().collect_vec()))
        .collect_vec();

    for instruction in include_str!("instructions.txt")
        .lines()
        .map(|line| inpt::inpt::<Instruction>(line).unwrap())
    {
        assert!(instruction.from != instruction.to);

        let mut from = stacks[instruction.from - 1].borrow_mut();
        let mut to = stacks[instruction.to - 1].borrow_mut();

        let truncated_len = from.len() - instruction.count;

        to.extend_from_slice(&from[from.len() - instruction.count..]);
        from.truncate(truncated_len);
    }

    print!("b: ");

    for stack in &stacks {
        print!("{}", stack.borrow().last().unwrap());
    }

    println!();
}
