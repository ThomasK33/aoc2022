use std::path::PathBuf;

use anyhow::Result;

pub(crate) fn solve(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let heights: Vec<Vec<u32>> = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    log::trace!("heights: {heights:#?}");

    let l_to_r_height: Vec<Vec<u32>> = heights
        .iter()
        .map(|line| {
            line.iter()
                .fold((vec![], 0), |(mut acc, mut max), &value| {
                    acc.push(max);

                    if value > max {
                        max = value;
                    }

                    (acc, max)
                })
                .0
        })
        .collect();
    log::debug!("l_to_r_height: {l_to_r_height:?}");

    let r_to_l_height: Vec<Vec<u32>> = heights
        .iter()
        .map(|line| {
            line.iter()
                .rev()
                .fold((vec![], 0), |(mut acc, mut max), &value| {
                    acc.push(max);

                    if value > max {
                        max = value;
                    }

                    (acc, max)
                })
                .0
                .into_iter()
                .rev()
                .collect()
        })
        .collect();
    log::debug!("r_to_l_height: {r_to_l_height:?}");

    let transposed_heights = transpose(heights.clone());
    log::trace!("transposed_heights: {transposed_heights:#?}");

    let t_to_b_height: Vec<Vec<u32>> = transposed_heights
        .iter()
        .map(|line| {
            line.iter()
                .fold((vec![], 0), |(mut acc, mut max), &value| {
                    acc.push(max);

                    if value > max {
                        max = value;
                    }

                    (acc, max)
                })
                .0
        })
        .collect();
    log::debug!("t_to_b_height: {t_to_b_height:?}");

    let b_to_t_height: Vec<Vec<u32>> = transposed_heights
        .iter()
        .map(|line| {
            line.iter()
                .rev()
                .fold((vec![], 0), |(mut acc, mut max), &value| {
                    acc.push(max);

                    if value > max {
                        max = value;
                    }

                    (acc, max)
                })
                .0
                .into_iter()
                .rev()
                .collect()
        })
        .collect();
    log::debug!("b_to_t_height: {b_to_t_height:?}");

    let map_size = heights.len();
    let visible_tree_count = heights
        .iter()
        .enumerate()
        .flat_map(|(idx, line)| {
            line.iter()
                .enumerate()
                .map(|(idy, &value)| {
                    idx == 0
                        || idy == 0
                        || idx == map_size - 1
                        || idy == map_size - 1
                        || l_to_r_height[idx][idy] < value
                        || r_to_l_height[idx][idy] < value
                        || t_to_b_height[idy][idx] < value
                        || b_to_t_height[idy][idx] < value
                })
                .collect::<Vec<_>>()
        })
        .filter(|&value| value)
        .count();
    log::info!("visible_tree_count: {visible_tree_count}");

    let scenic_scores = heights
        .iter()
        .enumerate()
        .map(|(idy, line)| {
            line.iter()
                .enumerate()
                .map(|(idx, &value)| {
                    if idx == 0 || idy == 0 || idx == map_size - 1 || idy == map_size - 1 {
                        return 0;
                    }

                    let mut l = 0;
                    let mut r = 0;
                    let mut u = 0;
                    let mut d = 0;

                    for index in (0..idx).rev() {
                        l += 1;

                        if heights[idy][index] >= value {
                            break;
                        }
                    }

                    for index in (idx + 1)..map_size {
                        r += 1;

                        if heights[idy][index] >= value {
                            break;
                        }
                    }

                    for index in (0..idy).rev() {
                        u += 1;

                        if heights[index][idx] >= value {
                            break;
                        }
                    }

                    for line in heights.iter().take(map_size).skip(idy + 1) {
                        d += 1;

                        if line[idx] >= value {
                            break;
                        }
                    }

                    l * r * u * d
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    log::trace!("scenic_scores: {scenic_scores:#?}");

    let max_scenic_score = scenic_scores.into_iter().flatten().max();
    log::info!("max_scenic_score: {max_scenic_score:?}");

    Ok(())
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
