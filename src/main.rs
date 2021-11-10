/*
The main file, containing the main function and the game loop.

STYLE GUIDE:
The class representing the Enterprise is always referred to as `ent`.
The class representing the universe is always referred to as `uni`.
Opening curly brackets are always on the same line, e.g. `if x {`.
Always have a space between each section of a function declaration, e.g. `fn foobar <'a> (args) -> return {`.

Module directory:
Input.rs - various input functions and game freezing/thawing
constants.rs - global constants
structs.rs - the Enterprise and Universe structs, along with the structs they use
weapons.rs - various weapons
movement.rs - move the ship
finish.rs - various ending conditions
deathray.rs - logic for the experimental deathray
*/

mod structs;
mod io;
mod constants;
mod events;
mod finish;
mod movement;
mod damage;
mod scans;
mod weapons;
mod defense;

use io::{input, freeze, thaw, CommandType, em_exit, get_yorn, slow_prout};
use structs::Universe;
use finish::DeathReason;

use crate::constants::DEBUG;


fn main() {
    println!("\n=======================
--- SUPER STAR TREK ---
=======================\n");
    
    if input("Load from save file? (y/n) ").to_lowercase().starts_with("y") {
        match thaw() {
            Some(u) => {match mainloop(u) {
                Ok(_) => {},
                Err(error) => println!("Fatal error: {}", error)
            }},
            None => return
        }
    } else {
        let password = input("Password (used for self-destruct and save-file encryption: ");
        let mut difficulty: u8;
        loop {
            match input("Difficulty (1=easy, 2=normal, 3=hard, 4=emeritus): ").as_str().parse::<u8>() {
                Err(_) => {println!("Invalid difficulty."); continue},
                Ok(res) => {
                    if 0 < res && res < 5 {
                        difficulty = res;
                        break;
                    }
                }
            };
        }

        match mainloop(Universe::new(password, difficulty)) {
            Ok(_) => {},
            Err(e) => println!("Fatal error: {}", e)
        }
    }

    println!();
}


fn mainloop <'a> (mut uni: Universe) -> Result<(), &'static str> {
    //! The game's main execution loop
    
    loop {
        match io::parse_args(input("\nCommad > ")) {
            CommandType::Abandon => {
                if get_yorn("Are you sure you want to abandon ship? ") {
                    println!("");
                    
                    // TODO: Add the rest of the logic
                    uni.abandon_ship();
                }
            },
            CommandType::CallStarbase => {},
            CommandType::Capture => {},
            CommandType::Cloak(yorn) => uni.cloak(yorn),
            CommandType::Commands => {},
            CommandType::Computer => {},
            CommandType::Damage => uni.damage.print_damage(),
            CommandType::DeathRay => uni.deathray(),
            CommandType::Destruct => uni.self_destruct(),
            CommandType::Dock => {},
            CommandType::EmExit => {
                em_exit(uni);
                return Ok(())
            },
            CommandType::Error => continue,
            CommandType::Freeze(file) => freeze(&uni),
            CommandType::Help(what) => {},
            CommandType::Impulse(mode, deltas) => uni.move_it(true, mode, deltas),
            CommandType::Load(file) => {},
            CommandType::LrScan => uni.lrscan(),
            CommandType::Mine => {},
            CommandType::Move(a, d) => uni.move_it(false, a, d),
            CommandType::Orbit => {},
            CommandType::Phasers(mode, targets) => {},
            CommandType::PlanetReport => {},
            CommandType::Probe(yorn, mode, deltas) => {},
            CommandType::Quit => {
                println!("\nGoodbye.\n");
                return Ok(())
            },
            CommandType::Report => {},
            CommandType::Request(what) => {},
            CommandType::Rest(duration) => uni.rest(duration),
            CommandType::Score => uni.score.print_score(),
            CommandType::SensorScan => {},
            CommandType::Shields(mode, amount) => uni.shields(mode, amount),
            CommandType::Shuttle => {},
            CommandType::SrScan => uni.srscan(),
            CommandType::StarChart => uni.starchart(),
            CommandType::Torpedo(num, deltas) => uni.torpedo(num, deltas),
            CommandType::Transporter(qubit) => {},
            CommandType::Warp(factor) => uni.change_warp(factor),
        }

        if uni.klingons == 0 {
            println!("\nThe last Klingon battlecruiser has been destroyed, and the invasion thwarted. Good job!");
            if uni.alive == false {
                println!("Unfortunately, you got yourself killed along with it.");
            }
            break;
        }

        if uni.death_reason != DeathReason::None { break; }
    }


    println!("\n\nThe stardate is {:.2}", uni.stardate);
    match uni.death_reason {
        DeathReason::MaximumEntropy => {
            println!("The Enterprise has experienced a warp core breach, resulting in its complete destruction.");
            println!("With the Enterprise out of the way, the Klingons proceed to conquer the Federation.");
        },
        DeathReason::NegativeSpaceWedgie => {
            println!("The Enterprise's experimental deathray has created a rift in spacetime, through which stream mind-boggling... things with too many tentacles and too little respect for the laws of geometry and physics.");
            println!("Although most attempts to study them result in insanity, one thing is known for certain: whether through hostility or outright indifference, they see no problem with brutally killing any normal creatures they encounter.");
        },
        DeathReason::Kaboom => {
            println!("The Enterprise has been destroyed in honorable battle with enemy forces.");
            if uni.klingons > 0 {
                println!("With the Enterprise out of the way, the Klingons easily conquer the Federation.");
            } else {
                println!("Thankfully, it took the last invading Klingon ship with it. Your noble sacrifice has saved the Federation.");
            }
        },
        DeathReason::Tribble => {
            println!("The Enterprise has been invaded with an infestation of Tribbles.");
            println!("Despite your attempts to destroy the cute vermin, they eat through the Enterprise's supplies.");
            println!("The ship attempts to dock at the nearest starbase, but is denied access.");
            println!("You and your crew starve to death, leaving the Federation defenseless.");
        },
        DeathReason::Stranded => {
            println!("The Enterprise is unable to retrieve you before leaving the system.");
            println!("As a result, Spock (who had wisely stayed behind) takes command of the ship.");
            println!("He sides with the Romulans, and uses the Enterprise's immense firepower to help them conquer the galaxy.");
        },
        DeathReason::TimeUp => {
            println!("Your attempts to stop the invasion have failed.");
            if uni.klingons > 5 {
                println!("With no other options, the Federation onconditionally surrenders to the Klingon Empire.");
                println!("You and your crew are imprisoned as war criminals, and the Enterprise is repurposed as a garbage scow.");
            } else {
                println!("However, the terrible damage which you inflicted on their forces allows the Federation to negotiate a treaty which, while highly disadvantageous, leaves its sovereignty intact.");
            }
        },
        DeathReason::NoAir => {
            println!("The Enterprise's life support reserves have run out. As a result, you and your crew suffocate to death.");
            println!("Without the Enterprise to help stop the invasion, the Federation falls within days.");
            println!("After drifting through space for several centuries the derelict hulk is found by a Klingon research vessel.");
            println!("It is taken back to their home world and turned into a museum attraction.");
        },
        DeathReason::NoGas => {
            println!("The Enterprise has run out of fuel. As a result, the ship's systems cease to function, and you gradually suffocate to death.");
            println!("Without the Enterprise to help stop the invasion, the Federation falls within days.");
            println!("After drifting through space for several centuries the derelict hulk is found by a Klingon research vessel.");
            println!("It is taken back to their home world and turned into a museum attraction.");
        },
        DeathReason::Transformation => {
            println!("You and your crew have been mutated into strange abominations.");
            println!("Interestingly, Mr. Spock is unaffected. As a result, he kills you and sides with the Romulans, leading them in a conquest of the galaxy.");
        },
        DeathReason::Borg => {
            println!("The Enterprise has been assimilated by the Borg. While they find your cranial capacity sub-par, your advanced technology sparks their interest.");
            println!("As a result, they decide to assimilate the rest of the Federation.");
            println!("While the Federation puts up a good fight, it isn't enough; the Federation falls.");
            println!("If only this had occured a century later; perhaps then things would have turned out differently.");
        },
        DeathReason::EventHorizon => {
            println!("The Enterprise has been sucked into a black hole.");
            println!("It is crushed like a tin can, leaving the Federation defenseless.");
        },
        DeathReason::SelfDestruct => {
            println!("Rather than let your ship fall into the Klingons' hands, you have blown it up.");
            if uni.klingons > 0 {
                println!("Without you, the Federation soon falls to the Klingons.");
            } else {
                println!("The blast takes out the last invading ship, saving the Federation.");
                println!("You are remembered forever as the heroes who made the ultimate sacrifice.");
            }
        },
        DeathReason::GalaxyEdge => {
            println!("Your repeated attempts to cross the negative energy barrier surrounding the galaxy have destroyed the Enterprise.");
            println!("Your navigation is abominable.");
        },
        _ => {}
    }

    return Ok(())
}

mod tests {
    use crate::io::{parse_args, CommandType, ControlMode};

    #[test]
    fn test_parser () {
        assert_eq!(parse_args(String::from("quit")), CommandType::Quit);
        assert_eq!(parse_args(String::from("abandon")), CommandType::Abandon);

        assert_ne!(parse_args(String::from("ca")), CommandType::CallStarbase);
        assert_eq!(parse_args(String::from("call")), CommandType::CallStarbase);

        assert_eq!(parse_args(String::from("cl y")), CommandType::Cloak(String::from("y")));

        assert_eq!(parse_args(String::from("dea")), CommandType::Error);
        assert_eq!(parse_args(String::from("deathray")), CommandType::DeathRay);

        assert_eq!(parse_args(String::from("dest")), CommandType::Error);
        assert_eq!(parse_args(String::from("destruct")), CommandType::Destruct);

        println!("{:?}", parse_args(String::from("shields u")));
        println!("{:?}", parse_args(String::from("shield tra -110.45")));

        assert_eq!(parse_args(String::from("probe arm auto 1 1")), CommandType::Probe(true, ControlMode::Auto, vec![1, 1]));
    }

    #[test]
    fn test_scans () {
        let mut uni = crate::Universe::new(String::from("asdf"), 1u8);
        uni.srscan();
        uni.lrscan();
        uni.starchart();
    }
}