use std::{collections::HashSet, path::PathBuf};

use anyhow::Result;
use chumsky::prelude::*;

pub(crate) fn solve(path: PathBuf, knot_count: usize) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let instructions = file_parser()
        .parse(file)
        .map_err(|err| anyhow::anyhow!("An error occurred while parsing the file: {err:?}"))?;

    let mut pos_knots = vec![(0, 0); knot_count];

    let mut unique_tail_pos: HashSet<(i32, i32)> = HashSet::new();

    let mut debug_counter = 0;

    for (direction, steps) in instructions {
        for _ in 0..steps {
            if let Some(pos_head) = pos_knots.first_mut() {
                *pos_head = match direction {
                    Direction::Right => (pos_head.0 + 1, pos_head.1),
                    Direction::Left => (pos_head.0 - 1, pos_head.1),
                    Direction::Up => (pos_head.0, pos_head.1 + 1),
                    Direction::Down => (pos_head.0, pos_head.1 - 1),
                };
            }

            for i in 1..pos_knots.len() {
                let pos_prev = pos_knots[i - 1];
                let pos_current = &mut pos_knots[i];

                if !within_bounds(&pos_prev, 1, pos_current) {
                    // Vertical move
                    if pos_prev.0 == pos_current.0 {
                        if pos_prev.1 < pos_current.1 {
                            pos_current.1 -= 1;
                        } else {
                            pos_current.1 += 1;
                        }
                    // Horizontal move
                    } else if pos_prev.1 == pos_current.1 {
                        if pos_prev.0 < pos_current.0 {
                            pos_current.0 -= 1;
                        } else {
                            pos_current.0 += 1;
                        }
                    } else {
                        // Diagonal move
                        *pos_current =
                            match (pos_prev.0 > pos_current.0, pos_prev.1 > pos_current.1) {
                                (true, true) => (pos_current.0 + 1, pos_current.1 + 1),
                                (true, false) => (pos_current.0 + 1, pos_current.1 - 1),
                                (false, true) => (pos_current.0 - 1, pos_current.1 + 1),
                                (false, false) => (pos_current.0 - 1, pos_current.1 - 1),
                            };
                    }
                }
            }

            if let Some(pos_tail) = pos_knots.last() {
                unique_tail_pos.insert(*pos_tail);
            }

            log::trace!("{debug_counter}: pos_knots: {pos_knots:?}");
            debug_counter += 1;
        }
    }

    log::debug!("unique_tail_pos: {unique_tail_pos:?}");

    log::info!("Unique tail positions: {}", unique_tail_pos.len());

    Ok(())
}

fn within_bounds(source: &(i32, i32), size: i32, target: &(i32, i32)) -> bool {
    (source.0 - target.0).abs() <= size && (source.1 - target.1).abs() <= size
}

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn file_parser() -> impl Parser<char, Vec<(Direction, u8)>, Error = Simple<char>> {
    (just('R').padded().map(|_| Direction::Right))
        .or(just('L').padded().map(|_| Direction::Left))
        .or(just('U').padded().map(|_| Direction::Up))
        .or(just('D').padded().map(|_| Direction::Down))
        .then(text::digits(10).from_str().unwrapped())
        .padded()
        .repeated()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = include_str!("../tasks/day9_dev.txt");

    #[test]
    fn test_file_parser() {
        let parsed_file = file_parser().parse(FILE);

        assert!(parsed_file.is_ok());
        assert_eq!(
            parsed_file.unwrap(),
            vec![
                (Direction::Right, 4),
                (Direction::Up, 4),
                (Direction::Left, 3),
                (Direction::Down, 1),
                (Direction::Right, 4),
                (Direction::Down, 1),
                (Direction::Left, 5),
                (Direction::Right, 2),
            ]
        );
    }
}
