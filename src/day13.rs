use std::{cmp::Ordering, path::PathBuf};

use anyhow::Result;
use chumsky::prelude::*;
use itertools::Itertools;

pub(crate) fn solve(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let parsed_file = file_parser().parse(file).map_err(|err| {
        anyhow::anyhow!("An error occurred while trying to parse the file: {err:?}")
    })?;

    // -- Task 1 --

    let sum_of_correctly_ordered_packets = parsed_file
        .iter()
        .enumerate()
        // One has to add +1 to the array index, as the tasks counts starting from 1
        .filter_map(|(index, pair)| {
            if pair[0] < pair[1] {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum::<usize>();

    log::info!("task 1: {sum_of_correctly_ordered_packets:?}");

    // -- Task 2 --

    let divider_1 = List::List(vec![List::List(vec![List::Number(2)])]);
    let divider_2 = List::List(vec![List::List(vec![List::Number(6)])]);

    let mut parsed_file = parsed_file;
    parsed_file.push(vec![divider_1.clone(), divider_2.clone()]);

    let mut flattened_file = parsed_file.into_iter().flatten().sorted();

    let Some(index_divider_1) = flattened_file.clone().position(|list| list == divider_1) else {
        anyhow::bail!("Could not determine index of first divider");
    };
    let Some(index_divider_2) = flattened_file.position(|list| list == divider_2) else {
        anyhow::bail!("could not determine index of second divider")
    };

    log::debug!("index_divider_1: {index_divider_1:?}");
    log::debug!("index_divider_2: {index_divider_2:?}");

    let decoder_key = (index_divider_1 + 1) * (index_divider_2 + 1);

    log::info!("task 2: {decoder_key}");

    Ok(())
}

// --- Parser ---

#[derive(Clone, Debug, Eq, PartialEq)]
enum List {
    Number(u32),
    List(Vec<List>),
}

impl From<u32> for List {
    fn from(value: u32) -> Self {
        List::List(vec![List::Number(value)])
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (List::Number(this), List::Number(other)) => this.cmp(other),
            (List::Number(this), List::List(_)) => List::from(*this).cmp(other),
            (List::List(_), List::Number(other_value)) => self.cmp(&List::from(*other_value)),
            (List::List(self_list), List::List(other_list)) => {
                for i in 0..self_list.len() {
                    let Some(self_item) = self_list.get(i) else {
                        return Ordering::Less;
                    };
                    let Some(other_item) = other_list.get(i) else {
                        return Ordering::Greater;
                    };

                    match self_item.cmp(other_item) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                        Ordering::Greater => return Ordering::Greater,
                    };
                }

                if self_list.len() < other_list.len() {
                    Ordering::Less
                } else if self_list.len() == other_list.len() {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }
        }
    }
}

fn file_parser() -> impl Parser<char, Vec<Vec<List>>, Error = Simple<char>> {
    line_parser().repeated().exactly(2).repeated()
}

fn line_parser() -> impl Parser<char, List, Error = Simple<char>> {
    recursive(|list| {
        list.separated_by(just(','))
            .delimited_by(just('['), just(']'))
            .map(List::List)
            .or(text::int(10).map(|number: String| List::Number(number.parse::<u32>().unwrap())))
            .padded()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = include_str!("../tasks/day13_dev.txt");

    #[test]
    fn test_line_parser() {
        let line = "[1,1,3,1,1]";

        let parsed_line = line_parser().parse(line);
        assert!(parsed_line.is_ok());

        assert_eq!(
            parsed_line.unwrap(),
            List::List(vec![
                List::Number(1),
                List::Number(1),
                List::Number(3),
                List::Number(1),
                List::Number(1),
            ])
        );
    }

    #[test]
    fn test_line_parser_nested() {
        let line = "[[1],[2,3,4]]";

        let parsed_line = line_parser().parse(line);
        assert!(parsed_line.is_ok());

        assert_eq!(
            parsed_line.unwrap(),
            List::List(vec![
                List::List(vec![List::Number(1)]),
                List::List(vec![List::Number(2), List::Number(3), List::Number(4),]),
            ])
        );
    }

    #[test]
    fn test_line_parser_empty_nested_lists() {
        let line = "[[[]]]";

        let parsed_line = line_parser().parse(line);
        assert!(parsed_line.is_ok());

        assert_eq!(
            parsed_line.unwrap(),
            List::List(vec![List::List(vec![List::List(vec![])]),])
        );
    }

    #[test]
    fn test_multi_line_parser_empty_nested_lists() {
        let line = r#"
            [[[]]]
            [[[]]]
        "#;

        let parsed_line = line_parser().repeated().exactly(2).parse(line);
        assert!(parsed_line.is_ok());

        assert_eq!(
            parsed_line.unwrap(),
            vec![
                List::List(vec![List::List(vec![List::List(vec![])]),]),
                List::List(vec![List::List(vec![List::List(vec![])]),]),
            ]
        );
    }

    #[test]
    fn test_file_parser_two_lines_only() {
        let line = r#"
            [[[]]]
            [[[]]]
        "#;

        let parsed_line = file_parser().parse(line);
        assert!(parsed_line.is_ok());

        assert_eq!(
            parsed_line.unwrap(),
            vec![vec![
                List::List(vec![List::List(vec![List::List(vec![])]),]),
                List::List(vec![List::List(vec![List::List(vec![])]),]),
            ]]
        );
    }

    #[test]
    fn test_file_parser_on_dev_file() {
        let parsed_file = file_parser().parse(TEST_FILE);
        assert!(parsed_file.is_ok());

        assert_eq!(
            parsed_file.unwrap(),
            vec![
                vec![
                    List::List(vec![
                        List::Number(1),
                        List::Number(1),
                        List::Number(3),
                        List::Number(1),
                        List::Number(1),
                    ]),
                    List::List(vec![
                        List::Number(1),
                        List::Number(1),
                        List::Number(5),
                        List::Number(1),
                        List::Number(1),
                    ])
                ],
                vec![
                    List::List(vec![
                        List::List(vec![List::Number(1)]),
                        List::List(vec![List::Number(2), List::Number(3), List::Number(4),])
                    ],),
                    List::List(vec![List::List(vec![List::Number(1)]), List::Number(4),],),
                ],
                vec![
                    List::List(vec![List::Number(9)]),
                    List::List(vec![List::List(vec![
                        List::Number(8),
                        List::Number(7),
                        List::Number(6)
                    ])]),
                ],
                vec![
                    List::List(vec![
                        List::List(vec![List::Number(4), List::Number(4)]),
                        List::Number(4),
                        List::Number(4),
                    ]),
                    List::List(vec![
                        List::List(vec![List::Number(4), List::Number(4)]),
                        List::Number(4),
                        List::Number(4),
                        List::Number(4),
                    ]),
                ],
                vec![
                    List::List(vec![
                        List::Number(7),
                        List::Number(7),
                        List::Number(7),
                        List::Number(7),
                    ]),
                    List::List(vec![List::Number(7), List::Number(7), List::Number(7),]),
                ],
                vec![List::List(vec![]), List::List(vec![List::Number(3)])],
                vec![
                    List::List(vec![List::List(vec![List::List(vec![])])]),
                    List::List(vec![List::List(vec![])]),
                ],
                vec![
                    List::List(vec![
                        List::Number(1),
                        List::List(vec![
                            List::Number(2),
                            List::List(vec![
                                List::Number(3),
                                List::List(vec![
                                    List::Number(4),
                                    List::List(vec![
                                        List::Number(5),
                                        List::Number(6),
                                        List::Number(7),
                                    ]),
                                ]),
                            ]),
                        ]),
                        List::Number(8),
                        List::Number(9),
                    ]),
                    List::List(vec![
                        List::Number(1),
                        List::List(vec![
                            List::Number(2),
                            List::List(vec![
                                List::Number(3),
                                List::List(vec![
                                    List::Number(4),
                                    List::List(vec![
                                        List::Number(5),
                                        List::Number(6),
                                        List::Number(0),
                                    ]),
                                ]),
                            ]),
                        ]),
                        List::Number(8),
                        List::Number(9),
                    ]),
                ],
            ]
        );
    }

    #[test]
    fn test_compare_int_only() {
        let line = r#"
            1
            3
        "#;

        let parsed_line = file_parser().parse(line);
        assert!(parsed_line.is_ok());

        let parsed_line = parsed_line.unwrap();
        assert!(parsed_line[0][0] < parsed_line[0][1]);
    }

    #[test]
    fn test_compare() {
        let line = r#"
            [1,1,3,1,1]
            [1,1,5,1,1]
        "#;

        let parsed_line = file_parser().parse(line);
        assert!(parsed_line.is_ok());

        let parsed_line = parsed_line.unwrap();
        assert!(parsed_line[0][0] < parsed_line[0][1]);
    }

    #[test]
    fn test_compare_incorrect() {
        let line = r#"
            [1,1,5,1,1]
            [1,1,3,1,1]
        "#;

        let parsed_line = file_parser().parse(line);
        assert!(parsed_line.is_ok());

        let parsed_line = parsed_line.unwrap();
        assert!(!(parsed_line[0][0] < parsed_line[0][1]));
    }
}
