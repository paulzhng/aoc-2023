use aoc_2023_common::{init, PuzzlePart};
use eyre::ContextCompat;
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

fn main() -> eyre::Result<()> {
    let puzzle_part = init()?;
    let sum = calculate_result(INPUT, puzzle_part)?;
    println!("The result for puzzle part '{puzzle_part:?}' is: {sum}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const GAME_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"Game (?<game_id>\d+): (?<cube_sets>.+)").unwrap());
        let game_captures = GAME_REGEX
            .captures(s)
            .wrap_err("game pattern didn't match")?;

        let game_id_match = game_captures
            .name("game_id")
            .context("invalid capture group name")?;
        let cube_sets_match = game_captures
            .name("cube_sets")
            .context("invalid capture group name")?;

        let game_id: u32 = game_id_match.as_str().parse()?;
        let cube_sets = cube_sets_match
            .as_str()
            .split("; ")
            .map(CubeSet::from_str)
            .collect::<eyre::Result<Vec<_>>>()?;

        Ok(Self {
            id: game_id,
            cube_sets,
        })
    }
}

impl FromStr for CubeSet {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cube_set = s
            .split(", ")
            .filter_map(|color_and_amount| {
                let mut split = color_and_amount.split(" ");
                Some((
                    split.next()?.parse::<u32>().ok()?,
                    split.next()?.to_string(),
                ))
            })
            .fold(CubeSet::empty(), |mut acc, (amount, color)| {
                match color.as_str() {
                    "red" => acc.red += amount,
                    "green" => acc.green += amount,
                    "blue" => acc.blue += amount,
                    _ => {}
                }

                acc
            });
        Ok(cube_set)
    }
}

impl CubeSet {
    pub fn empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn componentwise_max(self, rhs: CubeSet) -> CubeSet {
        Self {
            red: self.red.max(rhs.red),
            green: self.green.max(rhs.green),
            blue: self.blue.max(rhs.blue),
        }
    }

    pub fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn calculate_result(input: &str, puzzle_part: PuzzlePart) -> eyre::Result<u32> {
    let gmaes = parse_input(input)?;
    let sum = match puzzle_part {
        PuzzlePart::One => gmaes
            .iter()
            .filter(|game| {
                game.cube_sets.iter().all(|cube_set| {
                    cube_set.red <= 12 && cube_set.green <= 13 && cube_set.blue <= 14
                })
            })
            .map(|game| game.id)
            .sum(),
        PuzzlePart::Two => gmaes
            .iter()
            .map(|game| {
                game.cube_sets
                    .iter()
                    .copied()
                    .fold(CubeSet::empty(), CubeSet::componentwise_max)
            })
            .map(CubeSet::power)
            .sum(),
    };

    Ok(sum)
}

fn parse_input(input: &str) -> eyre::Result<Vec<Game>> {
    input.lines().map(Game::from_str).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_result_puzzle_part_1() -> eyre::Result<()> {
        let res = calculate_result(
            r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            PuzzlePart::One,
        )?;
        assert_eq!(res, 8);

        Ok(())
    }

    #[test]
    fn test_calculate_result_puzzle_part_2() -> eyre::Result<()> {
        let res = calculate_result(
            r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            PuzzlePart::One,
        )?;
        assert_eq!(res, 2286);

        Ok(())
    }

    #[test]
    fn test_game_parse() -> eyre::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_str(input)?;

        assert_eq!(
            game,
            Game {
                id: 1,
                cube_sets: vec![
                    CubeSet {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    CubeSet {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    CubeSet {
                        red: 0,
                        green: 2,
                        blue: 0
                    }
                ],
            }
        );

        Ok(())
    }
}
