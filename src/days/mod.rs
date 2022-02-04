mod one;

pub mod days {
    use crate::days::one;
    use crate::Puzzle;

    enum DaysErrorKind {
        CouldNotGetPuzzles,
    }

    fn get_days() -> Result<Vec<Box<Puzzle>>, DaysErrorKind> {
        Ok(vec!(one::get()))
    }
}