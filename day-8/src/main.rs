#![feature(slice_split_once)]

use aoc_2023_common::{init, PuzzlePart};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use eyre::{bail, ContextCompat, WrapErr};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use num::integer::lcm;
use once_cell::sync::Lazy;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() -> eyre::Result<()> {
    let puzzle_part = init()?;
    let sum = calculate_result(INPUT, puzzle_part)?;
    println!("The result for puzzle part '{puzzle_part:?}' is: {sum}");
    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Instruction {
    Left,
    Right,
}

type ElementId = [char; 3];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Element {
    id: ElementId,
    left: ElementId,
    right: ElementId,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Network {
    elements: HashMap<ElementId, Element>,
}

impl TryFrom<char> for Instruction {
    type Error = eyre::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => bail!("unknown instruction: {c}"),
        }
    }
}

impl FromStr for Network {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements = s
            .lines()
            .map(Element::from_str)
            .map(|res| res.wrap_err("could not parse element"))
            .map_ok(|element| (element.id, element))
            .collect::<eyre::Result<HashMap<ElementId, Element>>>()?;

        Ok(Self { elements })
    }
}

impl FromStr for Element {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static ELEMENT_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?<id>[A-Z0-9]{3}) = \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)")
                .unwrap()
        });

        let captures = ELEMENT_REGEX
            .captures(s)
            .wrap_err("invalid element format")?;
        let parse_element_id = |group_name: &str| -> eyre::Result<ElementId> {
            captures
                .name(group_name)
                .wrap_err("group name not found")?
                .as_str()
                .chars()
                .collect::<Vec<char>>()
                .as_slice()
                .try_into()
                .wrap_err("element ID too long")
        };

        let id = parse_element_id("id")?;
        let left = parse_element_id("left")?;
        let right = parse_element_id("right")?;

        Ok(Self { id, left, right })
    }
}

impl Network {
    pub fn take_step(&self, cur: ElementId, instruction: Instruction) -> Option<ElementId> {
        self.elements.get(&cur).map(|element| match instruction {
            Instruction::Left => element.left,
            Instruction::Right => element.right,
        })
    }

    pub fn steps(
        &self,
        start: ElementId,
        target: &HashSet<ElementId>,
        instructions: &[Instruction],
    ) -> usize {
        let (steps, _) = instructions
            .iter()
            .cycle()
            .copied()
            .fold_while((0usize, start), |(steps, cur), inst| {
                match self.take_step(cur, inst) {
                    Some(next) if target.contains(&next) => Done((steps + 1, next)),
                    Some(next) => Continue((steps + 1, next)),
                    None => panic!("Unknown node: {cur:?}"),
                }
            })
            .into_inner();

        steps
    }
}

fn calculate_result(input: &str, puzzle_part: PuzzlePart) -> eyre::Result<usize> {
    let (instructions, network) = parse_input(input)?;

    let res = match puzzle_part {
        PuzzlePart::One => {
            const START: ElementId = ['A', 'A', 'A'];
            const END: ElementId = ['Z', 'Z', 'Z'];

            network.steps(START, &HashSet::from([END]), &instructions)
        }
        PuzzlePart::Two => {
            let start_positions = network
                .elements
                .values()
                .map(|element| element.id)
                .filter(|element_id| element_id.ends_with(&['A']))
                .collect::<Vec<_>>();
            let end_positions = network
                .elements
                .values()
                .map(|element| element.id)
                .filter(|element_id| element_id.ends_with(&['Z']))
                .collect::<HashSet<_>>();

            start_positions.iter().fold(1, |acc, &x| {
                lcm(acc, network.steps(x, &end_positions, &instructions))
            })
        }
    };

    Ok(res)
}

fn parse_input(input: &str) -> eyre::Result<(Vec<Instruction>, Network)> {
    let (instructions_line, networks) = input.split_once("\n\n").wrap_err("invalid format")?;

    let instructions = instructions_line
        .chars()
        .map(Instruction::try_from)
        .collect::<eyre::Result<Vec<_>>>()?;

    let network = Network::from_str(networks)?;

    Ok((instructions, network))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_DIRECT_PART_1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_CIRCULAR_PART_1: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const EXAMPLE_PART_2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_calculate_result_puzzle_part_1_direct() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE_DIRECT_PART_1, PuzzlePart::One)?;
        assert_eq!(res, 2);

        Ok(())
    }

    #[test]
    fn test_calculate_result_puzzle_part_1_circular() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE_CIRCULAR_PART_1, PuzzlePart::One)?;
        assert_eq!(res, 6);

        Ok(())
    }

    #[test]
    fn test_calculate_result_puzzle_part_2() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE_PART_2, PuzzlePart::Two)?;
        assert_eq!(res, 6);

        Ok(())
    }

    #[test]
    fn test_input_parse() -> eyre::Result<()> {
        let (instructions, network) = parse_input(EXAMPLE_CIRCULAR_PART_1)?;

        assert_eq!(
            instructions,
            vec![Instruction::Left, Instruction::Left, Instruction::Right]
        );

        let aaa = ['A', 'A', 'A'];
        let bbb = ['B', 'B', 'B'];
        let zzz = ['Z', 'Z', 'Z'];

        assert_eq!(
            network,
            Network {
                elements: HashMap::from([
                    (
                        aaa,
                        Element {
                            id: aaa,
                            left: bbb,
                            right: bbb,
                        }
                    ),
                    (
                        bbb,
                        Element {
                            id: bbb,
                            left: aaa,
                            right: zzz,
                        }
                    ),
                    (
                        zzz,
                        Element {
                            id: zzz,
                            left: zzz,
                            right: zzz,
                        }
                    )
                ])
            }
        );

        Ok(())
    }
}
