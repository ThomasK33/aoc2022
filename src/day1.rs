use anyhow::Result;
use std::path::PathBuf;

pub(crate) fn solve_a(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let mut calories: Vec<usize> = vec![];

    let mut counter = 0;
    for line in file.lines() {
        if line.is_empty() {
            log::trace!("Elf has {counter} calories");

            calories.push(counter);
            counter = 0;
        } else {
            let number = usize::from_str_radix(line, 10)?;
            counter += number;
        }
    }
    log::trace!("Elf has {counter} calories");
    calories.push(counter);

    log::debug!("Calories vec: {calories:?}");

    if let Some(calories) = calories
        .into_iter()
        .reduce(|acc, value| if acc >= value { acc } else { value })
    {
        log::info!("The most calories carried by one elf are: {calories}",);
    } else {
        log::error!("No elves found");
    }

    Ok(())
}

pub(crate) fn solve_b(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let mut calories: Vec<usize> = vec![];

    let mut counter = 0;
    for line in file.lines() {
        if line.is_empty() {
            log::trace!("Elf has {counter} calories");

            calories.push(counter);
            counter = 0;
        } else {
            let number = usize::from_str_radix(line, 10)?;
            counter += number;
        }
    }
    log::trace!("Elf has {counter} calories");
    calories.push(counter);

    log::debug!("Calories vec: {calories:?}");
    calories.sort_by(|a, b| b.cmp(a));
    log::debug!("Calories vec sorted: {calories:?}");

    let highest_calory_count: usize = calories[0..3].to_vec().into_iter().sum();
    log::info!("The most calories carried by three elves are: {highest_calory_count}",);

    Ok(())
}
