use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn choice_points(&self) -> u64 {
        // rock.paper.scissors
        // +1   +2    +3  points
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    /// choice to make in order to win
    fn win_against(&self) -> Self {
        use Choice::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    /// choice to make in order to lose
    fn lose_against(&self) -> Self {
        use Choice::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    /// choice to make in order to draw
    fn draw(&self) -> Self {
        *self
    }
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // rock.paper.scissors
        //  A    B     C
        //  X    Y     Z (only applies to part 1)
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => panic!("unhandled game choice"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn as_points(&self) -> u64 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    opponent: Choice,
    me: Choice,
}

impl Round {
    fn outcome(&self) -> Outcome {
        use Choice::*;

        let opponent = self.opponent;

        match self.me {
            Rock if opponent == Rock => Outcome::Draw,
            Rock if opponent == Paper => Outcome::Lose,
            Rock if opponent == Scissors => Outcome::Win,

            Paper if opponent == Rock => Outcome::Win,
            Paper if opponent == Paper => Outcome::Draw,
            Paper if opponent == Scissors => Outcome::Lose,

            Scissors if opponent == Rock => Outcome::Lose,
            Scissors if opponent == Paper => Outcome::Win,
            Scissors if opponent == Scissors => Outcome::Draw,

            Rock | Paper | Scissors => unreachable!(),
        }
    }
}

pub fn solve_part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            // The first column is the opponent's choice.
            // The second column is my choice.
            let (opponent_choice, my_choice) = {
                let components = line.splitn(2, ' ').collect::<Vec<_>>();
                (components[0], components[1])
            };
            Round {
                opponent: Choice::from_str(opponent_choice).unwrap(),
                me: Choice::from_str(my_choice).unwrap(),
            }
        })
        .map(|round| round.outcome().as_points() + round.me.choice_points())
        .sum()
}

pub fn solve_part_2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            // The first column is the opponent's choice.
            // The second column is the expected outcome.
            let (opponent_choice, outcome) = {
                let components = line.splitn(2, ' ').collect::<Vec<_>>();
                (Choice::from_str(components[0]).unwrap(), components[1])
            };

            let my_choice = {
                // X -> lose
                // Y -> draw
                // Z -> win
                match outcome {
                    "X" => opponent_choice.lose_against(),
                    "Y" => opponent_choice.draw(),
                    "Z" => opponent_choice.win_against(),
                    _ => unreachable!(),
                }
            };
            Round {
                opponent: opponent_choice,
                me: my_choice,
            }
        })
        .map(|round| round.outcome().as_points() + round.me.choice_points())
        .sum()
}

fn main() {
    let input = read_to_string("input/day02").expect("missing day02 data file");

    let part1 = solve_part_1(&input);
    let part2 = solve_part_2(&input);

    println!("{}\n{}", part1, part2);
}

#[cfg(test)]
mod test {

    #[test]
    fn sample_part_1() {
        let input = r#"A Y
B X
C Z"#;
        let answer = super::solve_part_1(input);
        assert_eq!(answer, 15);
    }

    #[test]
    fn sample_part_2() {
        let input = r#"A Y
B X
C Z"#;
        let answer = super::solve_part_2(input);
        assert_eq!(answer, 12);
    }
}
