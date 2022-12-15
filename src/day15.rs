use std::{cmp::Ordering, path::PathBuf};

use anyhow::Result;
use chumsky::prelude::*;
use itertools::Itertools;
use rayon::prelude::*;

pub(crate) fn solve(path: PathBuf, y: i32, xy_limit: i32) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let parsed_file = file_parser()
        .parse(file)
        .map_err(|err| anyhow::anyhow!("An error occurred while parsing the file: {err:?}"))?;

    // --- task a ---

    let beacons: Vec<Coordinate> = parsed_file.iter().map(|(_, beacon)| *beacon).collect();

    let filtered_sensors: Vec<(Coordinate, u32)> = parsed_file
        .iter()
        .map(|((s_x, s_y), (b_x, b_y))| ((*s_x, *s_y), s_x.abs_diff(*b_x) + s_y.abs_diff(*b_y)))
        .filter(|((_, s_y), distance)| s_y.abs_diff(y) <= *distance)
        .collect();

    let task_a_count: usize = (i32::MIN..i32::MAX)
        .into_par_iter()
        .map(|x| {
            if beacons.contains(&(x, y)) {
                return false;
            };

            filtered_sensors
                .iter()
                .any(|((s_x, s_y), distance)| (s_x.abs_diff(x) + s_y.abs_diff(y)) <= *distance)
        })
        .filter(|inside| *inside)
        .count();
    log::info!("task_a_count: {task_a_count:?}");

    // --- task b ---

    let all_sensors: Vec<(Coordinate, u32)> = parsed_file
        .iter()
        .map(|&((s_x, s_y), (b_x, b_y))| ((s_x, s_y), s_x.abs_diff(b_x) + s_y.abs_diff(b_y)))
        .collect();

    let non_continuous_ranges_y: Vec<(i32, i32)> = (0..xy_limit)
        .map(|y| {
            let ranges = all_sensors
                .iter()
                .filter_map(|&((s_x, s_y), distance)| {
                    let delta_x = (distance - s_y.abs_diff(y)) as i32;

                    if delta_x >= 0 {
                        Some(((s_x - delta_x), (s_x + delta_x)))
                    } else {
                        None
                    }
                })
                .sorted_by(|a, b| {
                    let cmp = a.0.cmp(&(b.0));

                    if Ordering::Equal == cmp {
                        a.1.cmp(&(b.1))
                    } else {
                        cmp
                    }
                });

            (y, ranges)
        })
        .filter_map(|(y, ranges)| {
            let mut ranges = ranges.clone();
            let Some(range) = ranges.next() else {
                return None;
            };

            let mut min = range.0;
            let mut max = range.1;

            for range in ranges {
                let start = range.0;
                let end = range.1;

                if (min <= start && start <= max) || (min <= start - 1 && start - 1 <= max) {
                    min = min.min(start);
                    max = max.max(end);
                } else {
                    return Some((max + 1, y));
                }
            }

            return None;
        })
        .collect();

    let score: u64 =
        (non_continuous_ranges_y[0].0 as u64) * 4000000 + (non_continuous_ranges_y[0].1 as u64);
    log::info!("task_b score: {score}");

    Ok(())
}

// --- Parser ---

type Coordinate = (i32, i32);

fn file_parser() -> impl Parser<char, Vec<(Coordinate, Coordinate)>, Error = Simple<char>> {
    line_parser().repeated()
}

fn line_parser() -> impl Parser<char, (Coordinate, Coordinate), Error = Simple<char>> {
    just("Sensor at x=")
        .ignored()
        .then(text::int(10))
        .try_map(|(_, x): ((), String), span| {
            x.parse::<i32>()
                .map_err(|e| Simple::custom(span, format!("{}", e)))
        })
        .then_ignore(just(',').padded())
        .then_ignore(just("y="))
        .then(text::int(10))
        .try_map(|(x, y), span| {
            let y = y
                .parse::<i32>()
                .map_err(|e| Simple::custom(span, format!("{}", e)))?;

            Ok((x, y))
        })
        .then_ignore(just(": closest beacon is at x="))
        .then(take_until(just(',')))
        .padded()
        .try_map(|(sensor_coordinate, (x, _)), span| {
            let x = x
                .into_iter()
                .collect::<String>()
                .parse::<i32>()
                .map_err(|e| Simple::custom(span, format!("{}", e)))?;

            Ok((sensor_coordinate, (x)))
        })
        .then_ignore(just("y="))
        .then(take_until(text::newline().or(end())))
        .try_map(|((sensor_coordinate, x), (y, _)), span| {
            let y = y
                .into_iter()
                .collect::<String>()
                .parse::<i32>()
                .map_err(|e| Simple::custom(span, format!("{}", e)))?;

            Ok((sensor_coordinate, (x, y)))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = include_str!("../tasks/day15_dev.txt");

    #[test]
    fn test_line_parser() {
        let line = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";

        let parsed_line = line_parser().parse(line);
        assert!(parsed_line.is_ok());

        assert_eq!(parsed_line.unwrap(), ((2, 18), (-2, 15)));
    }

    #[test]
    fn test_file_parser() {
        let parsed_file = file_parser().parse(TEST_FILE);
        assert!(parsed_file.is_ok());

        assert_eq!(
            parsed_file.unwrap(),
            vec![
                ((2, 18), (-2, 15)),
                ((9, 16), (10, 16)),
                ((13, 2), (15, 3)),
                ((12, 14), (10, 16)),
                ((10, 20), (10, 16)),
                ((14, 17), (10, 16)),
                ((8, 7), (2, 10)),
                ((2, 0), (2, 10)),
                ((0, 11), (2, 10)),
                ((20, 14), (25, 17)),
                ((17, 20), (21, 22)),
                ((16, 7), (15, 3)),
                ((14, 3), (15, 3)),
                ((20, 1), (15, 3)),
            ]
        );
    }
}
