use anyhow::{self, Ok};
use std::{fs, str::FromStr};

#[derive(Debug)]
struct VisitedHouse {
    house: (i32, i32),
    presents: usize,
}

#[derive(Debug)]
struct VisitedLocations {
    locations: Vec<VisitedHouse>,
    current_position_santa: (i32, i32),
    current_position_robot: (i32, i32),
}

impl VisitedLocations {
    fn new() -> Self {
        Self {
            locations: vec![VisitedHouse {
                house: (0, 0),
                presents: 2,
            }],
            current_position_santa: (0, 0),
            current_position_robot: (0, 0),
        }
    }

    fn amount_of_places_with_presents_at_least(&self, amount_of_presents: usize) -> usize {
        return self
            .locations
            .iter()
            .filter(|l| l.presents >= amount_of_presents)
            .count();
    }
}

impl FromStr for VisitedLocations {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut locations = VisitedLocations::new();
        let mut is_santa = true;

        for direction in s.chars() {
            let starter_location = match is_santa {
                true => &mut locations.current_position_santa,
                false => &mut locations.current_position_robot,
            };

            let next_location = match direction {
                '^' => (starter_location.0, starter_location.1 + 1),

                'v' => (starter_location.0, starter_location.1 - 1),

                '>' => (starter_location.0 + 1, starter_location.1),

                '<' => (starter_location.0 - 1, starter_location.1),

                _ => return Err(anyhow::Error::msg("provided unsupported direction")),
            };

            let previouslu_visited_location = locations
                .locations
                .iter_mut()
                .find(|l| l.house == next_location);

            match previouslu_visited_location {
                None => locations.locations.push(VisitedHouse {
                    house: next_location,
                    presents: 1,
                }),
                Some(location) => location.presents += 1,
            }

            *starter_location = next_location;

            is_santa = !is_santa;
        }

        Ok(locations)
    }
}

fn main() {

    let input = fs::read_to_string("./day3_input.txt").expect("missing input from day3_input.txt");
    
    let list_of_visited_locations = input.parse::<VisitedLocations>().expect("Santa was correctly navigated");

    println!(
        "houses with at least 1 present: {}",
        list_of_visited_locations.amount_of_places_with_presents_at_least(1)
    )
}
