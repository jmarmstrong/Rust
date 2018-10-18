use std::io;

fn main() {
    println!("Hello to the game of games, the simulators of simulators ");
    println!("Welcome to The Arma 3 simulator");
    println!("Are you Ready?");
    let input = input();
    println!("{}", input);
    match input {};
}

fn input() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("You failed at life :(");
    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("IM PANICING!"),
    };
    input
}
