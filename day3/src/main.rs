use std::fs;

fn main() {
    let mut santa_coordinates = [0, 0];
    let mut robot_coordinates = [0, 0];
    let mut houses: Vec<House> = Vec::new();
    let mut position = 0;

    let first_house = House {
        coordinates: [0, 0],
        presents: 2,
    }; //initialise first house 

    houses.push(first_house);

    let file = fs::read_to_string("instructions.txt").expect("Error handling file");

    for c in file.chars() {
        position += 1; //update position for even/odd switching

        let current_coordinates = if let 0 = position % 2 {
            update_coords(&c, &mut robot_coordinates) //if even, pass mutable robot coordinates and update local coordinates
        } else {
            update_coords(&c, &mut santa_coordinates)
        };

        let mut found = false;

        //iterate through list of houses, if address is present then update its present
        // value and begin next instruction. If house is not found, then append a new
        // house with the current coordinates to the list and give it a present.
        for address in &mut houses {
            if check(address, current_coordinates) {
                found = true;
                break;
            }
        }

        if !found {
            let house = House {
                coordinates: current_coordinates,
                presents: 1,
            };

            houses.push(house);
        }
    }

    println!(
        "Final Santa coords: {},{}\nFinal Robot coords: {},{}\nTotal addresses visited: {}",
        santa_coordinates[0],
        santa_coordinates[1],
        robot_coordinates[0],
        robot_coordinates[1],
        houses.len()
    );
}

struct House {
    coordinates: [i32; 2],
    presents: i32,
}

fn check(house: &mut House, coords: [i32; 2]) -> bool {
    if house.coordinates == coords {
        house.presents += 1;
        true
    } else {
        false
    }
}

fn update_coords(order: &char, coords: &mut [i32; 2]) -> [i32; 2] {
    match order {
        '^' => coords[1] += 1,
        '>' => coords[0] += 1,
        '<' => coords[0] -= 1,
        'v' => coords[1] -= 1,
        _ => println!("other found"),
    }
    *coords
}
