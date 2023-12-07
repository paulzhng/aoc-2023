#![feature(array_chunks)]

use aoc_2023_common::{init, PuzzlePart};

use std::collections::HashMap;
use std::str::FromStr;

use eyre::{bail, ContextCompat, WrapErr};
use once_cell::sync::Lazy;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() -> eyre::Result<()> {
    let puzzle_part = init()?;
    let sum = calculate_result(INPUT, puzzle_part)?;
    println!("The result for puzzle part '{puzzle_part:?}' is: {sum}");

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Mapping {
    source_range: u64,
    destination_range: u64,
    len: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<u64>,
    conversions: HashMap<Category, Vec<Mapping>>,
}

impl Almanac {
    pub fn convert_seed_to_location(&self, seed_number: u64) -> Option<u64> {
        let mut category = Category::Seed;
        let mut number = seed_number;

        loop {
            let Some((new_category, new_number)) = self.convert(category, number) else {
                return None;
            };

            if new_category == Category::Location {
                return Some(new_number);
            }

            category = new_category;
            number = new_number;
        }
    }

    pub fn convert(
        &self,
        source_category: Category,
        source_number: u64,
    ) -> Option<(Category, u64)> {
        let Some(new_category) = source_category.next() else {
            return None;
        };

        let Some(mappings) = self.conversions.get(&source_category) else {
            return None;
        };

        let new_number = mappings
            .iter()
            .copied()
            .find_map(|mapping| {
                let Mapping {
                    source_range,
                    destination_range,
                    len,
                } = mapping;

                if (source_range..source_range + len).contains(&source_number) {
                    Some(destination_range + (source_number - source_range))
                } else {
                    None
                }
            })
            .unwrap_or(source_number);

        Some((new_category, new_number))
    }
}

impl FromStr for Almanac {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let (_, seeds_str) = lines
            .next()
            .wrap_err("expect seeds string")?
            .split_once(' ')
            .wrap_err("invalid format for seeds string")?;

        let seeds = seeds_str
            .split(' ')
            .map(u64::from_str)
            .map(|res| res.wrap_err("failed to parse seed number"))
            .collect::<eyre::Result<Vec<_>>>()?;

        static CONVERSION_HEADING_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?<source>.+)-to-(?<destination>.+) map:").unwrap());
        static CONVERSION_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?<destination>\d+) (?<source>\d+) (?<len>\d+)").unwrap());

        let (_, conversions) = lines.filter(|line| !line.trim().is_empty()).try_fold(
            (None, HashMap::new()),
            |(cur_source, mut conversions), line| {
                if let Some(heading_captures) = CONVERSION_HEADING_REGEX.captures(line) {
                    let source_category: Category = heading_captures
                        .name("source")
                        .wrap_err("invalid regex group")?
                        .as_str()
                        .parse()?;
                    let destination_category: Category = heading_captures
                        .name("destination")
                        .wrap_err("invalid regex group")?
                        .as_str()
                        .parse()?;

                    match source_category.next() {
                        Some(next_category) if next_category != destination_category => bail!(
                            "next category for {source_category:?} not {destination_category:?}"
                        ),
                        None => bail!("final category is input"),
                        _ => return Ok((Some(source_category), conversions)),
                    }
                }

                let Some(cur_source) = cur_source else {
                    bail!("no mapping active for line {line}");
                };

                let Some(conversion_captures) = CONVERSION_REGEX.captures(line) else {
                    bail!("invalid line: {line}");
                };

                let source_range_start: u64 = conversion_captures
                    .name("source")
                    .wrap_err("invalid regex group")?
                    .as_str()
                    .parse()?;
                let destination_range_start: u64 = conversion_captures
                    .name("destination")
                    .wrap_err("invalid regex group")?
                    .as_str()
                    .parse()?;
                let range_len: u64 = conversion_captures
                    .name("len")
                    .wrap_err("invalid regex group")?
                    .as_str()
                    .parse()?;

                let mappings: &mut Vec<_> = conversions.entry(cur_source).or_default();
                mappings.push(Mapping {
                    source_range: source_range_start,
                    destination_range: destination_range_start,
                    len: range_len,
                });

                Ok((Some(cur_source), conversions))
            },
        )?;

        Ok(Self { seeds, conversions })
    }
}

impl Category {
    pub fn next(self) -> Option<Category> {
        let next = match self {
            Self::Seed => Self::Soil,
            Self::Soil => Self::Fertilizer,
            Self::Fertilizer => Self::Water,
            Self::Water => Self::Light,
            Self::Light => Self::Temperature,
            Self::Temperature => Self::Humidity,
            Self::Humidity => Self::Location,
            Self::Location => return None,
        };

        Some(next)
    }
}

impl FromStr for Category {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let category = match s {
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "fertilizer" => Self::Fertilizer,
            "water" => Self::Water,
            "light" => Self::Light,
            "temperature" => Self::Temperature,
            "humidity" => Self::Humidity,
            "location" => Self::Location,
            _ => bail!("unknown category: {s}"),
        };

        Ok(category)
    }
}

fn calculate_result(input: &str, puzzle_part: PuzzlePart) -> eyre::Result<u64> {
    let almanac = Almanac::from_str(input)?;

    let res = match puzzle_part {
        PuzzlePart::One => almanac
            .seeds
            .iter()
            .copied()
            .filter_map(|seed_number| almanac.convert_seed_to_location(seed_number))
            .min()
            .unwrap_or(0),
        PuzzlePart::Two => almanac
            .seeds
            .array_chunks::<2>()
            .copied()
            .flat_map(|[seed_range_start, seed_range_len]| {
                seed_range_start..seed_range_start + seed_range_len
            })
            .filter_map(|seed_number| almanac.convert_seed_to_location(seed_number))
            .min()
            .unwrap_or(0),
    };

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Category::{Seed, Soil};

    const EXAMPLE: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_calculate_result_puzzle_part_1() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE, PuzzlePart::One)?;
        assert_eq!(res, 35);

        Ok(())
    }

    #[test]
    fn test_calculate_result_puzzle_part_2() -> eyre::Result<()> {
        let res = calculate_result(EXAMPLE, PuzzlePart::Two)?;
        assert_eq!(res, 46);

        Ok(())
    }

    #[test]
    fn test_almanac_parse() -> eyre::Result<()> {
        let res = Almanac::from_str(
            "\
seeds: 1 2

seed-to-soil map:
0 0 1

soil-to-fertilizer map:
1 3 2
        ",
        )?;
        assert_eq!(
            res,
            Almanac {
                seeds: vec![1, 2],
                conversions: HashMap::from([
                    (
                        Seed,
                        Vec::from([Mapping {
                            source_range: 0,
                            destination_range: 0,
                            len: 1,
                        }])
                    ),
                    (
                        Soil,
                        Vec::from([Mapping {
                            source_range: 3,
                            destination_range: 1,
                            len: 2,
                        }])
                    )
                ])
            }
        );

        Ok(())
    }
}
