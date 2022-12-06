use std::collections::HashSet;

fn solve(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        // Iterate through windows of size 4. Since we need to count the number
        // of the characters processed until we find a valid sequence (windows start at 0),
        // we need to add `window_size` to this value at the end of this chain.
        .windows(window_size)
        // Use enumerate because we want to count the characters.
        .enumerate()
        // Ignore the counter. We only need it at the end.
        .find(|(_, window)| {
            let set: HashSet<&u8> = HashSet::from_iter(*window);
            set.len() == window_size
        })
        .unwrap()
        // Get the number of iterations before a unique sequence was found.
        .0
        // character offset. See above note for details.
        + window_size
}

fn solve_part_1(input: &str) -> usize {
    solve(input, 4)
}

fn solve_part_2(input: &str) -> usize {
    solve(input, 14)
}

fn main() {
    let input = include_str!("data/day06");

    let part1 = solve_part_1(input);
    println!("{part1}");

    let part2 = solve_part_2(input);
    println!("{part2}");
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        let sample_input = &[
            (r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#, 5),
            (r#"nppdvjthqldpwncqszvftbrmjlhg"#, 6),
            (r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#, 10),
            (r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#, 11),
        ];

        for (input, expected) in sample_input {
            let answer = super::solve_part_1(input);
            assert_eq!(answer, *expected);
        }
    }

    #[test]
    fn part_2() {
        let sample_input = &[
            (r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#, 19),
            (r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#, 23),
            (r#"nppdvjthqldpwncqszvftbrmjlhg"#, 23),
            (r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#, 29),
            (r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#, 26),
        ];

        for (input, expected) in sample_input {
            let answer = super::solve_part_2(input);
            assert_eq!(answer, *expected);
        }
    }
}
