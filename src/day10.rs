use std::path::PathBuf;

use anyhow::Result;
use chumsky::prelude::*;

pub(crate) fn solve(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let mut parsed_instructions = file_parser()
        .parse(file)
        .map_err(|err| anyhow::anyhow!("Could not parse file, an error occurred: {err:?}"))?;

    // Prepend a noop instruction, as the counter starts with a 1
    parsed_instructions.insert(0, Instruction::NoOp);

    let cycle_value: Vec<(usize, i32)> = parsed_instructions
        .into_iter()
        .flat_map(|element| match element {
            Instruction::AddX(x) => vec![0, x],
            Instruction::NoOp => vec![0],
        })
        .scan(1, |state, x| {
            *state += x;

            Some(*state)
        })
        .enumerate()
        .collect();
    log::debug!("cycle_value: {cycle_value:#?}");

    let task_a_solution: i32 = cycle_value
        .iter()
        .map(|(a, x)| (i32::try_from(*a).unwrap_or(0) + 1, x))
        .filter(|(a, _)| (a - 20) % 40 == 0)
        .map(|(a, x)| a * x)
        .sum();
    log::info!("Task A solution: {task_a_solution}");

    let crt: Vec<Vec<char>> = cycle_value
        .iter()
        .map(|(a, x)| (i32::try_from(*a).unwrap_or(0) % 40, *x))
        .map(|(a, x)| a > (x - 2) && a < (x + 2))
        .map(|res| if res { '#' } else { '.' })
        .collect::<Vec<char>>()
        .chunks_exact(40)
        .map(|chunk| chunk.to_vec())
        .collect();

    log::info!("Task B - CRT Screen:");
    for row in crt {
        // log::info!("{row:?}");

        for row_char in row {
            print!("{row_char}");
        }
        println!();
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Instruction {
    AddX(i32),
    NoOp,
}

fn file_parser() -> impl Parser<char, Vec<Instruction>, Error = Simple<char>> {
    (just("addx")
        .padded()
        .ignore_then(take_until(text::newline()))
        .map(|(preceding, _)| preceding.into_iter().collect::<String>().parse().unwrap())
        .map(Instruction::AddX))
    .or(just("noop").padded().ignored().map(|_| Instruction::NoOp))
    .repeated()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY_10_DEV_INPUT: &str = include_str!("../tasks/day10_dev.txt");

    #[test]
    fn test_file_parser() {
        let sample_content = r#"
            addx 15
            addx -11
            noop
            addx -4
        "#;

        let result = file_parser().parse(sample_content);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                Instruction::AddX(15),
                Instruction::AddX(-11),
                Instruction::NoOp,
                Instruction::AddX(-4),
            ]
        );
    }

    #[test]
    fn test_file_parser_on_dev_file() {
        let result = file_parser().parse(DAY_10_DEV_INPUT);

        assert!(result.is_ok());
    }
}
