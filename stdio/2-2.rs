use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let inputs: Vec<u8> = line
        .split_whitespace()
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();

    let res = if inputs[0] > inputs[1] {
        inputs[0]
    } else {
        inputs[1]
    };

    println!("{}", res);
}
