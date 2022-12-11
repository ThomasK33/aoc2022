use std::path::PathBuf;

use anyhow::Result;
use chumsky::prelude::*;

pub(crate) fn solve(path: PathBuf, rounds: u32, decreasing_worry_levels: bool) -> Result<()> {
    let file = std::fs::read_to_string(path)?;

    let monkeys = file_parser()
        .parse(file)
        .map_err(|err| anyhow::anyhow!("Could not parse file, an error occurred: {err:?}"))?;

    let mut monkey_inspection_counter: Vec<usize> = vec![0; monkeys.len()];

    let monkey_modulos: Vec<u32> = monkeys.iter().map(|monkey| monkey.modulo).collect();

    let mut monkeys: Vec<Monkey> = monkeys
        .into_iter()
        .map(|monkey| Monkey {
            reduced_modulo_items: monkey
                .items
                .iter()
                .map(|item| {
                    monkey_modulos
                        .clone()
                        .iter()
                        .map(|modulo| (*modulo, *item % *modulo))
                        .collect::<Vec<(u32, u32)>>()
                })
                .collect(),
            items: monkey.items,
            operation: monkey.operation,
            next_monkey: monkey.next_monkey,
            modulo: monkey.modulo,
        })
        .collect();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            if decreasing_worry_levels {
                let mut items = monkeys[i].items.clone();
                while let Some(item) = items.pop() {
                    let score = (monkeys[i].operation)(item);
                    let score = score / 3;

                    let next_monkey_id = (monkeys[i].next_monkey)(score);
                    monkeys[next_monkey_id].items.push(score);

                    monkey_inspection_counter[i] += 1;
                }
                monkeys[i].items.clear();
            } else {
                let mut items = monkeys[i].reduced_modulo_items.clone();
                while let Some(item) = items.pop() {
                    let item: Vec<(u32, u32)> = item
                        .into_iter()
                        .map(|(modulo, item)| (modulo, (monkeys[i].operation)(item) % modulo))
                        .collect();

                    if let Some((_, current_monkey_score)) =
                        item.iter().find(|(modulo, _)| *modulo == monkeys[i].modulo)
                    {
                        let next_monkey_id = (monkeys[i].next_monkey)(*current_monkey_score);
                        monkeys[next_monkey_id].reduced_modulo_items.push(item);
                    }

                    monkey_inspection_counter[i] += 1;
                }
                monkeys[i].reduced_modulo_items.clear();
            }
        }
    }

    monkey_inspection_counter.sort();
    monkey_inspection_counter.reverse();
    log::debug!("monkey_inspection_counter: {monkey_inspection_counter:?}");

    log::info!(
        "monkey business: {}",
        monkey_inspection_counter[0] * monkey_inspection_counter[1]
    );

    Ok(())
}

// --- Parser ---

fn file_parser() -> impl Parser<char, Vec<Monkey>, Error = Simple<char>> {
    monkey_parser().padded().repeated()
}

struct Monkey {
    items: Vec<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    next_monkey: Box<dyn Fn(u32) -> usize>,

    modulo: u32,
    reduced_modulo_items: Vec<Vec<(u32, u32)>>,
}

fn monkey_parser() -> impl Parser<char, Monkey, Error = Simple<char>> {
    enum Operation {
        Add,
        Multiply,
    }

    enum OperationValue {
        Number(u32),
        Old,
    }

    just("Monkey")
        .padded()
        .ignore_then(take_until(just(':')))
        .ignore_then(just("Starting items:").padded())
        // -- Starting items --
        .ignore_then(take_until(text::newline()).map(|(preceding, _)| {
            preceding
                .into_iter()
                .collect::<String>()
                .split(',')
                .into_iter()
                .map(|item| item.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        }))
        // -- Operation --
        .then(
            just("Operation: new = old")
                .padded()
                .ignore_then(
                    just('+')
                        .or(just('*'))
                        .padded()
                        .then(take_until(text::newline()))
                        .map(|(op, (preceding, _))| {
                            (
                                match op {
                                    '+' => Operation::Add,
                                    '*' => Operation::Multiply,
                                    _ => todo!("Unknown character found"),
                                },
                                {
                                    let value = preceding.into_iter().collect::<String>();

                                    if value == "old" {
                                        OperationValue::Old
                                    } else {
                                        OperationValue::Number(value.parse::<u32>().unwrap())
                                    }
                                },
                            )
                        }),
                )
                .map(|(op, value)| {
                    Box::new(move |input: u32| {
                        let value = match value {
                            OperationValue::Number(value) => value,
                            OperationValue::Old => input,
                        };

                        match op {
                            Operation::Add => input + value,
                            Operation::Multiply => input * value,
                        }
                    })
                }),
        )
        // -- Test --
        .then(
            just("Test: divisible by")
                .padded()
                .ignore_then(text::int(10))
                .from_str::<u32>()
                .unwrapped()
                .padded()
                .then(
                    just("If true: throw to monkey")
                        .padded()
                        .ignore_then(text::int(10))
                        .from_str::<usize>()
                        .unwrapped()
                        .padded(),
                )
                .then(
                    just("If false: throw to monkey")
                        .padded()
                        .ignore_then(text::int(10))
                        .from_str::<usize>()
                        .unwrapped()
                        .padded(),
                )
                .map(|((modulo, true_condition), false_condition)| {
                    (
                        modulo,
                        Box::new(move |value: u32| {
                            if value % modulo == 0 {
                                true_condition
                            } else {
                                false_condition
                            }
                        }),
                    )
                }),
        )
        // -- Combine into monkey --
        .map(|((items, operation), (modulo, next_monkey))| {
            let reduced_modulo_items =
                vec![items.iter().map(|item| (modulo, item % modulo)).collect()];

            Monkey {
                items,
                operation,
                next_monkey,
                modulo,
                reduced_modulo_items,
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = include_str!("../tasks/day11_dev.txt");

    #[test]
    fn test_monkey_parser() {
        let raw_monkey = r#"
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3

        "#;

        let monkey = monkey_parser().parse(raw_monkey);
        assert!(monkey.is_ok());

        let monkey = monkey.unwrap();
        assert_eq!(monkey.items, vec![79, 98]);
        assert_eq!((&monkey.operation)(10), 190);
        assert_eq!((&monkey.next_monkey)(46), 2);
        assert_eq!((&monkey.next_monkey)(45), 3);
    }

    #[test]
    fn test_file_parser() {
        let monkeys = file_parser().parse(TEST_FILE);
        assert!(monkeys.is_ok());

        let monkeys = monkeys.unwrap();

        assert_eq!(monkeys[0].items, vec![79, 98]);
        assert_eq!(monkeys[1].items, vec![54, 65, 75, 74]);
        assert_eq!(monkeys[2].items, vec![79, 60, 97]);
        assert_eq!(monkeys[3].items, vec![74]);
    }
}
