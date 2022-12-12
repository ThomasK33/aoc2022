use std::{collections::HashSet, path::PathBuf};

use anyhow::Result;

type Coordinate = (usize, usize);

pub(crate) fn solve(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let heights: Vec<Vec<u8>> = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'S' => 'a',
                    'E' => 'z',
                    _ => char,
                })
                .map(|char| (char as u8) - 97)
                .collect()
        })
        .collect();

    log::trace!("heights:");
    for height in &heights {
        log::trace!("{height:?}");
    }

    // Coordinate system starts at the top left
    // start - (idx, idy)
    let Some(start) = file.lines().enumerate().find_map(|(idy, line)| {
        line.char_indices()
            .find_map(|(idx, char)| if char == 'S' { Some((idx, idy)) } else { None })
    }) else {
        anyhow::bail!("Failed to find starting point in file");
    };
    log::debug!("start: {start:?}");

    // end - (idx, idy)
    let Some(end) = file.lines().enumerate().find_map(|(idy, line)| {
        line.char_indices()
            .find_map(|(idx, char)| if char == 'E' { Some((idx, idy)) } else { None })
    }) else {
        anyhow::bail!("Failed to find end point in file");
    };
    log::debug!("end: {end:?}");

    // --- breadth first search ---
    let task_a = bfs(&heights, start, end);
    log::info!("task a: {:?}", task_a);

    let shortest_path_from_any_starting_point = file
        .lines()
        .enumerate()
        .flat_map(|(idy, line)| {
            line.char_indices().filter_map(move |(idx, char)| {
                if char == 'a' || char == 'S' {
                    Some((idx, idy))
                } else {
                    None
                }
            })
        })
        .map(|coord| bfs(&heights, coord, end).unwrap_or(u32::MAX))
        .min();

    log::info!("task_b: {shortest_path_from_any_starting_point:?}");

    Ok(())
}

fn possible_coordinates(heights: &Vec<Vec<u8>>, current_position: Coordinate) -> Vec<Coordinate> {
    let mut possible_moves = vec![];

    let (x, y) = current_position;

    let is_climbable = |next_x: usize, next_y: usize| {
        let current_height: i16 = heights[y][x].into();
        let next_height: i16 = heights[next_y][next_x].into();

        current_height - next_height >= -1
    };

    if x > 0 && is_climbable(x - 1, y) {
        possible_moves.push((x - 1, y));
    }
    if x < heights[0].len() - 1 && is_climbable(x + 1, y) {
        possible_moves.push((x + 1, y));
    }

    if y > 0 && is_climbable(x, y - 1) {
        possible_moves.push((x, y - 1));
    }
    if y < heights.len() - 1 && is_climbable(x, y + 1) {
        possible_moves.push((x, y + 1));
    }

    possible_moves
}

fn bfs(heights: &Vec<Vec<u8>>, start: Coordinate, end: Coordinate) -> Option<u32> {
    let mut counter = 0;
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut next_moves: HashSet<Coordinate> = HashSet::new();
    next_moves.insert(start);

    loop {
        counter += 1;

        for next_move in &next_moves {
            visited.insert(*next_move);
        }

        next_moves = next_moves
            .iter()
            .flat_map(|&current_coordinate| possible_coordinates(heights, current_coordinate))
            .filter(|coordinate| !visited.contains(coordinate))
            .collect();

        log::trace!("next_moves: {next_moves:?}");

        if next_moves.is_empty() {
            return None;
        }

        if next_moves.contains(&end) {
            break;
        }
    }

    Some(counter)
}
