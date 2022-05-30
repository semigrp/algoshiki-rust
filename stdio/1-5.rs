use std::io;

fn main() {
    let x = get_input();
    println!("{}", 24 - (x % 24));
}

fn get_input() -> i32 {
    let mut number = String::new();
    io::stdin().read_line(&mut number).ok();
    return number.trim().parse().ok().unwrap();
}