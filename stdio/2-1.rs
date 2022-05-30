use std::io;

fn main() {
    let s = get_input().split(' ');
    let v: Vec<&'static str> = s.split_whitespaces.collect();
    println!("{}", s);
}

fn get_input() -> String {
    let mut number = String::new();
    io::stdin().read_line(&mut number).ok();
    return number.trim().parse().ok().unwrap();
}