use std::collections::HashSet;

fn expand_assignment(sections: &str) -> HashSet<usize> {
    let section_range = {
        let (start, end) = sections.split_once('-').unwrap();
        let start = start.parse::<usize>().unwrap();
        let end = end.parse::<usize>().unwrap();
        start..=end
    };

    HashSet::from_iter(section_range)
}

fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|pair| {
            let (a, b) = {
                let (a, b) = pair.split_once(',').unwrap();
                (expand_assignment(a), expand_assignment(b))
            };
            // If a set is a subset of another, then it completely overlaps.
            // Check this for both sets.
            a.is_subset(&b) || b.is_subset(&a)
        })
        // Keep all the `true` sets
        .filter(|is_subset| *is_subset)
        // Count them to get the total overlapping assignments
        .count()
}

fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|pair| {
            let (a, b) = {
                let (a, b) = pair.split_once(',').unwrap();
                (expand_assignment(a), expand_assignment(b))
            };
            // If we have anything at all intersecting, then some
            // part of the assignment overlaps.
            a.intersection(&b).count() > 0
        })
        // Keep all the `true` sets
        .filter(|overlap| *overlap)
        // Count them to get the total overlapping assignments
        .count()
}

fn main() {
    let input = include_str!("data/day04");

    let part1 = solve_part_1(input);
    println!("{part1}");

    let part2 = solve_part_2(input);
    println!("{part2}");
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        let sample_input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        let answer = super::solve_part_1(sample_input);
        assert_eq!(answer, 2);
    }

    #[test]
    fn part_2() {
        let sample_input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        let answer = super::solve_part_2(sample_input);
        assert_eq!(answer, 4);
    }
}
