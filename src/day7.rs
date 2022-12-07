use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use chumsky::{prelude::*, Parser};

pub(crate) fn solve(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let command_outputs = file_parser()
        .parse(file.as_str())
        .map_err(|err| anyhow::anyhow!("An error occurred while parsing the file: {err:?}"))?;
    log::trace!("Parsed file: {:?}", command_outputs);

    let mut cwd = Arc::new(Mutex::new(PathBuf::from_str("/")?));
    let mut individual_folder_file_size_map = BTreeMap::new();

    for output in command_outputs {
        match output {
            CommandOutput::Cd(dir_name) => {
                match dir_name.as_str() {
                    "/" => {
                        cwd = Arc::new(Mutex::new(PathBuf::from_str("/")?));
                    }
                    ".." => {
                        cwd.lock().unwrap().pop();
                    }
                    dir_name => {
                        cwd.lock().unwrap().push(dir_name);
                    }
                };

                log::trace!("Current cwd: {:?}", cwd.lock().unwrap().to_str());
            }
            CommandOutput::Ls(entries) => {
                let current_cwd = cwd.lock().unwrap().to_str().unwrap().to_owned();

                let folder_size: usize = entries
                    .into_iter()
                    .filter_map(|entry| {
                        if let DirectoryEntry::File(_, size) = entry {
                            Some(size)
                        } else {
                            None
                        }
                    })
                    .sum();

                log::trace!("current_cwd: {current_cwd:?} --> {folder_size}");
                individual_folder_file_size_map.insert(current_cwd, folder_size);
            }
        }
    }

    log::debug!("folder_size_map: {individual_folder_file_size_map:#?}");

    let keys = individual_folder_file_size_map.keys();
    log::trace!("folder keys: {keys:?}");

    let mut total_folder_size_map: HashMap<&str, usize> = HashMap::new();
    for key_prefix in keys {
        let total_folder_size: usize = individual_folder_file_size_map
            .range(key_prefix.clone()..)
            .take_while(|(key, _)| key.starts_with(key_prefix))
            .map(|(_, v)| v)
            .sum();

        total_folder_size_map.insert(key_prefix, total_folder_size);
    }
    log::debug!("total_folder_size_map: {total_folder_size_map:#?}");

    let task_a: usize = total_folder_size_map
        .iter()
        .map(|(_, size)| size)
        .filter(|&&size| size < 100000)
        .sum();
    log::info!("Task a solution: {task_a}");

    let space_left = total_folder_size_map
        .get("/")
        .map(|used| 70000000 - used)
        .unwrap_or_default();
    log::debug!("Space left on device: {space_left}");

    let space_needed_for_update = 30000000;
    let additional_free_space_needed = space_needed_for_update - space_left;
    log::debug!("additional_free_space_needed: {additional_free_space_needed}");

    let task_b = total_folder_size_map
        .iter()
        .map(|(_, size)| *size)
        .filter(|&size| size >= additional_free_space_needed)
        .min()
        .unwrap_or_default();
    log::info!("Task b solution: {task_b:?}");

    Ok(())
}

// --- Parser ---

#[derive(Debug, PartialEq)]
enum CommandOutput {
    Cd(String),
    Ls(Vec<DirectoryEntry>),
}

fn file_parser() -> impl Parser<char, Vec<CommandOutput>, Error = Simple<char>> {
    cd_line_parser()
        .map(CommandOutput::Cd)
        .or(ls_lines_parser().map(CommandOutput::Ls))
        .repeated()
}

fn cd_line_parser() -> impl Parser<char, String, Error = Simple<char>> {
    just("$ cd")
        .padded()
        .ignore_then(take_until(text::newline()))
        .map(|(preceding, _)| preceding)
        .collect()
        .labelled("cd")
}

fn ls_lines_parser() -> impl Parser<char, Vec<DirectoryEntry>, Error = Simple<char>> {
    just("$ ls")
        .padded()
        .then(dir_line_parser().or(file_line_parser()).repeated())
        .map(|(_, o)| o)
        .collect()
}

#[derive(Debug, PartialEq)]
enum DirectoryEntry {
    Directory(String),
    File(String, usize),
}

fn dir_line_parser() -> impl Parser<char, DirectoryEntry, Error = Simple<char>> {
    just("dir")
        .padded()
        .ignore_then(take_until(text::newline()))
        .map(|(preceding, _)| preceding)
        .collect()
        .map(DirectoryEntry::Directory)
        .labelled("dir")
}

fn file_line_parser() -> impl Parser<char, DirectoryEntry, Error = Simple<char>> {
    text::digits(10)
        .padded()
        .then(take_until(text::newline()).map(|(preceding, _)| preceding))
        .map(|(size, name)| {
            DirectoryEntry::File(
                name.into_iter().collect::<String>(),
                size.parse::<usize>().unwrap(),
            )
        })
        .labelled("file")
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cd_line_parsing() {
        let line = "$ cd /test\n";
        let res = cd_line_parser().parse(line);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "/test".to_owned());
    }

    #[test]
    fn test_dir_line_parsing() {
        let line = "dir abcd\n";
        let res = dir_line_parser().parse(line);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), DirectoryEntry::Directory("abcd".to_owned()));
    }

    #[test]
    fn test_file_line_parsing() {
        let line = "14848514 b.txt\n";
        let res = file_line_parser().parse(line);

        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            DirectoryEntry::File("b.txt".to_owned(), 14848514)
        );
    }

    #[test]
    fn test_ls_lines_parsing() {
        let lines = r#"
		$ ls
		dir a
		14848514 b.txt
		8504156 c.dat
		dir d
	"#;
        let res = ls_lines_parser().parse(lines);

        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            vec![
                DirectoryEntry::Directory("a".to_owned()),
                DirectoryEntry::File("b.txt".to_owned(), 14848514),
                DirectoryEntry::File("c.dat".to_owned(), 8504156),
                DirectoryEntry::Directory("d".to_owned()),
            ]
        );
    }

    #[test]
    fn test_file_parsing() {
        let lines = include_str!("../tasks/day7_dev.txt");
        let res = file_parser().parse(lines);

        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            vec![
                CommandOutput::Cd("/".to_owned()),
                CommandOutput::Ls(vec![
                    DirectoryEntry::Directory("a".to_owned()),
                    DirectoryEntry::File("b.txt".to_owned(), 14848514),
                    DirectoryEntry::File("c.dat".to_owned(), 8504156),
                    DirectoryEntry::Directory("d".to_owned())
                ]),
                CommandOutput::Cd("a".to_owned()),
                CommandOutput::Ls(vec![
                    DirectoryEntry::Directory("e".to_owned()),
                    DirectoryEntry::File("f".to_owned(), 29116),
                    DirectoryEntry::File("g".to_owned(), 2557),
                    DirectoryEntry::File("h.lst".to_owned(), 62596)
                ]),
                CommandOutput::Cd("e".to_owned()),
                CommandOutput::Ls(vec![DirectoryEntry::File("i".to_owned(), 584)]),
                CommandOutput::Cd("..".to_owned()),
                CommandOutput::Cd("..".to_owned()),
                CommandOutput::Cd("d".to_owned()),
                CommandOutput::Ls(vec![
                    DirectoryEntry::File("j".to_owned(), 4060174),
                    DirectoryEntry::File("d.log".to_owned(), 8033020),
                    DirectoryEntry::File("d.ext".to_owned(), 5626152),
                    DirectoryEntry::File("k".to_owned(), 7214296)
                ])
            ]
        );
    }
}
