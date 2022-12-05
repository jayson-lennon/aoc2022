type StackNumber = usize;

struct StackDrawingPosition {
    byte_position: usize,
}

struct Stack {
    crates: Vec<char>,
}

#[derive(Clone, Copy)]
struct Move {
    count: usize,
    from_stack: StackNumber,
    to_stack: StackNumber,
}

fn build_stacks(input: &str) -> Vec<Stack> {
    // The input uses a blank line to separate the drawing of the crates
    // and the movelist. For the crates, we take everything until the blank line.
    let raw_stack_data = input
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    // Here we are reading the bottom line of the below diagram. The byte position
    // of the numer is recorded and will be used in the next step to read the letters
    // above each number.
    //     [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3
    let drawing_positions: Vec<StackDrawingPosition> = raw_stack_data
        .iter()
        // start from the "bottom" of the stacks of crates, which shows the stack numbers
        .rev()
        .take(1)
        .flat_map(|line| {
            line.chars()
                // go through each character
                .enumerate()
                .filter_map(|(index, ch)| {
                    // only take digits
                    if ch.is_ascii_digit() {
                        Some(StackDrawingPosition {
                            byte_position: index,
                        })
                    } else {
                        // ignore all whitespace
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Using the byte position gained from above, we can read each stack one at a time and
    // build up a Vector for each stack.
    drawing_positions
        .iter()
        .map(|stack_info| Stack {
            crates: raw_stack_data
                .iter()
                .rev()
                .skip(1)
                .map(|line| {
                    let line = line.as_bytes();
                    line[stack_info.byte_position] as char
                })
                // not all stacks will be full of crates, so remove empty slots
                .filter(|ch| !ch.is_whitespace())
                .collect::<Vec<_>>(),
        })
        .collect()
}

fn build_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let line = line
                .replace("move ", "")
                .replace("from ", "")
                .replace("to ", "");
            let mut components = line.split_whitespace().map(|n| n.parse::<usize>().unwrap());
            Move {
                count: components.next().unwrap(),
                from_stack: components.next().unwrap(),
                to_stack: components.next().unwrap(),
            }
        })
        .collect()
}

fn solve_part_1(input: &str) -> String {
    let (mut stacks, moves) = { (build_stacks(input), build_moves(input)) };

    for m in moves {
        let crates = {
            // Stack IDs start at 1, so we subtract 1 to get the index.
            let stack_id = m.from_stack - 1;
            let from_stack = &mut stacks[stack_id].crates;
            // drain the crates in reverse order (top of the stack to bottom)
            from_stack
                .drain(from_stack.len() - m.count..from_stack.len())
                .rev()
                .collect::<Vec<_>>()
        };

        let stack_id = m.to_stack - 1;
        let target_stack = &mut stacks[stack_id].crates;
        for c in crates {
            target_stack.push(c);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.crates.last().unwrap())
        .collect::<String>()
}

fn solve_part_2(input: &str) -> String {
    let (mut stacks, moves) = { (build_stacks(input), build_moves(input)) };

    for m in moves {
        let crates = {
            // Stack IDs start at 1, so we subtract 1 to get the index.
            let stack_id = m.from_stack - 1;
            let from_stack = &mut stacks[stack_id].crates;
            // since the 9001 version lifts all crates at once, there is no need to reverse this
            // iterator as in part 1. Drain takes the entire chunk in order.
            from_stack
                .drain(from_stack.len() - m.count..from_stack.len())
                .collect::<Vec<_>>()
        };

        let stack_id = m.to_stack - 1;
        let target_stack = &mut stacks[stack_id].crates;
        for c in crates {
            target_stack.push(c);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.crates.last().unwrap())
        .collect::<String>()
}

fn main() {
    let input = include_str!("data/day05");

    let part1 = solve_part_1(input);
    println!("{part1}");

    let part2 = solve_part_2(input);
    println!("{part2}");
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        let sample_input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        let answer = super::solve_part_1(sample_input);
        assert_eq!(answer, "CMZ");
    }

    #[test]
    fn part_2() {
        let sample_input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        let answer = super::solve_part_2(sample_input);
        assert_eq!(answer, "MCD");
    }
}
