use crate::{Puzzle, read_input_or_empty, split_text_lines_to_i32};

struct DayOne {
    pub input: Option<String>,
    name: String,
}

impl DayOne {
    fn init() -> Self {
        DayOne {
            name: String::from("Day One"),
            input: read_input_or_empty("./src/days/one/input.txt")
        }
    }
}

impl Puzzle for DayOne {
    fn print_name(&self) {
        println!("{}", self.name);
    }

    fn do_part_one(&self) {
        let input = match &self.input {
            Some(contents) => split_text_lines_to_i32(contents),
            None => return
        };
        let mut increase_count: i32 = 0;
        let mut last_number = input.first().unwrap();
        for number in input.iter() {
            if number > last_number {
                increase_count += 1;
            }
            last_number = &number;
        }
        println!("\tOutput: {}", increase_count);
    }

    fn do_part_two(&self) {
        let input = match &self.input {
            Some(contents) => split_text_lines_to_i32(contents),
            None => return
        };
        let mut increase_count: i32 = 0;
        let mut last_sum: i32 = 0;
        let mut position: usize = 0;

        while position + 3 <= input.len() {
            let sum = input.get(position).unwrap()
                + input.get(position + 1).unwrap()
                + input.get(position + 2).unwrap();

            if sum > last_sum && last_sum != 0 {
                increase_count += 1;
            }
            last_sum = sum;
            position += 1;
        }

        println!("\tOutput: {}", increase_count);
    }
}

pub fn get() -> Box<dyn Puzzle> {
    Box::new(DayOne::init())
}