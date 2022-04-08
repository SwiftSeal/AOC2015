use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Trip {
    distance: Option<i32>,
    route: Option<Vec<String>>,
}

impl Trip {
    fn get_status(&self) -> String {
        let joined = self.route.as_ref().unwrap().join(" -> ");
        format!("Route: {}\nDistance: {}", joined, self.distance.unwrap())
    }
}

fn main() {
    let routes = spawn_hashmap("day9.txt").unwrap();
    let locations = get_locations("day9.txt").unwrap();
    let mut shortest = create_trip();
    let mut longest = create_trip();

    for perm in locations.iter().permutations(locations.len()).unique() {
        let perm2: Vec<String> = perm.iter().map(|s| s.to_string()).collect();
        let dist = calculate_distance(perm, &routes);
        
        if shortest.distance.is_none() || dist < shortest.distance.unwrap() {
            shortest.distance = Some(dist);
            shortest.route = Some(perm2.clone());
        }
        if longest.distance.is_none() || dist > longest.distance.unwrap() {
            longest.distance = Some(dist);
            longest.route = Some(perm2);
        }
    }

    println!(
        "Shortest:\n{}\n\nLongest:\n{}\n",
        shortest.get_status(),
        longest.get_status()
    );
}

fn create_trip() -> Trip {
    Trip {
        distance: None,
        route: None,
    }
}

fn spawn_hashmap(handle: &str) -> Result<HashMap<(String, String), i32>, Box<dyn Error>> {
    let file = File::open(handle)?;
    let reader = BufReader::new(file);
    let mut routes: HashMap<(String, String), i32> = HashMap::new();

    for line in reader.lines().filter_map(|result| result.ok()) {
        let split: Vec<&str> = line.split(" ").collect();

        let key = if split[0] < split[2] {
            (split[0].to_string(), split[2].to_string())
        } else {
            (split[2].to_string(), split[0].to_string())
        };

        routes.insert(key, split[4].parse::<i32>().unwrap());
    }

    Ok(routes)
}

fn get_locations(handle: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(handle)?;
    let reader = BufReader::new(file);
    let mut locations: Vec<String> = Vec::new();

    for line in reader.lines().filter_map(|result| result.ok()) {
        let split: Vec<&str> = line.split(" ").collect();

        if !locations.contains(&split[0].to_string()) {
            locations.push(split[0].to_string());
        }
        if !locations.contains(&split[2].to_string()) {
            locations.push(split[2].to_string());
        }
    }

    Ok(locations)
}

fn calculate_distance(order: Vec<&String>, hash: &HashMap<(String, String), i32>) -> i32 {
    let mut dist: i32 = 0;
    for i in 0..order.len() - 1 {
        let loc = if order[i] < order[i + 1] {
            (order[i].clone(), order[i + 1].clone())
        } else {
            (order[i + 1].clone(), order[i].clone())
        };

        dist += hash.get(&loc).unwrap();
    }

    dist
}
