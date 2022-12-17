use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use chumsky::prelude::*;
use itertools::Itertools;
use petgraph::{algo::floyd_warshall, prelude::*};
use rayon::prelude::*;

pub(crate) fn solve(path: PathBuf) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let parsed_file = file_parser()
        .parse(file)
        .map_err(|err| anyhow::anyhow!("An error occurred while parsing the file: {err:?}"))?;

    let mut graph: DiGraph<(), ()> = DiGraph::new();

    let successors: HashMap<&str, &Vec<String>> =
        parsed_file.iter().fold(HashMap::new(), |mut acc, valve| {
            acc.insert(&valve.id, &valve.connected_to);

            acc
        });

    let nodes: HashMap<&str, NodeIndex> =
        parsed_file.iter().fold(HashMap::new(), |mut acc, valve| {
            acc.insert(&valve.id, graph.add_node(()));

            acc
        });
    log::trace!("nodes: {nodes:?}");

    let edges: Vec<(NodeIndex, NodeIndex)> = successors
        .iter()
        .flat_map(|(&id, &next_node_ids)| {
            let Some(current_node) = nodes.get(id) else { return vec![]; };

            next_node_ids
                .into_iter()
                .filter_map(|next_node_id| {
                    let Some(next_node) = nodes.get(&**next_node_id) else {return None};

                    Some((*current_node, *next_node))
                })
                .collect::<Vec<(NodeIndex, NodeIndex)>>()
        })
        .collect();
    log::trace!("edges: {edges:?}");

    graph.extend_with_edges(edges);

    let Ok(shortest_paths) = floyd_warshall(&graph, |_| 1) else { anyhow::bail!("Could not calculate floyd_warshall"); };

    // -- Task 1 --
    let paths = get_rated_paths(
        parsed_file.iter().filter(|valve| valve.rate > 0).collect(),
        vec![],
        &shortest_paths,
        &nodes,
        30,
        "AA",
    );
    log::trace!("paths: {paths:?}");

    let Some((high_score, path)) = find_highest_rated_path(&paths) else { anyhow::bail!("Could not find the highest rated path") };
    log::info!("task 1: high_score: {high_score} - path: {path}");

    // -- Task 2 --

    // Unfortunately, this doesn't work correctly
    let result = walk_the_graph_together(
        parsed_file.iter().filter(|valve| valve.rate > 0).collect(),
        &shortest_paths,
        &nodes,
        26,
        26,
        "AA",
        "AA",
    );
    log::info!("task 2 result: {result}");

    Ok(())
}

fn walk_the_graph_together(
    valves: Vec<&Valve>,
    shortest_paths: &HashMap<(NodeIndex, NodeIndex), i32>,
    nodes: &HashMap<&str, NodeIndex>,
    me_time: i32,
    elephant_time: i32,
    my_location: &str,
    elephant_location: &str,
) -> i32 {
    if valves.len() == 0 {
        return 0;
    }

    // let scores = get_rated_valves(valves.iter(), &shortest_paths, &nodes, me_time, my_location);
    let scores_elephant = get_rated_valves(
        valves.iter(),
        &shortest_paths,
        &nodes,
        elephant_time,
        elephant_location,
    );

    scores_elephant
        .into_par_iter()
        .flat_map(|elephant_score| {
            get_rated_valves(
                valves
                    .iter()
                    .filter(|valve| valve.id != elephant_score.valve_id)
                    .map(|a| *a)
                    .collect::<Vec<_>>()
                    .iter(),
                shortest_paths,
                nodes,
                me_time,
                my_location,
            )
            .into_iter()
            .map(|me_score| (elephant_score.clone(), me_score))
            .collect::<Vec<_>>()
        })
        .map(|(e_score, my_score)| {
            my_score.rating
                + e_score.rating
                + walk_the_graph_together(
                    valves
                        .iter()
                        .filter(|valve| {
                            valve.id != my_score.valve_id && valve.id != e_score.valve_id
                        })
                        .map(|a| *a)
                        .collect::<Vec<_>>(),
                    shortest_paths,
                    nodes,
                    my_score.time_left,
                    e_score.time_left,
                    &my_score.valve_id,
                    &e_score.valve_id,
                )
        })
        .max()
        .unwrap_or(0)

    // scores
    //     .into_par_iter()
    //     .flat_map(|me_score| {
    //         get_rated_valves(
    //             valves
    //                 .iter()
    //                 .filter(|valve| valve.id != me_score.valve_id)
    //                 .map(|a| *a)
    //                 .collect::<Vec<_>>()
    //                 .iter(),
    //             shortest_paths,
    //             nodes,
    //             elephant_time,
    //             elephant_location,
    //         )
    //         .into_iter()
    //         .map(|elephant_score| (me_score.clone(), elephant_score))
    //         .collect::<Vec<_>>()
    //     })
    //     .map(|(my_score, e_score)| {
    //         my_score.rating
    //             + e_score.rating
    //             + walk_the_graph_together(
    //                 valves
    //                     .iter()
    //                     .filter(|valve| {
    //                         valve.id != my_score.valve_id && valve.id != e_score.valve_id
    //                     })
    //                     .map(|a| *a)
    //                     .collect::<Vec<_>>(),
    //                 shortest_paths,
    //                 nodes,
    //                 my_score.time_left,
    //                 e_score.time_left,
    //                 &my_score.valve_id,
    //                 &e_score.valve_id,
    //             )
    //     })
    //     .max()
    //     .unwrap_or(0)
}

#[derive(Debug)]
struct Path {
    rating: i32,
    valve_id: String,
    paths: Vec<Path>,
}

fn find_highest_rated_path(paths: &Vec<Path>) -> Option<(i32, String)> {
    if paths.len() == 0 {
        return None;
    }

    paths
        .iter()
        .map(|path| {
            let next_highest_path =
                find_highest_rated_path(&path.paths).unwrap_or((0, "".to_owned()));

            (
                path.rating + next_highest_path.0,
                format!("{}{}", path.valve_id, next_highest_path.1),
            )
        })
        .max_by_key(|path| path.0)
}

fn get_rated_paths(
    valves: Vec<&Valve>,
    visited_valve_ids: Vec<String>,
    shortest_paths: &HashMap<(NodeIndex, NodeIndex), i32>,
    nodes: &HashMap<&str, NodeIndex>,
    time: i32,
    current_node: &str,
) -> Vec<Path> {
    let scores = get_rated_valves(
        valves
            .iter()
            .filter(|valve| !visited_valve_ids.contains(&valve.id.clone()))
            .map(|a| *a)
            .collect::<Vec<_>>()
            .iter(),
        &shortest_paths,
        &nodes,
        time,
        current_node,
    );

    if scores.len() == 0 {
        vec![]
    } else {
        scores
            .into_iter()
            .map(
                |ValveRating {
                     rating,
                     time_left,
                     valve_id,
                 }| Path {
                    rating,
                    valve_id: valve_id.clone(),
                    paths: get_rated_paths(
                        valves.iter().map(|a| *a).collect(),
                        {
                            let mut visited_valves = visited_valve_ids.clone();
                            visited_valves.push(valve_id.clone());

                            visited_valves
                        },
                        &shortest_paths,
                        &nodes,
                        time_left,
                        &valve_id,
                    ),
                },
            )
            .collect()
    }
}

#[derive(Clone, Debug)]
struct ValveRating {
    rating: i32,
    time_left: i32,
    valve_id: String,
}

fn get_rated_valves(
    iter: std::slice::Iter<&Valve>,
    shortest_paths: &HashMap<(NodeIndex, NodeIndex), i32>,
    nodes: &HashMap<&str, NodeIndex>,
    time: i32,
    current_node: &str,
) -> Vec<ValveRating> {
    iter.filter_map(|&valve| {
        let time_left = time
            - shortest_paths
                .get(&(
                    *(nodes.get(current_node).unwrap()),
                    *(nodes.get(&*valve.id).unwrap()),
                ))
                .unwrap()
            - 1;

        if time_left < 0 {
            return None;
        }

        let rating = (time_left) * valve.rate;

        Some(ValveRating {
            rating,
            time_left,
            valve_id: valve.id.clone(),
        })
    })
    .sorted_by(|a, b| b.rating.cmp(&a.rating))
    .collect()
}

// --- Parser ---
#[derive(Clone, Debug, PartialEq)]
struct Valve {
    id: String,
    rate: i32,
    connected_to: Vec<String>,
}

fn file_parser() -> impl Parser<char, Vec<Valve>, Error = Simple<char>> {
    line_parser().repeated()
}

fn line_parser() -> impl Parser<char, Valve, Error = Simple<char>> {
    just("Valve")
        .padded()
        .ignore_then(text::ident())
        .padded()
        .then_ignore(just("has flow rate="))
        .then(text::int(10))
        .try_map(|(id, rate): (String, String), span| {
            let rate = rate
                .parse::<i32>()
                .map_err(|e| Simple::custom(span, format!("{}", e)))?;

            Ok((id, rate))
        })
        .then_ignore(just(";").padded())
        .then_ignore(
            just("tunnel leads to valve")
                .or(just("tunnels lead to valves"))
                .padded(),
        )
        .then(
            text::ident::<_, Simple<char>>()
                .padded()
                .separated_by(just(',')),
        )
        .map(|((id, rate), valves)| Valve {
            id,
            rate,
            connected_to: valves,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = include_str!("../tasks/day16_dev.txt");

    #[test]
    fn test_line_parser() {
        let line = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";

        let parsed_line = line_parser().parse(line);
        assert!(parsed_line.is_ok());

        assert_eq!(
            parsed_line.unwrap(),
            Valve {
                id: "AA".to_owned(),
                rate: 0,
                connected_to: vec!["DD".to_owned(), "II".to_owned(), "BB".to_owned()],
            }
        );
    }

    #[test]
    fn test_line_parser_2() {
        let line = "Valve JJ has flow rate=21; tunnel leads to valve II";

        let parsed_line = line_parser().parse(line);
        assert!(parsed_line.is_ok());

        assert_eq!(
            parsed_line.unwrap(),
            Valve {
                id: "JJ".to_owned(),
                rate: 21,
                connected_to: vec!["II".to_owned()]
            }
        );
    }

    #[test]
    fn test_file_parser() {
        let parsed_file = file_parser().parse(TEST_FILE);
        assert!(parsed_file.is_ok());

        assert_eq!(
            parsed_file.unwrap(),
            vec![
                Valve {
                    id: "AA".to_owned(),
                    rate: 0,
                    connected_to: vec!["DD".to_owned(), "II".to_owned(), "BB".to_owned()],
                },
                Valve {
                    id: "BB".to_owned(),
                    rate: 13,
                    connected_to: vec!["CC".to_owned(), "AA".to_owned()],
                },
                Valve {
                    id: "CC".to_owned(),
                    rate: 2,
                    connected_to: vec!["DD".to_owned(), "BB".to_owned()],
                },
                Valve {
                    id: "DD".to_owned(),
                    rate: 20,
                    connected_to: vec!["CC".to_owned(), "AA".to_owned(), "EE".to_owned()],
                },
                Valve {
                    id: "EE".to_owned(),
                    rate: 3,
                    connected_to: vec!["FF".to_owned(), "DD".to_owned()],
                },
                Valve {
                    id: "FF".to_owned(),
                    rate: 0,
                    connected_to: vec!["EE".to_owned(), "GG".to_owned()],
                },
                Valve {
                    id: "GG".to_owned(),
                    rate: 0,
                    connected_to: vec!["FF".to_owned(), "HH".to_owned()],
                },
                Valve {
                    id: "HH".to_owned(),
                    rate: 22,
                    connected_to: vec!["GG".to_owned()],
                },
                Valve {
                    id: "II".to_owned(),
                    rate: 0,
                    connected_to: vec!["AA".to_owned(), "JJ".to_owned()],
                },
                Valve {
                    id: "JJ".to_owned(),
                    rate: 21,
                    connected_to: vec!["II".to_owned()],
                },
            ]
        );
    }
}
