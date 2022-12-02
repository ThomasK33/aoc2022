use std::path::PathBuf;

use anyhow::Result;

#[derive(Debug, PartialEq)]
pub(crate) enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Choice {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),

            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),

            _ => Err(anyhow::anyhow!("Could not determine choice")),
        }
    }
}

impl TryFrom<(&Choice, &str)> for Choice {
    type Error = anyhow::Error;

    fn try_from((opponent, outcome): (&Choice, &str)) -> Result<Self, Self::Error> {
        match (opponent, outcome) {
            (Choice::Rock, "X") => Ok(Self::Scissors),
            (Choice::Paper, "X") => Ok(Self::Rock),
            (Choice::Scissors, "X") => Ok(Self::Paper),

            (Choice::Rock, "Y") => Ok(Self::Rock),
            (Choice::Paper, "Y") => Ok(Self::Paper),
            (Choice::Scissors, "Y") => Ok(Self::Scissors),

            (Choice::Rock, "Z") => Ok(Self::Paper),
            (Choice::Paper, "Z") => Ok(Self::Scissors),
            (Choice::Scissors, "Z") => Ok(Self::Rock),

            _ => Err(anyhow::anyhow!("Could not determine choice")),
        }
    }
}

impl From<&Choice> for usize {
    fn from(choice: &Choice) -> Self {
        match choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl Choice {
    fn get_score(&self, opponent: &Self) -> usize {
        match (self, opponent) {
            (Choice::Rock, Choice::Rock) => 3,
            (Choice::Rock, Choice::Paper) => 0,
            (Choice::Rock, Choice::Scissors) => 6,
            (Choice::Paper, Choice::Rock) => 6,
            (Choice::Paper, Choice::Paper) => 3,
            (Choice::Paper, Choice::Scissors) => 0,
            (Choice::Scissors, Choice::Rock) => 0,
            (Choice::Scissors, Choice::Paper) => 6,
            (Choice::Scissors, Choice::Scissors) => 3,
        }
    }
}

pub(crate) fn solve(path: PathBuf, part_2: bool) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let total_score: usize = file
        .lines()
        .map(|line| {
            log::trace!("Line: {line}");

            let mut split = line.split_ascii_whitespace();
            let first = split.next().expect("Could not get first character");
            let second = split.next().expect("Could not get second character");

            log::trace!("First {first}, second {second}");

            let first = Choice::try_from(first).unwrap();
            let second = if !part_2 {
                Choice::try_from(second).unwrap()
            } else {
                Choice::try_from((&first, second)).unwrap()
            };
            log::debug!("First {first:?}, Second {second:?}");

            let round_outcome = &second.get_score(&first);
            log::debug!("Round outcome {}", round_outcome);

            let round_score: usize = usize::from(&second) + round_outcome;
            log::debug!("Round score: {round_score}");

            round_score
        })
        .sum();

    log::info!("Total score: {total_score}");

    Ok(())
}
