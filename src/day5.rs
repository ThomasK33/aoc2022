use std::path::PathBuf;

use anyhow::Result;
use chumsky::prelude::*;

pub(crate) fn solve(path: PathBuf, part_a: bool) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let (stack_elements, moves) = file_parser().parse(file).map_err(|err| {
        anyhow::Error::msg(format!("An error occurred while parsing the file: {err:?}"))
    })?;
    log::trace!("stack_elements: {stack_elements:?}");
    log::trace!("moves: {moves:?}");

    let stack_amount =
        stack_elements.iter().fold(
            0,
            |acc, item| {
                if acc > item.len() {
                    acc
                } else {
                    item.len()
                }
            },
        );

    let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_amount];

    for elements in stack_elements {
        for (index, element) in elements.iter().enumerate() {
            if let Some(element) = element {
                if let Some(stack) = stacks.get_mut(index) {
                    stack.insert(0, *element);
                }
            }
        }
    }

    if part_a {
        for (count, source, target) in moves {
            for _ in 0..count {
                if let Some(element) = stacks[(source - 1) as usize].pop() {
                    stacks[(target - 1) as usize].push(element);
                }
            }
        }
    } else {
        for (count, source, target) in moves {
            let mut temp_elements = vec![];

            for _ in 0..count {
                if let Some(element) = stacks[(source - 1) as usize].pop() {
                    temp_elements.insert(0, element);
                }
            }

            stacks[(target - 1) as usize].extend(temp_elements);
        }
    }

    log::debug!("Stacks: {stacks:?}");

    let mut answer = String::new();
    for stack in stacks {
        if let Some(element) = stack.last() {
            answer.push(*element);
        }
    }

    log::info!("Answer: {answer}");

    Ok(())
}

fn file_parser(
) -> impl Parser<char, (Vec<Vec<Option<char>>>, Vec<(u8, u8, u8)>), Error = Simple<char>> {
    block_section_parser()
        // Ignore bucket numbers
        .then_ignore(
            text::whitespace()
                .then_ignore(text::int(10).padded())
                .repeated(),
        )
        .then(move_parser())
        .then_ignore(end())
}

fn move_parser() -> impl Parser<char, Vec<(u8, u8, u8)>, Error = Simple<char>> {
    (just("move")
        .padded()
        .ignore_then(text::int(10).from_str::<u8>().unwrapped())
        .then_ignore(just("from").padded())
        .then(text::int(10).from_str::<u8>().unwrapped())
        .then_ignore(just("to").padded())
        .then(text::int(10).from_str::<u8>().unwrapped())
        .map(|((a, b), c)| (a, b, c)))
    .then_ignore(text::newline())
    .repeated()
}

fn block_section_parser() -> impl Parser<char, Vec<Vec<Option<char>>>, Error = Simple<char>> {
    block_parser()
        .repeated()
        .collect::<Vec<Option<char>>>()
        .then_ignore(text::newline())
        .repeated()
}

fn block_parser() -> impl Parser<char, Option<char>, Error = Simple<char>> {
    filled_block_parser().or(empty_block_parser())
}

fn filled_block_parser() -> impl Parser<char, Option<char>, Error = Simple<char>> {
    just('[')
        .ignore_then(text::ident())
        .map(|mut str| str.pop())
        .then_ignore(just("] ").or(just("]")))
        .labelled("block")
}

fn empty_block_parser() -> impl Parser<char, Option<char>, Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_whitespace())
        .repeated()
        .exactly(4)
        .ignored()
        .map(|_| None)
        .labelled("block")
}
