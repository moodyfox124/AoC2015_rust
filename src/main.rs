use std::fs;
use anyhow::{Result, Ok};

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    let mut floor = 0;
    let mut basement_enter_character_position: Option<usize> = None;
    let mut is_basement_character_found = false;

    input.chars().enumerate().for_each(|(index, character)| {
        match character {
            '(' => floor +=1,
            ')' => floor -=1,
            _ => return,
        };
        if is_basement_character_found == false && floor == -1 {
            basement_enter_character_position = Some(index + 1);
            is_basement_character_found = true;
        }
    });

    println!("Santa will end up on floor {}", floor);
    println!("Santa went in the basement after instruction in position {}", basement_enter_character_position.unwrap());
    return Ok(());
}
