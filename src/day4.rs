use std::{num::ParseIntError, path::PathBuf};

use anyhow::Result;

pub(crate) fn solve(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let results_iter = file.lines().map(|pairs| {
        let mut pairs = pairs.split(',');

        let elf_1 = pairs.next().map(|entry| {
            let mut entries = entry.split('-');

            (entries.next(), entries.next())
        });
        let elf_2 = pairs.next().map(|entry| {
            let mut entries = entry.split('-');

            (entries.next(), entries.next())
        });

        log::trace!("elf_1: {elf_1:?} | elf_2: {elf_2:?}");

        if let Some((Some(elf_1_min), Some(elf_1_max))) = elf_1 {
            if let Some((Some(elf_2_min), Some(elf_2_max))) = elf_2 {
                let elf_1_min: Result<u8, ParseIntError> = elf_1_min.parse();
                let elf_1_max: Result<u8, ParseIntError> = elf_1_max.parse();
                let elf_2_min: Result<u8, ParseIntError> = elf_2_min.parse();
                let elf_2_max: Result<u8, ParseIntError> = elf_2_max.parse();

                if let (Ok(elf_1_min), Ok(elf_1_max), Ok(elf_2_min), Ok(elf_2_max)) =
                    (elf_1_min, elf_1_max, elf_2_min, elf_2_max)
                {
                    log::debug!(
                        "elf_1: {elf_1_min}, {elf_1_max} | elf_2: {elf_2_min}, {elf_2_max}"
                    );

                    let includes = elf_1_min <= elf_2_min && elf_1_max >= elf_2_max
                        || elf_2_min <= elf_1_min && elf_2_max >= elf_1_max;

                    let overlaps = !(elf_1_max < elf_2_min || elf_2_max < elf_1_min);

                    return (includes, overlaps);
                }
            }
        }

        (false, false)
    });

    let total_pair_intersections = results_iter.clone().filter(|result| result.0).count();
    log::info!("Total pair intersections: {total_pair_intersections}");

    let total_pair_overlaps = results_iter.filter(|result| result.1).count();
    log::info!("Total pair overlaps: {total_pair_overlaps}");

    Ok(())
}
