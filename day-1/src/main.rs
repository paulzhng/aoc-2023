#![feature(iter_collect_into)]

use aoc_2023_common::{init, PuzzlePart};

const WORD_TO_DIGIT: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];
const INPUT: &str = include_str!("input.txt");

fn main() -> eyre::Result<()> {
    let puzzle_part = init()?;
    let sum = calculate_result(INPUT, puzzle_part);
    println!("The result for puzzle part '{puzzle_part:?}' is: {sum}");

    Ok(())
}

fn calculate_result(input: &str, puzzle_part: PuzzlePart) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let mut index_to_digit = Vec::new();

            line.chars()
                .enumerate()
                .filter_map(|(idx, c)| Some((idx, c.to_digit(10)?)))
                .collect_into(&mut index_to_digit);

            if puzzle_part == PuzzlePart::Two {
                WORD_TO_DIGIT
                    .iter()
                    .flat_map(|(word, digit)| {
                        line.match_indices(word).map(|(idx, _)| (idx, *digit))
                    })
                    .collect_into(&mut index_to_digit);
            }

            index_to_digit.sort_unstable_by_key(|(idx, _)| *idx);

            let first_digit = index_to_digit.first().map(|(_, digit)| *digit)?;
            let second_digit = index_to_digit.last().map(|(_, digit)| *digit)?;

            Some((first_digit, second_digit))
        })
        .map(|(first_digit, last_digit)| first_digit * 10 + last_digit)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_result_puzzle_part_1() {
        let res = calculate_result(
            r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
            "#,
            PuzzlePart::One,
        );
        assert_eq!(res, 142);
    }

    #[test]
    fn test_calculate_result_puzzle_part_2() {
        let res = calculate_result(
            r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            "#,
            PuzzlePart::Two,
        );
        assert_eq!(res, 281);
    }
}
