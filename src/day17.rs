use std::path::PathBuf;

use anyhow::Result;
use chumsky::prelude::*;

pub(crate) fn solve(path: PathBuf, rock_count: usize) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let parsed_file = file_parser()
        .parse(file)
        .map_err(|err| anyhow::anyhow!("An error occurred while parsing the file: {err:?}"))?;

    // -- Task 1 --
    let mut move_instructions = parsed_file.into_iter().cycle();

    let mut max_heights: [usize; 7] = [0; 7];

    // I could probably change this to not be a vector of bool slices but
    // instead a vector over u8. Then performing bit operations on those would
    // be possible.
    let mut grid: Vec<[bool; 7]> = vec![[false; 7]; rock_count * 4 + 4];
    grid[0] = [true; 7];

    for rock_number in 0..rock_count {
        let shape: Shape = rock_number.into();

        let mut left_offset = 2;
        // shape height = max value from heights + 3
        let mut shape_height = max_heights.iter().max().copied().unwrap_or(0) + 4; // + 4 because counting from 0

        // -- Falling loop --
        loop {
            let move_direction = move_instructions.next().unwrap();

            log::trace!("Move: {move_direction:?}");
            log::trace!("Pre {shape:?} - x: {left_offset} y: {shape_height}");

            // Move rock based on jet of gas
            let potential_new_offset = match move_direction {
                Direction::Left if 0 < left_offset => left_offset - 1,
                Direction::Right if left_offset + shape.width() < max_heights.len() => {
                    left_offset + 1
                }
                _ => left_offset,
            };
            let potential_collision_points = shape.points(potential_new_offset, shape_height);
            let no_collision = potential_collision_points
                .into_iter()
                .all(|(x, y)| !grid[y][x]);
            // If no collision when moving left/right move one unit in left / right direction
            if no_collision {
                left_offset = potential_new_offset;
            }

            log::trace!("Post {shape:?} - x: {left_offset} y: {shape_height}");

            // Check if all lowest points of shape do not collide with highest shapes below their (x, y) coordinates
            let potential_collision_points = shape.points(left_offset, shape_height - 1);
            let any_collision = potential_collision_points
                .into_iter()
                .any(|(x, y)| grid[y][x]);

            // If at least one collides: stop moving the rock, update heights and break loop & continue to next rock
            if any_collision {
                for (x, y) in shape.points(left_offset, shape_height) {
                    max_heights[x] = max_heights[x].max(y);
                    grid[y][x] = true;
                }

                log::trace!("max_heights: {max_heights:?}");

                break;
            } else {
                // Else decrease the height
                shape_height -= 1;
            }
        }
    }

    log::debug!("max_heights: {max_heights:?}");

    let Some(task_a) = max_heights.iter().max() else { anyhow::bail!("Could not get max height for task a") };
    log::info!("task_a solution: {task_a:?}");

    // -- Task 2 --
    // As the left/right move operations and shapes repeat, there is a sequence to be found.
    // Once one determines the sequence and the height of it, one can multiply it to the closest
    // number to 1000000000000 and simulate the last few remaining steps to be performed to obtain a score

    Ok(())
}

#[derive(Debug)]
enum Shape {
    HorizontalLine,
    Plus,
    ReverseL,
    VerticalLine,
    Square,
}

impl From<usize> for Shape {
    fn from(value: usize) -> Self {
        match value % 5 {
            0 => Shape::HorizontalLine,
            1 => Shape::Plus,
            2 => Shape::ReverseL,
            3 => Shape::VerticalLine,
            4 => Shape::Square,
            _ => panic!("This shouldn't be reachable"),
        }
    }
}

impl Shape {
    fn points(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match self {
            Shape::HorizontalLine => (x..x + 4).map(|x| (x, y)).collect(),
            Shape::Plus => vec![
                (x + 1, y + 2), // Top point
                (x, y + 1),     // Left point
                (x + 1, y + 1), // Center point
                (x + 2, y + 1), // Right point
                (x + 1, y),     // Bottom point
            ],
            Shape::ReverseL => vec![
                (y..=y + 2)
                    .map(|y| (x + 2, y))
                    .collect::<Vec<(usize, usize)>>(), // Right side
                (x..x + 2).map(|x| (x, y)).collect::<Vec<(usize, usize)>>(), // Bottom row
            ]
            .into_iter()
            .flatten()
            .collect(),
            Shape::VerticalLine => (y..y + 4).map(|y| (x, y)).collect(),
            Shape::Square => (x..=x + 1).flat_map(|x| vec![(x, y), (x, y + 1)]).collect(),
        }
    }

    fn width(&self) -> usize {
        match self {
            Shape::HorizontalLine => 4,
            Shape::Plus => 3,
            Shape::ReverseL => 3,
            Shape::VerticalLine => 1,
            Shape::Square => 2,
        }
    }
}

// --- Parser

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

fn file_parser() -> impl Parser<char, Vec<Direction>, Error = Simple<char>> {
    (just('<').map(|_| Direction::Left))
        .or(just('>').map(|_| Direction::Right))
        .repeated()
        .padded()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = include_str!("../tasks/day17_dev.txt");

    #[test]
    fn test_line_parser() {
        let line = ">>><";

        let parsed_line = file_parser().parse(line);
        assert!(parsed_line.is_ok());

        assert_eq!(
            parsed_line.unwrap(),
            vec![
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Left,
            ]
        );
    }

    #[test]
    fn test_file_parser() {
        let parsed_file = file_parser().parse(TEST_FILE);
        assert!(parsed_file.is_ok());
    }
}
