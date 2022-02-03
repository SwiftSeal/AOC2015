use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);
    let contents = fs::read_to_string(filename).expect("Unable to read file");

    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in contents.lines() {
        let sums = Sums::calculate(&line);

        total_paper += sums.paper;
        total_ribbon += sums.ribbon;
    }

    println!(
        "total paper needed: {}\ntotal ribbon needed: {}",
        total_paper, total_ribbon
    );
}

fn parse_config(args: &[String]) -> &str {
    if args.len() < 2 {
        panic!("missing argument");
    }

    let filename = &args[1];

    filename
}

struct Sums {
    paper: i32,
    ribbon: i32,
}

impl Sums {
    fn calculate(line: &str) -> Sums {
        let mut nums: Vec<i32> = line.split('x').map(|s| s.parse().unwrap()).collect();

        nums.sort();

        let (&l, &w, &h) = (&nums[0], &nums[1], &nums[2]);

        let paper = 3 * l * w + 2 * w * h + 2 * l * h;

        let ribbon = l + l + w + w + l * w * h;

        Sums { paper, ribbon }
    }
}
