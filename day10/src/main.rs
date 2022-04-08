fn main() {
    let input = parse_str("22");

    let mut output = looksay(&input);
    while true {
        let prev_len: f32 = output.len() as f32;
        output = looksay(&output);
        let new_len: f32 = output.len() as f32;
        println!("{:?}", new_len/prev_len);
    }
}

fn looksay(input: &Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    let mut i: usize = 0;

    while i < input.len() {
        let mut j: usize = i;

        while j < input.len() && input[j] == input[i] {
            j += 1;
        }

        output.push(u8::try_from(j - i).unwrap());
        output.push(input[i]);

        i += j - i;
    }
    output
}

fn parse_str(input: &str) -> Vec<u8> {
    let output: Vec<u8> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    output
}
