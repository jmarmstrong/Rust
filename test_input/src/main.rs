fn main() {
    use std::io;

println!("Whats your name?");
let mut input = String::new();
match io::stdin().read_line(&mut input) {
    Ok(_n) => {
        println!("hello {}", input);
    }
    Err(error) => println!("error: {}", error),
}
}
