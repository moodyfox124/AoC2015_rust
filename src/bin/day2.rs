use anyhow::Result;
use std::fs;

fn calculate_amount_of_present_paper(length: usize, width: usize, height: usize) -> Result<usize> {
    if let Some(v) = [length * width, width * height, height * length]
        .iter()
        .min()
    {
        return Ok((2 * length * width + 2 * width * height + 2 * height * length) + v);
    };
    return Ok(0usize);
}

fn calculate_ribbon(length: usize, width: usize, height: usize) -> usize {
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
    let mut total_paper = 0;
    let mut total_ribbon = 0;
    let input = fs::read_to_string("./day2_input.txt")?;
    input.lines().for_each(|line| {
        let sizes: Vec<usize> = line
            .split('x')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let paper = calculate_amount_of_present_paper(sizes[0], sizes[1], sizes[2]).unwrap();
        total_paper += paper;

        let ribbon = calculate_ribbon(sizes[0], sizes[1], sizes[2]);
        total_ribbon += ribbon;
    });

    println!("total square feet of wrapping paper is {}", total_paper);
    println!("total square feet of ribbon is {}", total_ribbon);
    Ok(())
}
