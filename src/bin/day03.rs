use std::collections::HashSet;

struct Rucksack<'a> {
    items: &'a str,
}

impl<'a> Rucksack<'a> {
    fn new(items: &'a str) -> Self {
        Self { items }
    }

    fn compartments(&self) -> (&str, &str) {
        self.items.split_at(self.items.len() / 2)
    }

    fn items_in_both_compartments(&self) -> Vec<char> {
        let (a, b) = {
            let compartments = self.compartments();
            (
                compartments.0.chars().collect::<HashSet<char>>(),
                compartments.1.chars().collect::<HashSet<char>>(),
            )
        };
        let k = a.intersection(&b).copied().collect();
        k
    }
}

trait Priority {
    fn priority(&self) -> u64;
}

impl Priority for char {
    fn priority(&self) -> u64 {
        const ASCII_LOWERCASE_BEGIN: u64 = 97; // 'a'
        const ASCII_UPPERCASE_BEGIN: u64 = 65; // 'A'

        let ch = *self as u64;

        if ch >= ASCII_LOWERCASE_BEGIN {
            // Lowercase item types a through z have priorities 1 through 26.
            ch - ASCII_LOWERCASE_BEGIN + 1
        } else {
            // Uppercase item types A through Z have priorities 27 through 52.
            ch - ASCII_UPPERCASE_BEGIN + 1 + 26
        }
    }
}

fn solve_part_1(input: &str) -> u64 {
    input
        .lines()
        .map(Rucksack::new)
        .map(|sack| {
            sack.items_in_both_compartments()
                .iter()
                .fold(0, |sum, item| sum + item.priority())
        })
        .sum()
}

fn solve_part_2(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();

    lines
        .as_slice()
        // elves are in groups of 3
        .chunks(3)
        .map(|elf_group| {
            elf_group
                .iter()
                // put each character into a set
                .map(|line| line.chars().collect::<HashSet<char>>())
                // intersect all the sets
                .reduce(|badge, line| badge.intersection(&line).copied().collect())
                .unwrap()
                // take the badge from the hashset which was generated above
                .drain()
                .next()
                .unwrap()
                // calculate priority of the badge character
                .priority()
        })
        .sum()
}

fn main() {
    let input = include_str!("data/day03");

    let part1 = solve_part_1(input);
    println!("{part1}");

    let part2 = solve_part_2(input);
    println!("{part2}");
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        let sample_input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let answer = super::solve_part_1(sample_input);
        assert_eq!(answer, 157);
    }

    #[test]
    fn part_2() {
        let sample_input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let answer = super::solve_part_2(sample_input);
        assert_eq!(answer, 70);
    }
}
