use std::io;

fn main () {
    let s = get_input();
    let b: u8 = s.as_bytes()[2];
    let c: char = b as char;
    println!("{}", c);
}

fn get_input() -> String {
    let mut number = String::new();
    io::stdin().read_line(&mut number).ok();
    return number.trim().parse().ok().unwrap();
}




/*use std::io;
//use io::stdout;

fn main() {
  let s = get_input();
  //let c = s.chars().;
  let char_vec: Vec<char> = s.chars().collect();
  println!("{:?}", char_vec.get(2));
  //println!("{:}", c);
  //Stdout().flush();
}

fn get_input() -> String {
    let mut number = String::new();
    io::stdin().read_line(&mut number).ok();
    return number.trim().parse().ok().unwrap();
}*/