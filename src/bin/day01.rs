use std::fs::read_to_string;

pub fn solve() -> (u64, u64) {
    let input = read_to_string("input/day01").expect("missing day01 data file");

    let mut totals = input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|calories| str::parse::<u64>(calories).expect("non-numeric data"))
                .sum()
        })
        .collect::<Vec<_>>();

    totals.sort();

    let sol1: u64 = totals[totals.len() - 1];
    let sol2: u64 = totals.iter().rev().take(3).sum();

    (sol1, sol2)
}

fn main() {
    let (part1, part2) = solve();
    println!("{}\n{}", part1, part2);
}
