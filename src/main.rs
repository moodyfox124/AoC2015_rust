use std::fs;
use anyhow::{Result, Ok};

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    let mut floor = 0;

    input.chars().for_each(|character| {
        match character {
            '(' => floor +=1,
            ')' => floor -=1,
            _ => return, 
        };
    });

    println!("Santa will end up on floor {}", floor);
    return Ok(());
}
