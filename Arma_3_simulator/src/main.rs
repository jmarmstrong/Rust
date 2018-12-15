extern crate dont_disappear;
extern crate read_input;
use read_input::*;
use std::io;
use std::io::prelude::*;
use std::{thread, time};

struct Player {
    name: String,
    money: u32,
    inventory: Vec<Item>,
}

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
    println!("You are in command of a squad of 5 and you have to make sure they live.");
    pause();
    println!("What is your surname?");
    let mut player = Player {
        name: simple_input(),
        money: 0,
        inventory: Vec::new(),
    };
    println!("Hello to the game of games, the simulators of simulators ");
    pause();
    println!("Welcome to The Arma 3 simulator");
    pause();
    println!("Are you Ready?");
    pause();
    println!("1) Yes");
    println!("2) No");

    match simple_input() {
        1 => println!("Ok well lets go!"),
        2 => println!("Well you chose to play this game so lets go!"),
        2004 => pass(&mut player),
        _ => unreachable!(),
    };

    player.inventory.push(Item::Ak47);
    println!("Hello commander {}", player.name);
    pause();
    println!("Your Squad mates are called: Mark, Jose, arthur, Dutch and John");
    pause();
    println!("crossroads: We are going to send your team on a recon mission.");
    pause();
    println!("Crossroads: You will be undercover as citizens so you will be unarmed  .");
    pause();
    println!("Crossroads: I will let you make the choices lets go.");
    pause();
    println!("Crossroads: How do you want to get there?");
    pause();
    println!("1) Tank");
    println!("2) Truck");
    println!("3) Jeep");

    match valid_input(|x| *x < 4 && *x > 0) {
        1 => {
            println!("Crossroad: Copy that we will send the Tank to you now. ");
            pause();
            println!("Crossroad: Drive as near the enemy base without being detectied");
            pause();
            println!("You go near the enemy base but the enemy hear the tank");
            pause();
            println!("They get out the Anti-Tank launcher and you are about to load the tank but your too slow and the tank is destroyed");
            pause();
            println!("Mission Failed");
            pause();
            println!("We'll get them next time.");
            dead();
        }
        2 => {
            println!("Crossroads: Copy that we wil send the Truck to you now.");
            pause();
            player.inventory.push(Item::Truck);
            println!("Crossroad: Drive as near the enemy base without being detectied");
            pause();
            println!("You go near the enemy base but you get spotted and they send someone to check the back of your truck");
            pause();
            println!("He saw your team and got suspicious");
            pause();
            println!("He asks why you have people in the back");
            pause();
            println!("How do you respond?");
            pause();
            println!("1) I'm doing recon of your base.");
            println!("2) ...");
            println!("3) Just bird watching with my friends");
            match valid_input(|x| *x < 4 && *x > 0) {
                1 => {
                    println!("He shoots you and the others.");
                    pause();
                    println!("Mission failed");
                    pause();
                    println!("Sometimes truth isn't the correct awnser");
                    dead();
                }

                2 => {
                    println!("He gets even more suspicious then your radio gose off");
                    pause();
                    println!(
                        "He hears them call you commander {} and shoots all of you",
                        player.name
                    );
                    println!("Mission Failed");
                    pause();
                    println!("Silence is not always the right awnser.");
                    dead();
                }

                3 => {
                    println!("He dosen't belive you at first but sees your banoculors");
                    pause();
                    println!("He leaves you alove");
                    pause();
                    println!("You head back to base");
                    pause();
                    println!("You have been given £100");
                    pause();
                    player.money = 100;
                    base(&mut player);
                }
                _ => unreachable!(),
            }
        }
        3 => {
            println!("Crossroads: Copy that we will send the Jeep to you now.");
            pause();
            player.inventory.push(Item::Jeep);
            println!("Crossroad: Drive as near the enemy base without being detectied.");
            pause();
            println!("You get detected but no one comes out.");
            pause();
            println!("You finish your recon and head back to base.");
            pause();
            println!("You have been given £100");
            pause();
            player.money = 100;
            base(&mut player);
        }
        _ => unreachable!(),
    }
}

fn pass(mut player: &mut Player) {
    player.money = 100;
    println!("1) The begining");
    println!("2) Base");
    match valid_input(|x| *x < 3 && *x > 0) {
        1 => main(),
        2 => base(&mut player),
        _ => unreachable!(),
    }
}

fn shop() {}

fn mission1(mut player: &mut Player) {}

fn dead() {
    println!("Do you want to play again?");
    pause();
    println!("1) Yes");
    println!("2) No");
    if simple_input::<u32>() == 1 {
        main()
    }
}

fn pause() {
    use thread;
    use time;
    sleep(Duration::from_secs(2));
}

fn pause2() {
    dont_disappear::any_key_to_continue::default();
}

fn base(mut player: &mut Player) {
    println!("Crossroads: Welcome {} to the base", player.name);
    pause();
    println!("Crossroads: Here you can either Start the next mission, goto the shop and look at your inventory. I will leave you to it");
    pause();
    println!("1) Missions");
    println!("2) Shop");
    println!("3) Inventory");

    match valid_input(|x| *x < 4 && *x > 0) {
        1 => {
            println!("Here you can play or repaly Missions.");
            pause();
            println!("1) The begining");
            println!("2) The Raid");
            match valid_input(|x| *x < 3 && *x > 0) {
                1 => {
                    println!("Are you sure you want to repaly?");
                    pause();
                    println!("1) Yes");
                    println!("2) No");
                }
                2 => {
                    println!("Are you ready to start the mission?");
                    println!("1) Yes");
                    println!("2) No");
                }
                _ => unreachable!(),
            }
        }
        2 => {
            println!("Crossroads: Welcome to the shop.");
            pause();
            println!("Crossroads: Here you can: Buy guns, vehicles and more.");
            pause();
            println!("Choose a category");
            pause();
            println!("1) Vehicles");
            println!("2) Guns");

            match simple_input() {
                1 => {
                    println!("Choose a type of vehicle.");
                }

                2 => {
                    println!("Choose a type of gun.");
                    pause();
                    println!("1) Pistols");
                    println!("2) Revolvers");
                    println!("3) Assult rifle");
                    println!("4) Shotgun");

                    match simple_input() {
                        1 => {
                            println!("{}", player.money);
                            println!("Pick a weapon");
                            pause();
                            println!("1) Desert eagle");
                            println!("2) Mauser C96");

                            match valid_input(|x| *x < 3 && *x > 0) {
                                1 => {
                                    if player.money < 200 {
                                        println!("You cannot buy this item.");
                                        pause2();
                                        base(&mut player)
                                    } else {
                                        player.inventory.push(Item::Deserteagle);
                                        println!("You have successfully bought a Desert eagle.");
                                        player.money = player.money - 200;
                                        pause2();
                                        base(&mut player)
                                    }
                                }
                                2 => {
                                    if player.money < 250 {
                                        println!("You cannot buy this item.");
                                        pause2();
                                        base(&mut player);
                                    } else {
                                        player.inventory.push(Item::MauserC96);
                                        println!("You have successfully bought a Desert eagle.");
                                        player.money = player.money - 250;
                                        pause2();
                                        base(&mut player)
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        2 => {
                            println!("{}", player.money);
                            println!("Pick a weapon");
                            println!("1) Colt Python");
                            println!("2) Colt Anaconder");
                            println!("3) Mateba Auto revolver");

                            match valid_input(|x| *x < 4 && *x > 0) {
                                1 => {
                                    if player.money < 100 {
                                        println!("You cannot buy this item.");
                                        pause2();
                                        base(&mut player);
                                    } else {
                                        player.inventory.push(Item::Coltpython);
                                        println!("You have successfully bought a Colt Python.");
                                        player.money = player.money - 100;
                                        pause2();
                                        base(&mut player);
                                    }
                                }
                                2 => {
                                    if player.money < 150 {
                                        println!("You cannot buy this item.");
                                        pause2();
                                        base(&mut player);
                                    } else {
                                        player.inventory.push(Item::Coltanaconda);
                                        println!("You have successfully bought a Colt Anaconda.");
                                        player.money = player.money - 150;
                                        pause2();
                                        base(&mut player);
                                    }
                                }
                                3 => {
                                    if player.money < 200 {
                                        println!("You cannot buy this item.");
                                        pause2();
                                        base(&mut player);
                                    } else {
                                        player.inventory.push(Item::MatebaAutorevolver);
                                        println!(
                                            "You have successfully boughT a Mateba Auto Revolver."
                                        );
                                        player.money = player.money - 200;
                                        pause2();
                                        base(&mut player);
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                3 => {
                    println!("{}", player.money);
                    println!("1) Ak-47");
                    println!("2) Vektor CR21");

                    match valid_input(|x| *x > 3 && *x < 0) {
                        1 => {
                            if player.money < 255 {
                                println!("You cannot buy this item.");
                                pause2();
                                base(&mut player);
                            } else {
                                player.inventory.push(Item::Ak47);
                                println!("You have successfully bought an AK-47.");
                                player.money = player.money - 255;
                                pause2();
                                base(&mut player);
                            }
                        }

                        2 => {
                            if player.money < 200 {
                                println!("You cannot buy this item.");
                                pause2();
                                base(&mut player);
                            } else {
                                player.inventory.push(Item::VektorCR21);
                                println!("You have successfully bought a Vektor CR21.");
                                player.money = player.money - 200;
                                pause2();
                                base(&mut player);
                            }
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
