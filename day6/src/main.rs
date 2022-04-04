use std::fs::File;
use std::io::{BufRead, BufReader};

const SIZE: usize = 1000;

fn main() {
    //let mut array_1: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut array_2: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    let file = File::open("day6.txt").expect("cannot open");

    let reader = BufReader::new(file);

    for line in reader.lines().filter_map(|result| result.ok()) {
        let vec: Vec<&str> = line.split(" ").collect();

        let topleft: Vec<usize> = vec[vec.len() - 3]
            .split(",")
            .map(|s| s.parse().expect("error"))
            .collect();

        let bottomright: Vec<usize> = vec[vec.len() - 1]
            .split(",")
            .map(|s| s.parse().expect("error"))
            .collect();

        let cmd = vec[vec.len() - 4];

        for i in topleft[0]..bottomright[0] + 1 {
            for j in topleft[1]..bottomright[1] + 1 {
                //array_1[i][j] = light(array_1[i][j], cmd);
                array_2[i][j] = light2(array_2[i][j], cmd);
            }
        }
    }

    println!("brightness: {}", brightness(array_2));
}

fn light(state: i32, cmd: &str) -> i32 {
    match cmd {
        "toggle" => {
            if state == 1 {
                0
            } else {
                1
            }
        }
        "on" => 1,
        "off" => 0,
        _ => 2,
    }
}

fn light2(state: i32, cmd: &str) -> i32 {
    match cmd {
        "toggle" => state + 2,
        "on" => state + 1,
        "off" => {
            if state > 0 {
                state - 1
            } else {
                state
            }
        }
        _ => 2,
    }
}

fn on_count(array: [[i32; SIZE]; SIZE]) -> i32 {
    let mut count = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            if array[i][j] > 0 {
                count += 1;
            }
        }
    }
    count
}

fn brightness(array: [[i32; SIZE]; SIZE]) -> i32 {
    let mut count = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            count = count + array[i][j];
        }
    }
    count
}
