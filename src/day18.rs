use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
    sync::Arc,
};

use anyhow::Result;

pub(crate) fn solve(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let cubes = file_parser(&file);

    // --- Task 1 ---
    let mut side_count = cubes.len() * 6;
    let mut cube_set: HashSet<(i8, i8, i8)> = HashSet::new();

    for cube in &cubes {
        for i in [-1, 1] {
            if cube_set.contains(&(cube.0 + i, cube.1, cube.2)) {
                side_count -= 2;
            }
            if cube_set.contains(&(cube.0, cube.1 + i, cube.2)) {
                side_count -= 2;
            }
            if cube_set.contains(&(cube.0, cube.1, cube.2 + i)) {
                side_count -= 2;
            }
        }

        cube_set.insert(*cube);
    }
    log::info!("task 1: {side_count}");

    // --- Task 2 ---
    let max_x: usize = cubes
        .iter()
        .map(|(x, _, _)| *x)
        .max()
        .unwrap()
        .try_into()
        .unwrap();
    let max_y: usize = cubes
        .iter()
        .map(|(_, y, _)| *y)
        .max()
        .unwrap()
        .try_into()
        .unwrap();
    let max_z: usize = cubes
        .iter()
        .map(|(_, _, z)| *z)
        .max()
        .unwrap()
        .try_into()
        .unwrap();
    let mut block_grid: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![false; max_x + 1]; max_y + 1]; max_z + 1];

    for (x, y, z) in cubes {
        block_grid[z as usize][y as usize][x as usize] = true;
    }

    // Perform a BFS
    let mut reachable_coordinates: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![false; max_x + 1]; max_y + 1]; max_z + 1];

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));

    while let Some((x, y, z)) = queue.pop_front() {
        if !block_grid[z][y][x] && !reachable_coordinates[z][y][x] {
            if x + 1 <= max_x {
                queue.push_back((x + 1, y, z));
            }
            if y + 1 <= max_y {
                queue.push_back((x, y + 1, z));
            }
            if z + 1 <= max_z {
                queue.push_back((x, y, z + 1));
            }
        }

        reachable_coordinates[z][y][x] = true;
    }

    let block_grid = Arc::new(block_grid);

    let enclosed_sides: usize = reachable_coordinates
        .into_iter()
        .enumerate()
        .flat_map(|(z, vec)| {
            let z = z.clone();
            let block_grid = block_grid.clone();

            vec.into_iter().enumerate().flat_map(move |(y, vec)| {
                let y = y.clone();
                let block_grid = block_grid.clone();

                vec.into_iter().enumerate().map(move |(x, value)| {
                    if !value && !block_grid[z][y][x] {
                        let mut side_count = 0;

                        if x > 0 && block_grid[z][y][x - 1] {
                            side_count += 1;
                        }
                        if x + 1 <= max_x && block_grid[z][y][x + 1] {
                            side_count += 1;
                        }

                        if y > 0 && block_grid[z][y - 1][x] {
                            side_count += 1;
                        }
                        if y + 1 <= max_y && block_grid[z][y + 1][x] {
                            side_count += 1;
                        }

                        if z > 0 && block_grid[z - 1][y][x] {
                            side_count += 1;
                        }
                        if z + 1 <= max_z && block_grid[z + 1][y][x] {
                            side_count += 1;
                        }

                        side_count
                    } else {
                        0
                    }
                })
            })
        })
        .sum();
    log::debug!("enclosed_sides: {enclosed_sides}");
    log::info!("task 2: {}", side_count - enclosed_sides);

    Ok(())
}

// --- Parser ---

fn file_parser(file: &str) -> Vec<(i8, i8, i8)> {
    file.split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| row.split(',').collect::<Vec<&str>>())
        .map(|indices| {
            (
                indices[0].parse().unwrap(),
                indices[1].parse().unwrap(),
                indices[2].parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = include_str!("../tasks/day18_dev.txt");

    #[test]
    fn test_line_parser() {
        let line = "2,2,2";

        let parsed_line = file_parser(line);
        assert_eq!(parsed_line, vec![(2, 2, 2)]);
    }

    #[test]
    fn test_file_parser() {
        let parsed_file = file_parser(TEST_FILE);
        assert_eq!(
            parsed_file,
            vec![
                (2, 2, 2),
                (1, 2, 2),
                (3, 2, 2),
                (2, 1, 2),
                (2, 3, 2),
                (2, 2, 1),
                (2, 2, 3),
                (2, 2, 4),
                (2, 2, 6),
                (1, 2, 5),
                (3, 2, 5),
                (2, 1, 5),
                (2, 3, 5),
            ]
        )
    }
}
