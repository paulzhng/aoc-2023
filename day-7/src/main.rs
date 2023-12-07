use aoc_2023_common::{init, PuzzlePart};
use std::cmp::Ordering;

use eyre::{bail, eyre, ContextCompat};
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() -> eyre::Result<()> {
    let puzzle_part = init()?;
    let sum = calculate_result(INPUT, puzzle_part)?;
    println!("The result for puzzle part '{puzzle_part:?}' is: {sum}");

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

impl Hand {
    pub fn hand_type(&self) -> HandType {
        let jokers = self
            .cards
            .iter()
            .filter(|&&card| card == Card::Joker)
            .count();

        let mut amounts = self
            .cards
            .into_iter()
            .filter(|&card| card != Card::Joker)
            .counts_by(|c| c)
            .into_values()
            .sorted()
            .rev()
            .collect::<Vec<_>>();

        if amounts.is_empty() {
            amounts.push(jokers);
        } else {
            amounts[0] += jokers;
        }

        match amounts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!("invalid amounts: {amounts:?}"),
        }
    }
}

fn calculate_result(input: &str, puzzle_part: PuzzlePart) -> eyre::Result<u64> {
    let hands = parse_input(input, puzzle_part)?;

    let res = hands
        .into_iter()
        .sorted()
        .enumerate()
        .map(|(idx, hand)| (idx as u64 + 1, hand.bid))
        .fold(0, |acc, (rank, bid)| acc + rank * bid);

    Ok(res)
}

fn parse_input(input: &str, puzzle_part: PuzzlePart) -> eyre::Result<Vec<Hand>> {
    input
        .lines()
        .map(|line| Hand::try_from((line, puzzle_part)))
        .collect()
}

impl TryFrom<(&str, PuzzlePart)> for Hand {
    type Error = eyre::Error;

    fn try_from((s, puzzle_part): (&str, PuzzlePart)) -> Result<Self, Self::Error> {
        let (cards_str, bid_str) = s.split_once(' ').wrap_err("invalid line format")?;

        let cards: [_; 5] = cards_str
            .chars()
            .map(|c| Card::try_from((c, puzzle_part)))
            .collect::<eyre::Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| eyre!("too many cards"))?;
        let bid: u64 = bid_str.parse()?;

        Ok(Self { cards, bid })
    }
}

impl TryFrom<(char, PuzzlePart)> for Card {
    type Error = eyre::Error;

    fn try_from((c, puzzle_part): (char, PuzzlePart)) -> Result<Self, Self::Error> {
        let card = match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' if puzzle_part == PuzzlePart::One => Self::Jack,
            'J' if puzzle_part == PuzzlePart::Two => Self::Joker,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => bail!("unknown card: {c}"),
        };
        Ok(card)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_calculate_result_puzzle_part_1() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE, PuzzlePart::One)?;
        assert_eq!(res, 6440);

        Ok(())
    }

    #[test]
    fn test_calculate_result_puzzle_part_2() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE, PuzzlePart::Two)?;
        assert_eq!(res, 5905);

        Ok(())
    }

    #[test]
    fn test_part_1_hand_parse() -> eyre::Result<()> {
        let res = Hand::try_from(("32T3J 765", PuzzlePart::One))?;
        assert_eq!(
            res,
            Hand {
                cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::Jack],
                bid: 765,
            }
        );

        Ok(())
    }

    #[test]
    fn test_part_2_hand_parse() -> eyre::Result<()> {
        let res = Hand::try_from(("32T3J 765", PuzzlePart::Two))?;
        assert_eq!(
            res,
            Hand {
                cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::Joker],
                bid: 765,
            }
        );

        Ok(())
    }

    #[test]
    fn test_part_1_hand_type() -> eyre::Result<()> {
        for (hand, expected_hand_type) in [
            ("AAAAA 1", HandType::FiveOfAKind),
            ("AA8AA 1", HandType::FourOfAKind),
            ("23332 1", HandType::FullHouse),
            ("TTT98 1", HandType::ThreeOfAKind),
            ("23432 1", HandType::TwoPair),
            ("A23A4 1", HandType::OnePair),
            ("23456 1", HandType::HighCard),
        ] {
            let hand = Hand::try_from((hand, PuzzlePart::One))?;
            assert_eq!(hand.hand_type(), expected_hand_type);
        }

        Ok(())
    }

    #[test]
    fn test_part_2_hand_type() -> eyre::Result<()> {
        for (hand, expected_hand_type) in [
            ("32T3K 1", HandType::OnePair),
            ("KK677 1", HandType::TwoPair),
            ("T55J5 1", HandType::FourOfAKind),
            ("KTJJT 1", HandType::FourOfAKind),
            ("QQQJA 1", HandType::FourOfAKind),
        ] {
            let hand = Hand::try_from((hand, PuzzlePart::Two))?;
            assert_eq!(hand.hand_type(), expected_hand_type, "hand = {hand:?}");
        }

        Ok(())
    }

    #[test]
    fn test_hand_ord_neq_kind() -> eyre::Result<()> {
        let greater_hand = Hand::try_from(("AAAAA 1", PuzzlePart::One))?;
        let smaller_hand = Hand::try_from(("AA8AA 1", PuzzlePart::One))?;

        assert!(
            greater_hand > smaller_hand,
            "greater_hand_type = {:?}, smaller_hand_type = {:?}",
            greater_hand.hand_type(),
            smaller_hand.hand_type()
        );

        Ok(())
    }

    #[test]
    fn test_hand_ord_eq_kind() -> eyre::Result<()> {
        let greater_hand = Hand::try_from(("33332 1", PuzzlePart::One))?;
        let smaller_hand = Hand::try_from(("2AAAA 1", PuzzlePart::One))?;

        assert!(
            greater_hand > smaller_hand,
            "greater_hand_type = {:?}, smaller_hand_type = {:?}",
            greater_hand.hand_type(),
            smaller_hand.hand_type()
        );

        Ok(())
    }
}
