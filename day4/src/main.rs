use md5::{Digest, Md5};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (input, zeroes) = return_arg(&args);

    let mut v = Vec::new(); // Used to create hex header

    for _i in 0..zeroes {
        v.push('0');
    }

    let front: String = v.into_iter().collect(); // make header of size 'zeroes'

    println!("{}", bruteforce(input, &front));
}

fn hashcheck(input: &str, front: &str) -> bool {
    let mut hasher = Md5::new();

    hasher.update(input);

    let result = format!("{:x}", hasher.finalize()); // format to hex for parsing

    if &result[0..front.chars().count()] == front {
        true
    } else {
        false
    }
}

fn bruteforce(input: &str, front: &str) -> String {
    let mut count = 0;

    loop {
        let numbered_input = format!("{}{}", input, count);

        if hashcheck(&numbered_input, &front) == true {
            break numbered_input;
        } else {
            count += 1;
        }
    }
}

fn return_arg(args: &[String]) -> (&str, i32) {
    if args.len() < 3 {
        panic!("missing argument(s)");
    }

    let input = &args[1];
    let zeroes: i32 = args[2].parse().unwrap();
    (input, zeroes)
}
