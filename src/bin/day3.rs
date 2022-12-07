use std::fs;

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

fn main() {
    let mut is_santa = true;

    let input = fs::read_to_string("./day3_input.txt").expect("missing input from day3_input.txt");
    let mut list_of_visited_places = VisitedLocations {
        locations: vec![VisitedHouse {
            house: (0, 0),
            presents: 2,
        }],
        current_position_santa: (0, 0),
        current_position_robot: (0, 0),
    };

    input.chars().for_each(|direction| {
        let mut last_visited_place = match is_santa {
            true => list_of_visited_places.current_position_santa,
            false => list_of_visited_places.current_position_robot,
        };

        last_visited_place = match direction {
            '^' => (last_visited_place.0, last_visited_place.1 + 1),

            'v' => (last_visited_place.0, last_visited_place.1 - 1),

            '>' => (last_visited_place.0 + 1, last_visited_place.1),

            '<' => (last_visited_place.0 - 1, last_visited_place.1),

            _ => panic!("unsupported symbol"),
        };

        let visited_location = list_of_visited_places
            .locations
            .iter_mut()
            .find(|v| v.house == last_visited_place);

        match visited_location {
            Some(location) => location.presents += 1,
            None => list_of_visited_places.locations.push(VisitedHouse {
                house: last_visited_place,
                presents: 1,
            }),
        }

        match is_santa {
            true => list_of_visited_places.current_position_santa = last_visited_place,
            false => list_of_visited_places.current_position_robot = last_visited_place,
        }

        is_santa = !is_santa;
    });

    println!(
        "houses with at least 1 present: {}",
        list_of_visited_places.locations.len()
    );
}
