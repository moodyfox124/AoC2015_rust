use anyhow::Result;
use std::{fs, str::FromStr};

#[derive(Debug)]
struct PresentSizes {
    values: [usize; 3],
}

impl FromStr for PresentSizes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sizes: Vec<usize> = s
            .split('x')
            .map(|v| {
                v.parse::<usize>()
                    .expect("provided string contain non number value")
            })
            .collect();
        Ok(Self {
            values: [sizes[0], sizes[1], sizes[2]],
        })
    }
}

fn calculate_amount_of_present_paper(sizes: &PresentSizes) -> usize {
    let PresentSizes {
        values: [length, width, height],
    } = sizes;

    let sides = [length * width, width * height, height * length];
    let min_size = sides.iter().min().expect("min side was not found");

    (2 * sides[0] + 2 * sides[1] + 2 * sides[2]) + min_size
}

fn calculate_ribbon(sizes: &PresentSizes) -> usize {
    let &PresentSizes { values } = sizes;

    let bow = values.iter().fold(1, |prev, curr| curr * prev);

    let mut sorted_arr = values;
    sorted_arr.sort();

    let wrap = sorted_arr[0] * 2 + sorted_arr[1] * 2;

    wrap + bow
}

fn main() -> Result<()> {
    let required_paper = fs::read_to_string("./day2_input.txt")?
        .lines()
        .flat_map(|v| v.parse::<PresentSizes>())
        .fold((0, 0), |init, v| {
            (
                init.0 + calculate_amount_of_present_paper(&v),
                init.1 + calculate_ribbon(&v),
            )
        });

    println!(
        "total square feet of wrapping paper is {}",
        required_paper.0
    );
    println!("total square feet of ribbon is {}", required_paper.1);
    Ok(())
}
