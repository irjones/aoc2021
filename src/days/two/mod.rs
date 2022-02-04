use crate::{Puzzle, read_input_or_empty, split_text_to_lines};

struct DayTwo {
    input: Option<String>,
    name: String,
}

impl DayTwo {
    fn init() -> Self {
        DayTwo {
            input: read_input_or_empty("./src/days/two/input.txt"),
            name: String::from("Day Two")
        }
    }
}

enum Ordinal {
    Forward,
    Up,
    Down
}

impl Ordinal {
    fn of(str_val: &str) -> Self {
        match str_val {
            "up" => Ordinal::Up,
            "down" => Ordinal::Down,
            "forward" => Ordinal::Forward,
            _ => panic!("Unable to parse value {} into an Ordinal", str_val)
        }
    }
}

impl Puzzle for DayTwo {
    fn print_name(&self) {
        println!("{}", self.name);
    }

    fn do_part_one(&self) {
        let input = match &self.input {
            Some(content) => split_text_to_lines(content),
            None => return
        };
        let mut horizontal_position: i32 = 0;
        let mut depth: i32 = 0;

        for instruction in input {
            let parsed_instruction: Vec<&str> = instruction.split(' ').collect::<Vec<_>>();
            assert_eq!(parsed_instruction.len(), 2, "Could not parse instruction {}", instruction);
            let scalar_value: i32 = parsed_instruction.get(1).unwrap().parse::<i32>().unwrap();
            let parsed_ordinal: Ordinal = Ordinal::of(parsed_instruction.get(0).unwrap());
            match parsed_ordinal {
                Ordinal::Up => depth = depth - scalar_value,
                Ordinal::Down => depth = depth + scalar_value,
                Ordinal::Forward => horizontal_position += scalar_value
            }
        }
        println!("\tOutput: {}", horizontal_position * depth);
    }

    fn do_part_two(&self) {
        let input: Vec<&str> = match &self.input {
            Some(content) => split_text_to_lines(content),
            None => return
        };
        let mut horizontal_position = 0;
        let mut depth = 0;
        let mut aim = 0;

        for instruction in input {
            let parsed_instruction: Vec<&str> = instruction.split(' ').collect::<Vec<_>>();
            assert_eq!(parsed_instruction.len(), 2, "Could not parse instruction {}", instruction);
            let scalar_value: i32 = parsed_instruction.get(1).unwrap().parse::<i32>().unwrap();
            let parsed_ordinal: Ordinal = Ordinal::of(parsed_instruction.get(0).unwrap());
            match parsed_ordinal {
                Ordinal::Up => aim -= scalar_value,
                Ordinal::Down => aim += scalar_value,
                Ordinal::Forward => {
                    horizontal_position += scalar_value;
                    depth += aim * scalar_value;
                }
            }
        }
        println!("\tOutput: {}", horizontal_position * depth);
    }
}

pub fn get() -> Box<dyn Puzzle> {
    Box::new(DayTwo::init())
}
