use std::fs;

mod days;

pub trait Puzzle {
    fn print_name(&self);
    fn do_part_one(&self);
    fn do_part_two(&self);
}

pub fn run_puzzles() {
    let puzzles: Vec<Box<dyn Puzzle>> = days::get_days();
    for puzzle in puzzles {
        puzzle.print_name();
        println!("- Part One -");
        puzzle.do_part_one();
        println!("- Part Two -");
        puzzle.do_part_two();
        println!("\n");
    }
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

fn split_text_lines_to_i32(text: &str) -> Vec<i32> {
    text.split('\n')
        .collect::<Vec<_>>()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}