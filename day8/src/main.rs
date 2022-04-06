use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day8.txt").expect("nah");
    let reader = BufReader::new(file);
    let strip = Regex::new(r#"^"|"$"#).unwrap();
    let re = Regex::new(r#"(\\"|\\x[a-fA-F0-9]{2}|\\\\)"#).unwrap();
    let mut diff = 0;
    let mut diff2 = 0;

    for line in reader.lines().filter_map(|result| result.ok()) {
        let mut text = strip.replace_all(&line, "").to_string();
        text = re.replace_all(&text, "!").to_string();
        diff = diff + line.len() - text.len();
        diff2 = diff2 + format!("{:?}", line).len() - line.len();
        println!("diff: {}\ndiff2: {}\n", diff, diff2);
    }
}

