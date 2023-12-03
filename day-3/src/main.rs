use aoc_2023_common::{init, PuzzlePart};
use std::collections::HashMap;
use std::str::FromStr;

use std::{iter, vec};

const INPUT: &str = include_str!("input.txt");

fn main() -> eyre::Result<()> {
    let puzzle_part = init()?;
    let sum = calculate_result(INPUT, puzzle_part)?;
    println!("The result for puzzle part '{puzzle_part:?}' is: {sum}");

    Ok(())
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Schematic {
    objects: Vec<Object>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Object {
    Number(Number),
    Symbol(Symbol),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Number {
    x: usize,
    y: usize,
    width: usize,
    num: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Symbol {
    x: usize,
    y: usize,
    symbol: char,
}

impl Schematic {
    pub fn numbers(&self) -> impl Iterator<Item = Number> + '_ {
        self.objects.iter().filter_map(|obj| match obj {
            Object::Number(number) => Some(*number),
            _ => None,
        })
    }

    pub fn symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.objects.iter().filter_map(|obj| match obj {
            Object::Symbol(symbol) => Some(*symbol),
            _ => None,
        })
    }

    pub fn adjacent_symbols(&self, num: Number) -> impl Iterator<Item = Symbol> + '_ {
        let Number { x, y, width, .. } = num;

        let in_adjacent_row = move |sym: &Symbol| (sym.y as isize - y as isize).abs() <= 1;
        let in_adjacent_column =
            move |sym: &Symbol| sym.x as isize >= (x as isize) - 1 && sym.x <= (x + width);

        self.symbols()
            .filter(in_adjacent_row)
            .filter(in_adjacent_column)
    }
}

impl FromStr for Schematic {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        enum ParseState {
            Number { num: u32, width: usize },
            Other,
        }

        let objects = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                let (parsed, _) = line
                    .chars()
                    .chain(iter::once('.')) // chain a '.' for flushing the last parse state
                    .enumerate()
                    .fold(
                        (vec![], ParseState::Other),
                        |(mut parsed, state), (x, c)| {
                            let next_state = if c.is_ascii_digit() {
                                let digit = c.to_digit(10).expect("must be digit");

                                match state {
                                    ParseState::Number { num, width } => ParseState::Number {
                                        num: num * 10 + digit,
                                        width: width + 1,
                                    },
                                    ParseState::Other => ParseState::Number {
                                        num: digit,
                                        width: 1,
                                    },
                                }
                            } else {
                                if let ParseState::Number { num, width } = state {
                                    parsed.push(Object::Number(Number {
                                        x: x - width,
                                        y,
                                        width,
                                        num,
                                    }));
                                }

                                if c != '.' {
                                    parsed.push(Object::Symbol(Symbol { x, y, symbol: c }));
                                }

                                ParseState::Other
                            };

                            (parsed, next_state)
                        },
                    );

                parsed
            })
            .collect();

        Ok(Schematic { objects })
    }
}

fn calculate_result(input: &str, puzzle_part: PuzzlePart) -> eyre::Result<u32> {
    let schematic = Schematic::from_str(input)?;

    let res = match puzzle_part {
        PuzzlePart::One => schematic
            .numbers()
            .filter(|num| schematic.adjacent_symbols(*num).next().is_some())
            .map(|num| num.num)
            .sum(),
        PuzzlePart::Two => {
            let symbol_to_adjacent_numbers = schematic
                .numbers()
                .flat_map(|num| {
                    schematic
                        .adjacent_symbols(num)
                        .filter(|sym| sym.symbol == '*')
                        .map(move |sym| (sym, num))
                })
                .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (sym, num)| {
                    acc.entry(sym).or_default().push(num);
                    acc
                });

            symbol_to_adjacent_numbers
                .into_iter()
                .filter_map(|(_, nums)| match nums.as_slice() {
                    &[num1, num2] => Some(num1.num * num2.num),
                    _ => None,
                })
                .sum()
        }
    };

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_calculate_result_puzzle_part_1() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE, PuzzlePart::One)?;
        assert_eq!(res, 4361);

        Ok(())
    }

    #[test]
    fn test_calculate_result_puzzle_part_2() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE, PuzzlePart::Two)?;
        assert_eq!(res, 467835);

        Ok(())
    }

    #[test]
    fn test_schematic_parse() -> eyre::Result<()> {
        let res = Schematic::from_str(
            "\
467..114..
...*.....1
",
        )?;
        assert_eq!(
            res,
            Schematic {
                objects: vec![
                    Object::Number(Number {
                        x: 0,
                        y: 0,
                        width: 3,
                        num: 467,
                    }),
                    Object::Number(Number {
                        x: 5,
                        y: 0,
                        width: 3,
                        num: 114
                    }),
                    Object::Symbol(Symbol {
                        x: 3,
                        y: 1,
                        symbol: '*',
                    }),
                    Object::Number(Number {
                        x: 9,
                        y: 1,
                        width: 1,
                        num: 1
                    })
                ]
            }
        );

        Ok(())
    }

    #[test]
    fn schematic_adjacent_symbols() -> eyre::Result<()> {
        let schematic = Schematic::from_str(
            "\
*...../
.467+./
....-./
",
        )?;
        let num = schematic.numbers().next().unwrap();
        let res = schematic.adjacent_symbols(num).collect::<Vec<_>>();
        assert_eq!(
            res,
            vec![
                Symbol {
                    x: 0,
                    y: 0,
                    symbol: '*',
                },
                Symbol {
                    x: 4,
                    y: 1,
                    symbol: '+',
                },
                Symbol {
                    x: 4,
                    y: 2,
                    symbol: '-',
                }
            ]
        );

        Ok(())
    }
}
