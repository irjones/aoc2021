use crate::{Puzzle, PuzzleErrorKind, read_input_or_empty};

struct DayOne {
    pub input: Option<String>,
}

impl DayOne {
    /// Initialize an instance of DayOne
    /// assumes input is in same relative dir
    fn init() -> Self {
        DayOne {
            input: read_input_or_empty("./input.txt")
        }
    }
}

impl Puzzle for DayOne {
    fn do_part_one(&self) -> Result<(), PuzzleErrorKind> {
        let input = match &self.input {
            Some(contents) => contents,
            None => return Err(PuzzleErrorKind::NoInput)
        };
        todo!()
    }

    fn do_part_two(&self) -> Result<(), PuzzleErrorKind> {
        todo!()
    }
}

pub fn get() -> Box<dyn Puzzle> {

}