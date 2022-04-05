use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day7.txt").expect("nah");
    let reader = BufReader::new(file);
    let mut wires: HashMap<String, String> = HashMap::new();
    let mut cache: HashMap<String, i32> = HashMap::new();

    for line in reader.lines().filter_map(|result| result.ok()) {
        let mut iter = line.split(" -> ");
        let cmd = iter.next().unwrap();
        let key = iter.next().unwrap();

        wires.insert(key.to_string(), cmd.to_string());
    }

    let solution = calculate("a".to_string(), &wires, &mut cache);
    println!("solution for part 1: {}", solution);

    wires.insert("b".to_string(), solution.to_string()); // updating b to value of a
    let mut cache: HashMap<String, i32> = HashMap::new(); // refresh cache

    let solution = calculate("a".to_string(), &wires, &mut cache);
    println!("solution for part 2: {}", solution);
}

fn calculate(key: String, h: &HashMap<String, String>, c: &mut HashMap<String, i32>) -> i32 {
    if c.contains_key(&key) {
        return *c.get(&key).unwrap();
    } // if cache contains solved key, return this

    if key.parse::<i32>().is_ok() {
        return key.parse::<i32>().unwrap();
    } // if command can be parsed as an integer, return it

    let cmd: Vec<&str> = h.get(&key).unwrap().split(" ").collect(); // parse command into vector

    if cmd.contains(&"NOT") {
        let soln = !calculate(cmd[1].to_string(), h, c);
        c.insert(key.clone(), soln); // once solution has been calculated, add this to the cache
        return soln;
    }

    if cmd.contains(&"AND") {
        let soln = calculate(cmd[0].to_string(), h, c) & calculate(cmd[2].to_string(), h, c);
        c.insert(key.clone(), soln);
        return soln;
    } else if cmd.contains(&"OR") {
        let soln = calculate(cmd[0].to_string(), h, c) | calculate(cmd[2].to_string(), h, c);
        c.insert(key.clone(), soln);
        return soln;
    } else if cmd.contains(&"LSHIFT") {
        let soln = calculate(cmd[0].to_string(), h, c) << calculate(cmd[2].to_string(), h, c);
        c.insert(key.clone(), soln);
        return soln;
    } else if cmd.contains(&"RSHIFT") {
        let soln = calculate(cmd[0].to_string(), h, c) >> calculate(cmd[2].to_string(), h, c);
        c.insert(key.clone(), soln);
        return soln;
    } else {
        let soln = calculate(cmd[0].to_string(), h, c);
        c.insert(key.clone(), soln);
        return soln;
    }
}
