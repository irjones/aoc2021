use std::fs;

mod days;

enum PuzzleErrorKind {
    PuzzleIncomplete,
    PuzzleFailedUnexpectedly,
    NoInput
}

trait Puzzle {
    fn do_part_one(&self) -> Result<(), PuzzleErrorKind>;
    fn do_part_two(&self) -> Result<(), PuzzleErrorKind>;
}

fn getPuzzles() -> Vec<Puzzle> {
    let puzzles: Vec<Puzzle> = vec!(days::get_days());
    puzzles
}

fn read_input_or_empty(path: &str) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(contents) => Some(contents),
        Err(err) => {
            println!("Error reading input: {}", err);
            None
        }
    }
}