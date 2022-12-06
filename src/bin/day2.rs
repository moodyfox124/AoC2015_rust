use anyhow;
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
    if let Some(v) = [length * width, width * height, height * length]
        .iter()
        .min()
    {
        return (2 * length * width + 2 * width * height + 2 * height * length) + v;
    };
    return 0;
}

fn calculate_ribbon(sizes: &PresentSizes) -> usize {
    let &PresentSizes {
        values: [length, width, height],
    } = sizes;
    let bow = length * width * height;
    let max = [length, width, height].iter().max().unwrap().to_owned();
    let min_sides: Vec<usize> = [length, width, height]
        .into_iter()
        .filter(|v| v < &max)
        .collect();
    let wrap = min_sides.get(0).unwrap_or(&max) * 2 + min_sides.get(1).unwrap_or(&max) * 2;
    return wrap + bow;
}

fn main() -> Result<()> {
    let required_paper = fs::read_to_string("./day2_input.txt")?
        .lines()
        .map(|v| PresentSizes::from_str(v))
        .flatten()
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
