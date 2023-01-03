use anyhow;
use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    TOOGLE,
    ON,
    OFF,
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
            2 => (Operation::TOOGLE, instruction_and_start[1].parse()?),
            3 => {
                let operation = match instruction_and_start[1].trim() {
                    "on" => Operation::ON,
                    "off" => Operation::OFF,
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

#[derive(Debug, Clone, Copy)]
struct Vec2(usize, usize);

impl FromStr for Vec2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points: Vec<usize> = Vec::new();
        for point in s.trim().split(',') {
            let value = point.parse()?;
            points.push(value);
        }
        Ok(Self(points[0], points[1]))
    }
}

#[derive(Debug)]
struct Grid {
    // lights: Vec<[bool; 1000]>,
    lights: Vec<[usize; 1000]>,
}

impl Grid {
    fn new() -> Self {
        Self {
            // lights: vec![[false; 1000]; 1000],
            lights: vec![[0; 1000]; 1000],
        }
    }

    fn operate(&mut self, instruction: &Instruction) {
        match instruction.operation {
            Operation::ON => {
                for i in instruction.start.0..=instruction.end.0 {
                    for j in instruction.start.1..=instruction.end.1 {
                        // self.lights[i][j] = true;
                        self.lights[i][j] += 1;
                    }
                }
            }
            Operation::OFF => {
                for i in instruction.start.0..=instruction.end.0 {
                    for j in instruction.start.1..=instruction.end.1 {
                        // self.lights[i][j] = false;
                        if self.lights[i][j] >= 1 {
                            self.lights[i][j] -= 1;
                        }
                    }
                }
            }
            Operation::TOOGLE => {
                for i in instruction.start.0..=instruction.end.0 {
                    for j in instruction.start.1..=instruction.end.1 {
                        // self.lights[i][j] = !self.lights[i][j];
                        self.lights[i][j] += 2;
                    }
                }
            }
        };
    }
}

fn main() {
    let input = std::fs::read_to_string("./day6_input.txt").expect("input file should present");
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse()).flatten().collect();

    let mut grid = Grid::new();
    instructions
        .iter()
        .for_each(|instruction| grid.operate(instruction));

    // let mut on_lights = 0;

    // for i in 0..1000 {
    //     for j in 0..1000 {
    //         if grid.lights[i][j] {
    //             on_lights += 1;
    //         }
    //     }
    // }

    let mut power = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            power += grid.lights[i][j];
        }
    }

    // println!("turned on lights: {}", on_lights);
    println!("brightnes: {}", power);
}
