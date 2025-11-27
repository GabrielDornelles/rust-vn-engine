use std::{io};
use std::num::ParseIntError;
use crate::scene::ChoiceOption;

fn parse_user_string(input: &str) -> Result<usize, ParseIntError> {
    input.trim().parse::<usize>()
}

pub fn get_next_from_user_choice(options: &[ChoiceOption]) -> Option<String> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        match parse_user_string(&input) {
            Ok(i) if i < options.len() => {
                // Valid option → return its "next" value
                return Some(options[i].next.clone());
            }
            _ => {
                println!("Invalid choice. Please type a number between 0 and {}.", options.len() - 1);
            }
        }
    }
}