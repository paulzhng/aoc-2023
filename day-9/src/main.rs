#![feature(iter_map_windows)]

use aoc_2023_common::{init, PuzzlePart};

use std::str::FromStr;

use eyre::WrapErr;

const INPUT: &str = include_str!("input.txt");

fn main() -> eyre::Result<()> {
    let puzzle_part = init()?;
    let sum = calculate_result(INPUT, puzzle_part)?;
    println!("The result for puzzle part '{puzzle_part:?}' is: {sum}");
    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct History(Vec<i64>);

impl FromStr for History {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let history = s
            .split(' ')
            .map(i64::from_str)
            .map(|res| res.wrap_err("couldn't parse history number"))
            .collect::<eyre::Result<Vec<_>>>()?;

        Ok(Self(history))
    }
}

impl History {
    pub fn extrapolate_next_value(&self) -> i64 {
        self.calculate_diff()
            .iter()
            .rev()
            .map(|row| row.last().copied().unwrap_or_default())
            .fold(0, |acc, x| x + acc)
    }

    pub fn extrapolate_prev_value(&self) -> i64 {
        self.calculate_diff()
            .iter()
            .rev()
            .map(|row| row.first().copied().unwrap_or_default())
            .fold(0, |acc, x| x - acc)
    }

    fn calculate_diff(&self) -> Vec<Vec<i64>> {
        let mut row = self.0.clone();
        let mut rows = vec![row.clone()];

        while row.iter().any(|&x| x != 0) {
            row = row.iter().copied().map_windows(|[a, b]| *b - *a).collect();

            rows.push(row.clone());
        }

        rows
    }
}

fn calculate_result(input: &str, puzzle_part: PuzzlePart) -> eyre::Result<i64> {
    let histories = parse_input(input)?;

    let res = match puzzle_part {
        PuzzlePart::One => histories.iter().map(History::extrapolate_next_value).sum(),
        PuzzlePart::Two => histories.iter().map(History::extrapolate_prev_value).sum(),
    };

    Ok(res)
}

fn parse_input(input: &str) -> eyre::Result<Vec<History>> {
    input.lines().map(History::from_str).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_result_puzzle_part_1() -> eyre::Result<()> {
        let res = calculate_result(
            "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
            PuzzlePart::One,
        )?;
        assert_eq!(res, 114);

        Ok(())
    }

    #[test]
    fn test_calculate_result_puzzle_part_2() -> eyre::Result<()> {
        let res = calculate_result("10 13 16 21 30 45", PuzzlePart::Two)?;
        assert_eq!(res, 5);

        Ok(())
    }

    #[test]
    fn test_history_extrapolate_positive() -> eyre::Result<()> {
        let history = History::from_str("0 3 6 9 12 15")?;

        assert_eq!(history.extrapolate_next_value(), 18);

        Ok(())
    }

    #[test]
    fn test_history_extrapolate_negative() -> eyre::Result<()> {
        let history = History::from_str(
            "6 1 -4 -9 -14 -19 -24 -29 -34 -39 -44 -49 -54 -59 -64 -69 -74 -79 -84 -89 -94",
        )?;

        assert_eq!(history.extrapolate_next_value(), -99);

        Ok(())
    }

    #[test]
    fn test_history_parse() -> eyre::Result<()> {
        let history = History::from_str("0 3 6 9 12 15")?;

        assert_eq!(history, History(vec![0, 3, 6, 9, 12, 15]));

        Ok(())
    }
}
