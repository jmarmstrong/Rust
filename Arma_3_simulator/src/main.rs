extern crate read_input;
use read_input::*;
use std::io;
use std::io::prelude::*;
use std::{thread, time};
enum Item {
    Ak47,
    Deserteagle,
    Coltpython,
    Coltanaconda,
    MatebaAutorevolver,
    VektorCR21,
    MauserC96,
    Truck,
    Jeep,
}
fn main() {
    let mut money = 0;
    let mut inventory: Vec<Item> = Vec::new();
    inventory.push(Item::Ak47);
    println!("Hello to the game of games, the simulators of simulators ");
    pause();
    println!("Welcome to The Arma 3 simulator");
    println!("Are you Ready?");
    println!("1) Yes");
    println!("2) No");

    match simple_input() {
        1 => println!("Ok well lets go!"),
        2 => println!("Well you chose to play this game so lets go!"),
        2004 => pass(),
        _ => unreachable!(),
    };
    println!("You are in command of a squad of 5 and you have to make sure they live.");
    println!("What is your surname?");
    let name: String = simple_input();
    println!("Hello commander {}", name);
    println!("Your Squad mates are called: Mark, Jose, arthur, Dutch and John");
    println!("crossroads: We are going to send your team on a recon mission.");
    println!("Crossroads: You will be undercover as citizens so you will be unarmed  .");
    println!("Crossroads: I will let you make the choices lets go.");
    println!("Crossroads: How do you want to get there?");
    println!("1) Tank");
    println!("2) Truck");
    println!("3) Jeep");

    match valid_input(|x| *x < 4 && *x > 0) {
        1 => {
            println!("Crossroad: Copy that we will send the Tank to you now. ");
            println!("Crossroad: Drive as near the enemy base without being detectied");
            println!("You go near the enemy base but the enemy hear the tank");
            println!("They get out the Anti-Tank launcher and you are about to load the tank but your too slow and the tank is destroyed");
            println!("Mission Failed");
            println!("We'll get them next time.");
            dead()
        }
        2 => {
            println!("Crossroads: Copy that we wil send the Truck to you now.");
            inventory.push(Item::Truck);
            println!("Crossroad: Drive as near the enemy base without being detectied");
            println!("You go near the enemy base but you get spotted and they send someone to check the back of your truck");
            println!("He saw your team and got suspicious");
            println!("He asks why you have people in the back");
            println!("How do you respond?");
            println!("1) I'm doing recon of your base.");
            println!("2) ...");
            println!("3) Just bird watching with my friends");
            match valid_input(|x| *x < 4 && *x > 0) {
                1 => {
                    println!("He shoots you and the others.");
                    println!("Mission failed");
                    println!("Sometimes truth isn't the correct awnser");
                    dead()
                }

                2 => {
                    println!("He gets even more suspicious then your radio gose off");
                    println!(
                        "He hears them call you commander {} and shoots all of you",
                        name
                    );
                    println!("Mission Failed");
                    println!("Silence is not always the right awnser.");
                    dead()
                }

                3 => {
                    println!("He dosen't belive you at first but sees your banoculors");
                    println!("He leaves you alove");
                    println!("You head back to base");
                    println!("You have been given £100");
                    money = 100;
                    println!("Crossroads: Welcome to the base")
                }
                _ => unreachable!(),
            }
        }
        3 => {
            println!("Crossroads: Copy that we will send the Jeep to you now.");
            inventory.push(Item::Jeep);
            println!("Crossroad: Drive as near the enemy base without being detectied.");
            println!("You get detected but no one comes out.");
            println!("You finish your recon and head back to base.");
            println!("You have been given £100");
            money = 100;
            base()
        }
        _ => unreachable!(),
    }
}

fn inventory() {}

fn pass() {
    println!("1) The begining");
    match valid_input(|x| *x < 3 && *x > 0) {
        1 => main(),
        _ => unreachable!(),
    }
}

fn shop() {}

fn mission1() {}

fn dead() {
    println!("Do you want to play again?");
    println!("1) Yes");
    println!("2) No");
    if simple_input::<u32>() == 1 {
        main()
    }
}

fn pause() {
    thread::sleep(time::Duration::from_secs(2));
}

fn pause2() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn base() {
    println!("Crossroads: Welcome {} to the base", name);
    println!("Crossroads: Here you can either Start the next mission, goto the shop and look at your inventory. I will leave you to it");
    println!("1) Missions");
    println!("2) Shop");
    println!("3) Inventory");

    match valid_input(|x| *x < 4 && *x > 0) {
        1 => {
            println!("Here you can play or repaly Missions.");
            println!("1) The begining");
            println!("2) The Raid");
            match valid_input(|x| *x < 3 && *x > 0) {
                1 => {
                    println!("Are you sure you want to repaly.");
                    println!("1) Yes");
                    println!("2) No");
                }
                _ => unreachable!(),
            }
        }
        2 => {
            println!("Crossroads: Welcome to the shop.");
            println!("Crossroads: Here you can: Buy guns, vehicles and more.");
            println!("Choose a category");
            println!("1) Vehicles");
            println!("2) Guns");

            match simple_input() {
                1 => {
                    println!("sadfgfhghj");
                }

                2 => {
                    println!("Choose a type of gun.");
                    println!("1) Pistols");
                    println!("2) Revolvers");
                    println!("3) Assult rifle");
                    println!("4) Shotgun");

                    match simple_input() {
                        1 => {
                            println!("{}", money);
                            println!("Pick a weapon");
                            println!("1) Desert eagle");
                            println!("2) Mauser C96");
                        }
                        2 => {
                            println!("{}", money);
                            println!("Pick a weapon");
                            println!("1) Colt Python");
                            println!("2) Colt Anaconder");
                            println!("3) Mateba Auto revolver");

                            match valid_input(|x| *x < 4 && *x > 0) {
                                1 => {
                                    if money < 100 {
                                        println!("You cannot buy this item.");
                                        pause2();
                                        base();
                                        2
                                    } else {
                                        inventory.push(Item::Coltpython);
                                        println!("You have successfully bought a Colt Python.");
                                        pause2();
                                        base();
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        3 => {
                            println!("{}", money);
                            println!("Pick a weapon");
                            println!("1) Ak-47");
                            println!("2) Vektor CR21");
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        3 => println!("3) Inventory"),
        _ => unreachable!(),
    }
}
