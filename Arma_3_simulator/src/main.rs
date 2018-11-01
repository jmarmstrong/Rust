use std::io;
extern crate read_input;
use read_input::*;
fn main() {
    let mut money = 0;
    println!("Hello to the game of games, the simulators of simulators ");
    println!("Welcome to The Arma 3 simulator");
    println!("Are you Ready?");
    println!("1) Yes");
    println!("2) No");

    match valid_input(&|x| *x < 3 && *x > 0) {
        1 => println!("Ok well lets go!"),
        2 => println!("Well you chose to play this game so lets go!"),
        _ => panic!("You entered a unknown character"),
    };
    println!("You are in command of a squad of 5 and you have to make sure they live.");
    println!("What is your surname");
    let name: String = simple_input();

    println!("Hello commander {}", name);
    println!("Your Squad mates are called: Mark, Jose, arthur, Dutch and John");
    println!("crossroads: We are going to send your team on a recon mission.");
    println!("Crossroads: You will be undercover as citizens.");
    println!("Crossroads: I will let you make the choices lets go.");
    println!("Crossroads: How do you want to get there?");

    match valid_input(&|x| *x < 4 && *x > 0) {
        1 => {
            println!("Crossroad: Copy that we will send the Tank to you now. ");
            println!("Crossroad: Drive as near the enemy base without being detectied");
            println!("You go near the enemy base but the enemy hear the tank");
            println!("They get out the Anti-Tank launcher and you are about to load the tank but your too slow and the tank is destroyed");
            println!("Mission Failed");
            println!("We'll get them next time.");
            println!("Do you want to Play again");
            match valid_input(&|x| *x < 3 && *x > 0) {
                1 => main(),
                _ => (),
            }
        }
        2 => {
            println!("Crossroads: Copy that we wil send the lorry to you now.");
            println!("Crossroad: Drive as near the enemy base without being detectied");
            println!("You go near the enemy base but you get spotted and they send someone to check the back of your lorry");
            println!("He saw your team and got suspicious");
            println!("He asks why you have people in the back");
        }
        3 => {
            println!("Crossroads: Copy that we will send the Jeep to you now");
        }
        _ => panic!("You enterd a unknown character"),
    }
}
