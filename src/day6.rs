use std::{collections::HashSet, path::PathBuf};

use anyhow::Result;

pub(crate) fn solve(path: PathBuf, window_size: usize) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let chars: Vec<char> = file.as_str().chars().collect();

    let mut first_index: Option<usize> = None;
    for (idx, value) in chars.windows(window_size).enumerate() {
        let set: HashSet<_> = HashSet::from_iter(value);

        log::debug!("Hash set: {set:?}");

        if set.len() == window_size {
            first_index = Some(idx);
            break;
        }
    }

    log::info!(
        "First start-of-packet marker detected at {:?}",
        first_index.map(|idx| idx + window_size)
    );

    Ok(())
}
