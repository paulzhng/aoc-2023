use aoc_2023_common::{init, PuzzlePart};

use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;

use eyre::{ContextCompat, WrapErr};
use once_cell::sync::Lazy;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() -> eyre::Result<()> {
    let puzzle_part = init()?;
    let sum = calculate_result(INPUT, puzzle_part)?;
    println!("The result for puzzle part '{puzzle_part:?}' is: {sum}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Card {
    id: usize,
    winning: HashSet<u32>,
    guessed: HashSet<u32>,
}

impl FromStr for Card {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static CARD_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"Card +(?<card_id>\d+): (?<winning>.+) \| (?<guessed>.+)").unwrap()
        });

        let card_captures = CARD_REGEX
            .captures(s)
            .wrap_err("card pattern didn't match")?;

        let card_id_match = card_captures
            .name("card_id")
            .wrap_err("invalid capture group name")?;
        let winning_numbers_match = card_captures
            .name("winning")
            .wrap_err("invalid capture group name")?;
        let guessed_numbers_match = card_captures
            .name("guessed")
            .wrap_err("invalid capture group name")?;

        let card_id = usize::from_str(card_id_match.as_str())?;
        let winning_numbers = winning_numbers_match
            .as_str()
            .split(' ')
            .filter(|str| !str.is_empty())
            .map(u32::from_str)
            .map(|res| res.wrap_err("invalid number"))
            .collect::<eyre::Result<HashSet<_>>>()?;
        let guessed_numbers = guessed_numbers_match
            .as_str()
            .split(' ')
            .filter(|str| !str.is_empty())
            .map(u32::from_str)
            .map(|res| res.wrap_err("invalid number"))
            .collect::<eyre::Result<HashSet<_>>>()?;

        Ok(Self {
            id: card_id,
            winning: winning_numbers,
            guessed: guessed_numbers,
        })
    }
}

fn calculate_result(input: &str, puzzle_part: PuzzlePart) -> eyre::Result<u32> {
    let cards = parse_input(input)?;

    let res = match puzzle_part {
        PuzzlePart::One => cards
            .iter()
            .map(|card| {
                card.guessed
                    .iter()
                    .copied()
                    .filter(|guess| card.winning.contains(guess))
                    .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
            })
            .sum(),
        PuzzlePart::Two => {
            let mut card_amount = cards
                .iter()
                .map(|card| (card.id, 1usize))
                .collect::<BTreeMap<_, _>>();
            for card in &cards {
                let matches = card
                    .guessed
                    .iter()
                    .copied()
                    .filter(|guess| card.winning.contains(guess))
                    .count();

                let amount = card_amount[&card.id];
                for i in 1..=matches {
                    if let Some(old_amount) = card_amount.get_mut(&(card.id + i)) {
                        *old_amount += amount;
                    }
                }
            }

            card_amount.values().map(|count| *count as u32).sum()
        }
    };

    Ok(res)
}

fn parse_input(input: &str) -> eyre::Result<Vec<Card>> {
    input.lines().map(Card::from_str).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_calculate_result_puzzle_part_1() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE, PuzzlePart::One)?;
        assert_eq!(res, 13);

        Ok(())
    }

    #[test]
    fn test_calculate_result_puzzle_part_2() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE, PuzzlePart::Two)?;
        assert_eq!(res, 30);

        Ok(())
    }

    #[test]
    fn test_card_parse() -> eyre::Result<()> {
        let res = Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")?;
        assert_eq!(
            res,
            Card {
                id: 1,
                winning: HashSet::from([41, 48, 83, 86, 17]),
                guessed: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
            }
        );

        Ok(())
    }
}
