use eyre::bail;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub enum PuzzlePart {
    One,
    Two,
}

impl PuzzlePart {
    pub fn from_env() -> eyre::Result<PuzzlePart> {
        let puzzle_part = match std::env::var("PUZZLE_PART") {
            Ok(val) if val.eq_ignore_ascii_case("one") => PuzzlePart::One,
            Ok(val) if val.eq_ignore_ascii_case("two") => PuzzlePart::Two,
            Err(_) => PuzzlePart::One,
            Ok(val) => bail!("Unknown puzzle_part: {val}"),
        };

        Ok(puzzle_part)
    }
}

pub fn init() -> eyre::Result<PuzzlePart> {
    color_eyre::install().expect("couldn't install color_eyre");
    PuzzlePart::from_env()
}
