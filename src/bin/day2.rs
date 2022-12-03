use anyhow::Result;
use std::fs;

fn calculate_amount_of_present_paper(length: usize, width: usize, height: usize) -> Result<usize> {
    let additional_paper = match [length * width, width * height, height * length].iter().min() {
        Some(v) => v.to_owned(),
        _ => 0usize,
    };
    return Ok((2 * length * width + 2 * width * height + 2 * height * length) + additional_paper);
}

fn main() -> Result<()> {
    let mut total_paper = 0;
    let input = fs::read_to_string("./day2_input.txt")?;
    input.lines().for_each(|line| {
        let sizes:Vec<usize> = line
        .split('x')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
        let paper = calculate_amount_of_present_paper(sizes[0], sizes[1], sizes[2]).unwrap();
        total_paper += paper;
    });

    println!("total square feet of wrapping paper is {}", total_paper);
    
    Ok(())
}
