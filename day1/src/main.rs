fn main() {
    let floor_code = String::from("()()") // Probably better to handle from file contents rather than string.
    let mut floor = 0;
    let mut count = 0;

    for c in floor_code.chars() {
        
        count +=1;

        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -=1;
        }

        if floor == -1 {
            println!("position is: {}", count); // will print out all instances where -1 is passed.
        }
    }

    println!("{}", floor);
}
