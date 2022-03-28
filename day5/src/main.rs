#[macro_use]
extern crate lazy_static;

use fancy_regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day5.txt").expect("cannot open");
    let reader = BufReader::new(file);
    let mut nice_count = 0;
    let mut nice_count_2 = 0;

    for line in reader.lines().filter_map(|result| result.ok()) {
        if double_check(&line) && vowel_check(&line) && forbidden_check(&line) {
            nice_count += 1;
        }
        if separated_repeat_check(&line) && distinct_pair_check(&line) {
            nice_count_2 += 1;
        }
    }

    println!(
        "Final nice count (part 1): {}\nFinal nice count (part 2): {}",
        nice_count, nice_count_2
    );
}

fn double_check(input: &str) -> bool {
    let mut first = true;
    let mut previous: char = '_';

    for c in input.chars() {
        if first {
            previous = c;
            first = false;
        } else {
            if c == previous {
                return true;
            }

            previous = c;
        }
    }
    return false;
}

fn vowel_check(input: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut vowel_count = 0;

    for c in input.chars() {
        if vowels.contains(&c) {
            vowel_count += 1;
        }
    }

    if vowel_count > 2 {
        return true;
    } else {
        return false;
    }
}

fn forbidden_check(input: &str) -> bool {
    let forbidden_list = ["ab", "cd", "pq", "xy"];

    for forbidden in forbidden_list {
        if input.contains(&forbidden) {
            return false;
        }
    }

    return true;
}

fn distinct_pair_check(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\w{2}).*\1+").unwrap();
    }
    return RE.is_match(input).unwrap();
}

fn separated_repeat_check(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\w)\w\1").unwrap();
    }
    return RE.is_match(input).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::distinct_pair_check;
    use crate::double_check;
    use crate::forbidden_check;
    use crate::separated_repeat_check;
    use crate::vowel_check;

    #[test]
    fn double_check_true() {
        assert_eq!(double_check("apple"), true);
    }

    #[test]
    fn double_check_false() {
        assert_eq!(double_check("orange"), false);
    }

    #[test]
    fn vowel_check_true() {
        assert_eq!(vowel_check("orange"), true);
    }

    #[test]
    fn vowel_check_false() {
        assert_eq!(vowel_check("apple"), false);
    }

    #[test]
    fn forbidden_check_true() {
        assert_eq!(forbidden_check("aaa"), true);
    }

    #[test]
    fn forbidden_check_false() {
        assert_eq!(forbidden_check("abcdef"), false);
    }

    #[test]
    fn distinct_pair_check_true() {
        assert_eq!(distinct_pair_check("qjhvhtzxzqqjkmpb"), true);
    }

    #[test]
    fn distinct_pair_check_false() {
        assert_eq!(distinct_pair_check("ieodomkazucvgmuy"), false);
    }

    #[test]
    fn separated_repeat_check_true() {
        assert_eq!(separated_repeat_check("qjhvhtzxzqqjkmpb"), true);
    }

    #[test]
    fn separated_repeat_check_false() {
        assert_eq!(separated_repeat_check("uurcxstgmygtbstg"), false);
    }
}
