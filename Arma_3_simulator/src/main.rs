extern crate read_input;
use read_input::*;

fn main() {
    let mut money = 0;
    println!("Hello to the game of games, the simulators of simulators ");
    println!("Welcome to The Arma 3 simulator");
    println!("Are you Ready?");
    println!("1) Yes");
    println!("2) No");

    match valid_input(&|x| *x < 4 && *x > 0) {
        1 => println!("Ok well lets go!"),
        2 => println!("Well you chose to play this game so lets go!"),
        _ => panic!("You entered a unknown character"),
    };
    println!("You are in command of a squad of 5 and you have to make sure they live.");
    println!("What is your surname?");
    let name: String = simple_input();
    if name = "Armstrong" {
    println!("Whats your password?")
    let password: String = simple_input();}
    if password = "test" {
        bypass()
    }
    println!("Hello commander {}", name);
    println!("Your Squad mates are called: Mark, Jose, arthur, Dutch and John");
    println!("crossroads: We are going to send your team on a recon mission.");
    println!("Crossroads: You will be undercover as citizens so you will be unarmed  .");
    println!("Crossroads: I will let you make the choices lets go.");
    println!("Crossroads: How do you want to get there?");
    println!("1) Tank");
    println!("2) Truck");
    println!("3) Jeep");

    match valid_input(&|x| *x < 4 && *x > 0) {
        1 => {
            println!("Crossroad: Copy that we will send the Tank to you now. ");
            println!("Crossroad: Drive as near the enemy base without being detectied");
            println!("You go near the enemy base but the enemy hear the tank");
            println!("They get out the Anti-Tank launcher and you are about to load the tank but your too slow and the tank is destroyed");
            println!("Mission Failed");
            println!("We'll get them next time.");
            println!("Do you want to Play again");
            println!("1) Yes");
            println!("2) No");
            match valid_input(&|x| *x < 3 && *x > 0) {
                1 => main(),
                2 => (),
                _ => (),
            }
        }
        2 => {
            println!("Crossroads: Copy that we wil send the truck to you now.");
            println!("Crossroad: Drive as near the enemy base without being detectied");
            println!("You go near the enemy base but you get spotted and they send someone to check the back of your truck");
            println!("He saw your team and got suspicious");
            println!("He asks why you have people in the back");
            println!("How do you respond?");
            println!("1) I'm doing recon of your base.");
            println!("2) ...");
            println!("3) Just bird watching with my friends");
            match valid_input(&|x| *x < 4 && *x > 0) {
                1 => {
                    println!("He shoots you and the others.");
                    println!("Mission failed");
                    println!("Sometimes truth isn't the correct awnser");
                    println!("Do you want to play again?");
                    println!("1) Yes");
                    println!("2) No");
                    match valid_input(&|x| *x < 3 && *x > 0) {
                        1 => main(),
                        2 => (),
                        _ => (),
                    }
                }
                2 => {
                    println!("He gets even more suspicious then your radio gose off");
                    println!(
                        "He hears them call you commander {} and shoots all of you",
                        name
                    );
                    println!("Mission Failed");
                    println!("Silence is not always the right awnser.");
                    println!("Do you want to play again?");
                    println!("1) Yes");
                    println!("2) No");
                    match valid_input(&|x| *x < 3 && *x > 0) {
                        1 => main(),
                        2 => (),
                        _ => (),
                    }
                }
                3 => {
                    println!("He dosen't belive you at first but sees your banoculors");
                    println!("He leaves you alove");
                    println!("You head back to base camp");
                    println!("You have been given £100");
                    let mut money = 100;
                    base()
                }
                _ => panic!("You enterd a unknown character"),
            }
        }
        3 => {
            println!("Crossroads: Copy that we will send the Jeep to you now.");
            println!("Crossroad: Drive as near the enemy base without being detectied.");
            println!("You get detected but no one comes out.")
            println!("You finish your recon and head to base.")
        }
        _ => panic!("You enterd a unknown character"),
    }
}
fn base() {
    println!("Welcome to base camp");
    println!("Here you can: goto the shop, open your inventory and select missions");
    println!("I will leave you to it");
    println!("1) Missions");
    println!("2) Shop");
    println!("3) Inventory");
}
fn inventory() {}
    
fn bypass() {
    pritnln!("1) To begining")
    println!("2) Base 1")
    
    match valid_input(&|x| x < 3 && x > 0)
    1 => main(),
    2 => base(),
    _ => panic!("You enterd a unknown character"),
    }
