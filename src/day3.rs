use std::{collections::HashSet, path::PathBuf};

use anyhow::Result;
use itertools::Itertools;

pub(crate) fn solve(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let total_sum: i32 = file
        .lines()
        .map(|line| {
            log::trace!("Raw line: {line}");

            let first_half = &line[0..line.len() / 2];
            let second_half = &line[line.len() / 2..line.len()];

            log::trace!("First half: {first_half}, second half: {second_half}");

            let items = first_half
                .chars()
                .filter(|item| second_half.chars().any(|second_item| item == &second_item));

            let item_set: HashSet<char> = items.collect();
            log::trace!("Item set: {item_set:?}");

            let rated_items: Vec<i32> = item_set
                .into_iter()
                .map(|char| {
                    let value = (char as i32) - 96;

                    if value > 0 {
                        value
                    } else {
                        value + 58
                    }
                })
                .collect();
            log::debug!("Rated items: {rated_items:?}");

            rated_items[0]
        })
        .sum();

    log::info!("Total sum: {total_sum}");

    Ok(())
}

pub(crate) fn solve_b(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let total_sum: i32 = file
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut bags| {
            if let Some((bag_1, bag_2, bag_3)) = bags.next_tuple() {
                log::trace!("Raw bags: {bag_1} | {bag_2} | {bag_3}");

                let items = bag_1
                    .chars()
                    .filter(|item| bag_2.chars().any(|second_item| item == &second_item))
                    .filter(|item| bag_3.chars().any(|second_item| item == &second_item));

                let item_set: HashSet<char> = items.collect();
                log::trace!("Item set: {item_set:?}");

                let rated_items: Vec<i32> = item_set
                    .into_iter()
                    .map(|char| {
                        let value = (char as i32) - 96;

                        if value > 0 {
                            value
                        } else {
                            value + 58
                        }
                    })
                    .collect();
                log::debug!("Rated items: {rated_items:?}");

                rated_items[0]
            } else {
                0
            }
        })
        .sum();

    log::info!("Total sum: {total_sum}");

    Ok(())
}
