use std::io;

fn main() {
    println!("Hello to the game of games, the simulators of simulators ");
    println!("Welcome to The Arma 3 simulator");
    println!("Are you Ready?");
    println!("1) Yes");
    println!("2) No");
    let choice1 = input_n();
    match choice1 {
        1 => println!("Ok well lets go!"),
        2 => println!("Well tuff you have to"),
        _ => panic!("You enterd a unknown character"),
    };
    println!("You are in command of a squad and you have to make sure they live");
}

fn input_n() -> i32 {
    let mut input_n = String::new();
    io::stdin()
        .read_line(&mut input_n)
        .expect("You failed at life :(");
    let input_n: i32 = match input_n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("IM PANICING!"),
    };
    input_n
}
