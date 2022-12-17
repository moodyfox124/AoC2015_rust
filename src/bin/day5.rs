use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Wishes {
    naughty: usize,
    nice: usize,
}

const ALLOWED_VOWELS: &str = "aeiou";
const DISALLOWED_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

impl FromStr for Wishes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut wishes = Wishes {
            naughty: 0,
            nice: 0,
        };

        'lines_loop: for line in s.lines() {
            // part 1
            // for disallowed_value in DISALLOWED_STRINGS {
            //     if line.contains(disallowed_value) {
            //         wishes.naughty += 1;
            //         continue 'lines_loop;
            //     }
            // }

            // let mut vowel_counter = 0;
            // for c in line.chars() {
            //     if ALLOWED_VOWELS.contains(c) {
            //         vowel_counter += 1;

            //         if vowel_counter == 3 {
            //             break;
            //         }
            //     }
            // }

            // if vowel_counter != 3 {
            //     wishes.naughty += 1;
            //     continue 'lines_loop;
            // }

            // let chars: Vec<char> = line.chars().collect();

            // for (index, &c) in chars.iter().enumerate() {

            //     let next_c = match chars.get(index + 1) {
            //         None => continue,
            //         Some(v) => v,
            //     };

            //     if c == *next_c {
            //         wishes.nice += 1;
            //         continue 'lines_loop;
            //     }
            // }

            // wishes.naughty += 1;

            // part 2

            let chars: Vec<char> = line.chars().collect();

            for (index, c) in chars.iter().enumerate() {
                let next_c = match chars.get(index + 1) {
                    None => {
                        wishes.naughty += 1;
                        continue 'lines_loop;
                    }
                    Some(v) => v,
                };

                let pair = format!("{}{}", c, next_c);
                let str_to_check = &chars[index + 2..].iter().collect::<String>();

                if str_to_check.len() == 0 {
                    wishes.naughty += 1;
                    continue 'lines_loop;
                }

                if str_to_check.contains(&pair) {
                    break;
                }
            }

            for (index, &c) in chars.iter().enumerate() {
                let next_c = match chars.get(index + 2) {
                    None => continue,
                    Some(v) => v,
                };

                if c == *next_c {
                    wishes.nice += 1;
                    continue 'lines_loop;
                }
            }

            wishes.naughty += 1;
        }

        return Ok(wishes);
    }
}

fn main() {
    let input = fs::read_to_string("./day5_input.txt").expect("missing input file");
    let wishes: Wishes = input.parse().unwrap();
    println!("{:#?}", wishes);
}
