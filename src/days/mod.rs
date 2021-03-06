mod one;
mod two;

use crate::Puzzle;

pub(crate) fn get_days() -> Vec<Box<dyn Puzzle>> {
    vec!(one::get(), two::get())
}
