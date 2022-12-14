use std::path::PathBuf;

use anyhow::Result;
use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum Entry {
    Air,
    Rock,
    Sand,
    // Tilde,
}

pub(crate) fn solve(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let parsed_file = file_parser()
        .parse(file)
        .map_err(|err| anyhow::anyhow!("An error occurred while parsing the file: {err:?}"))?;

    let Some(min_y) = parsed_file
        .iter()
        .flatten()
        .map(|item| item.0 .1.min(item.1 .1))
        .min() else {
            anyhow::bail!("Could not determine max_y")
        };
    log::debug!("min_y: {min_y:?}");
    let Some(max_y) = parsed_file
        .iter()
        .flatten()
        .map(|item| item.0 .1.max(item.1 .1))
        .max() else {
            anyhow::bail!("Could not determine max_y")
        };
    log::debug!("max_y: {max_y:?}");

    let Some(min_x) = parsed_file
        .iter()
        .flatten()
        .map(|item| item.0 .0.min(item.1 .0))
        .min() else {
            anyhow::bail!("Could not determine max_x")
        };
    log::debug!("min_x: {min_x:?}");
    let Some(max_x) = parsed_file
        .iter()
        .flatten()
        .map(|item| item.0 .0.max(item.1 .0))
        .max() else {
            anyhow::bail!("Could not determine max_x")
        };
    log::debug!("max_x: {max_x:?}");

    let grid_height = max_y + 3;
    let grid_width = grid_height * 2 + 1;
    let offset = grid_width / 2 - (500 - min_x);
    let mut grid: Vec<Vec<Entry>> = vec![vec![Entry::Air; grid_width]; grid_height];

    for line in &parsed_file {
        for pair in line {
            let (start, end) = pair;

            for x in start.0.min(end.0)..=end.0.max(start.0) {
                for y in start.1.min(end.1)..=end.1.max(start.1) {
                    grid[y][x + offset - min_x] = Entry::Rock;
                }
            }
        }
    }
    // draw_grid_trace(&grid);

    let starting_point: Coordinate = (grid_width / 2, 0);
    fill_grid(&mut grid, starting_point);

    // draw_tildes(&mut grid, starting_point);
    // draw_grid(&grid);

    let task_1 = grid
        .iter()
        .flatten()
        .filter(|entry| &&Entry::Sand == entry)
        .count();
    log::info!("task_1: {task_1}");

    // -- task 2 --
    let floor_height = max_y + 2;
    grid[floor_height] = vec![Entry::Rock; grid_width];
    fill_grid(&mut grid, starting_point);

    // draw_tildes(&mut grid, starting_point);
    // draw_grid(&grid);

    let task_2 = grid
        .iter()
        .flatten()
        .filter(|entry| &&Entry::Sand == entry)
        .count();
    log::info!("task_2: {}", task_2);

    Ok(())
}

fn fill_grid(grid: &mut Vec<Vec<Entry>>, starting_point: (usize, usize)) {
    'outer: loop {
        let mut point = (starting_point.0, starting_point.1);

        'inner: loop {
            point.1 += 1;

            let Some(block) = grid.get(point.1).and_then(|row|row.get(point.0)) else {
                break 'outer;
            };

            if block == &Entry::Rock || block == &Entry::Sand {
                let block_below_left = grid.get(point.1).and_then(|row| row.get(point.0 - 1));
                let block_below_right = grid.get(point.1).and_then(|row| row.get(point.0 + 1));

                match (block_below_left, block_below_right) {
                    (Some(Entry::Air), _) => point.0 -= 1,
                    (_, Some(Entry::Air)) => point.0 += 1,

                    _ => {
                        point.1 -= 1;

                        (*grid)[point.1][point.0] = Entry::Sand;
                        // draw_grid_trace(grid);
                        break 'inner;
                    }
                }
            }
        }

        if point.1 == starting_point.1 {
            // draw_grid_trace(grid);
            break;
        }

        if point == (starting_point.0, starting_point.1) {
            break;
        }
    }
}

// fn draw_tildes(grid: &mut Vec<Vec<Entry>>, starting_point: (usize, usize)) {
//     let mut point = starting_point;
//     loop {
//         (*grid)[point.1][point.0] = Entry::Tilde;

//         let Some(block_below) = grid
//                 .get(point.1 + 1)
//                 .and_then(|row| row.get(point.0)) else {
//                     break;
//                 };

//         // If below point not blocked, continue down
//         if &Entry::Air == block_below {
//             point.1 += 1;
//             (*grid)[point.1][point.0] = Entry::Tilde;
//             continue;
//         }

//         let Some(block_below_left) = grid
//             .get(point.1 + 1)
//             .and_then(|row| row.get(point.0 - 1)) else {
//                 break ;
//             };

//         // If below left point not blocked, continue down
//         if &Entry::Air == block_below_left {
//             point.1 += 1;
//             point.0 -= 1;

//             (*grid)[point.1][point.0] = Entry::Tilde;

//             continue;
//         }

//         let Some(blocked_below_right) = grid
//                 .get(point.1 + 1)
//                 .and_then(|row| row.get(point.0 + 1)) else {
//                     break ;
//                 };

//         // If below right point not blocked, continue down
//         if &Entry::Air == blocked_below_right {
//             point.1 += 1;
//             point.0 += 1;

//             (*grid)[point.1][point.0] = Entry::Tilde;

//             continue;
//         }

//         (*grid)[point.1][point.0] = Entry::Tilde;

//         break;
//     }
// }

// fn draw_grid_trace(grid: &Vec<Vec<Entry>>) {
//     log::trace!(target: "aoc2022::day14::grid", "");
//     for line in grid {
//         let string_line = line
//             .iter()
//             .map(|value| match value {
//                 Entry::Air => '.',
//                 Entry::Rock => '#',
//                 Entry::Sand => 'o',
//                 Entry::Tilde => '~',
//             })
//             .collect::<String>();

//         log::trace!(target: "aoc2022::day14::grid", "{string_line}");
//     }
//     log::trace!(target: "aoc2022::day14::grid","");
// }

// fn draw_grid(grid: &Vec<Vec<Entry>>) {
//     log::debug!(target: "aoc2022::day14::grid", "");
//     for line in grid {
//         let string_line = line
//             .iter()
//             .map(|value| match value {
//                 Entry::Air => '.',
//                 Entry::Rock => '#',
//                 Entry::Sand => 'o',
//                 Entry::Tilde => '~',
//             })
//             .collect::<String>();

//         log::debug!(target: "aoc2022::day14::grid", "{string_line}");
//     }
//     log::debug!(target: "aoc2022::day14::grid","");
// }

// --- Parser ---

type Coordinate = (usize, usize);

fn file_parser() -> impl Parser<char, Vec<Vec<(Coordinate, Coordinate)>>, Error = Simple<char>> {
    line_parser().repeated().then_ignore(end())
}

fn line_parser() -> impl Parser<char, Vec<(Coordinate, Coordinate)>, Error = Simple<char>> {
    coordinate_parser()
        .padded()
        .separated_by(just("->"))
        .at_least(1)
        .map(|coordinates| {
            coordinates
                .windows(2)
                .into_iter()
                .map(|coordinate| (coordinate[0], coordinate[1]))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn coordinate_parser() -> impl Parser<char, Coordinate, Error = Simple<char>> {
    text::int(10)
        .separated_by(just(','))
        .at_least(1)
        .map(|coordinate| {
            (
                coordinate[0].parse::<usize>().unwrap(),
                coordinate[1].parse::<usize>().unwrap(),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = include_str!("../tasks/day14_dev.txt");

    #[test]
    fn test_coord_parser() {
        let input = "498,4";

        let parsed_line = coordinate_parser().parse(input);
        assert!(parsed_line.is_ok());

        assert_eq!(parsed_line.unwrap(), (498, 4));
    }

    #[test]
    fn test_line_parser() {
        let input = "498,4 -> 498,6 -> 496,6";

        let parsed_line = line_parser().parse(input);
        assert!(parsed_line.is_ok());

        assert_eq!(
            parsed_line.unwrap(),
            vec![((498, 4), (498, 6)), ((498, 6), (496, 6)),]
        );
    }

    #[test]
    fn test_file_parser() {
        let parsed_file = file_parser().parse(TEST_FILE);
        assert!(parsed_file.is_ok());

        assert_eq!(
            parsed_file.unwrap(),
            vec![
                vec![((498, 4), (498, 6)), ((498, 6), (496, 6))],
                vec![
                    ((503, 4), (502, 4)),
                    ((502, 4), (502, 9)),
                    ((502, 9), (494, 9)),
                ]
            ]
        )
    }
}
