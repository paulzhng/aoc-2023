use aoc_2023_common::{init, PuzzlePart};

use std::str::FromStr;

use eyre::{ContextCompat, WrapErr};

const INPUT: &str = include_str!("input.txt");

fn main() -> eyre::Result<()> {
    let puzzle_part = init()?;
    let sum = calculate_result(INPUT, puzzle_part)?;
    println!("The result for puzzle part '{puzzle_part:?}' is: {sum}");

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    pub fn winning_possibilities(&self) -> usize {
        (1..self.time)
            .map(|time_pressed| self.calculate_distance(time_pressed))
            .filter(|distance| distance > &self.record)
            .count()
    }

    pub fn calculate_distance(&self, time_pressed: u64) -> u64 {
        if self.time <= time_pressed {
            return 0;
        }

        let speed = time_pressed;
        let travel_time = self.time - time_pressed;

        speed * travel_time
    }
}

fn calculate_result(input: &str, puzzle_part: PuzzlePart) -> eyre::Result<usize> {
    let res = match puzzle_part {
        PuzzlePart::One => {
            let races = parse_input_part_1(input)?;

            races
                .iter()
                .map(Race::winning_possibilities)
                .reduce(|acc, x| acc * x)
                .unwrap_or(0)
        }
        PuzzlePart::Two => {
            let race = parse_input_part_2(input)?;
            race.winning_possibilities()
        }
    };

    Ok(res)
}

fn parse_input_part_1(input: &str) -> eyre::Result<Vec<Race>> {
    let mut lines = input.lines();
    let time_line = lines.next().wrap_err("time line does not exist")?;
    let record_line = lines.next().wrap_err("record line does not exist")?;

    fn line_iter(line: &str) -> impl Iterator<Item = eyre::Result<u64>> + '_ {
        line.split(" ")
            .skip(1)
            .filter(|s| !s.is_empty())
            .map(u64::from_str)
            .map(|res| res.wrap_err("could not parse part to u64"))
    }

    line_iter(time_line)
        .zip(line_iter(record_line))
        .map(|(time, record)| {
            Ok(Race {
                time: time?,
                record: record?,
            })
        })
        .collect::<eyre::Result<Vec<_>>>()
}

fn parse_input_part_2(input: &str) -> eyre::Result<Race> {
    let mut lines = input.lines();
    let time_line = lines.next().wrap_err("time line does not exist")?;
    let record_line = lines.next().wrap_err("record line does not exist")?;

    let parse_line = |line: &str| {
        line.splitn(2, " ")
            .skip(1)
            .filter(|s| !s.is_empty())
            .flat_map(|str| str.chars())
            .filter_map(|c| c.to_digit(10))
            .fold(0u64, |acc, x| acc * 10 + x as u64)
    };

    let time = parse_line(time_line);
    let record = parse_line(record_line);

    Ok(Race { time, record })
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_calculate_result_puzzle_part_1() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE, PuzzlePart::One)?;
        assert_eq!(res, 288);

        Ok(())
    }

    #[test]
    fn test_calculate_result_puzzle_part_2() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE, PuzzlePart::Two)?;
        assert_eq!(res, 71503);

        Ok(())
    }

    #[test]
    fn test_race_parse_part_1() -> eyre::Result<()> {
        let res = parse_input_part_1(EXAMPLE)?;
        assert_eq!(
            res,
            Vec::from([
                Race { time: 7, record: 9 },
                Race {
                    time: 15,
                    record: 40
                },
                Race {
                    time: 30,
                    record: 200
                },
            ])
        );

        Ok(())
    }

    #[test]
    fn test_race_parse_part_2() -> eyre::Result<()> {
        let res = parse_input_part_2(EXAMPLE)?;
        assert_eq!(
            res,
            Race {
                time: 71530,
                record: 940200,
            }
        );

        Ok(())
    }
}
