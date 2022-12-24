use anyhow;
use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    toogle,
    on,
    off,
}

#[derive(Debug)]
struct Instruction {
    start: Vec2,
    end: Vec2,
    operation: Operation,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw_instructions: Vec<&str> = s.trim().split("through").collect();
        let end_position = raw_instructions[1].parse::<Vec2>()?;

        let instruction_and_start: Vec<&str> = raw_instructions[0].trim().split(' ').collect();

        let (operation, start_position): (Operation, Vec2) = match instruction_and_start.len() {
            2 => (Operation::toogle, instruction_and_start[1].parse()?),
            3 => {
                let operation = match instruction_and_start[1].trim() {
                    "on" => Operation::on,
                    "off" => Operation::off,
                    _ => return Err(anyhow::Error::msg("unexpected operation")),
                };
                (operation, instruction_and_start[2].parse()?)
            }
            _ => return Err(anyhow::Error::msg("unexpected len")),
        };

        Ok(Instruction {
            start: start_position,
            end: end_position,
            operation,
        })
    }
}

#[derive(Debug)]
struct Vec2(i32, i32);

impl FromStr for Vec2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points: Vec<i32> = Vec::new();
        for point in s.trim().split(',') {
            let value = point.parse()?;
            points.push(value);
        }
        Ok(Self(points[0], points[1]))
    }
}

struct Grid {
    on: Vec<Vec2>,
}


fn main() {
    let input = std::fs::read_to_string("./day6_input.txt").expect("input file should present");
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse()).flatten().collect();
    println!("{:#?}", instructions);

}
