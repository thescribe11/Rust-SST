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

// NOTE: This must be ran with the Nightly compiler.
#![feature(allow_internal_unstable)]

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
mod enums;

use io::{input, freeze, thaw, CommandType, em_exit, get_yorn, slow_prout, SLOW};
use rand::Rng;
use structs::Universe;
use finish::DeathReason;

use crate::{constants::DEBUG, enums::Event};


fn main() {
    prout!("\n=======================
--- SUPER STAR TREK ---
=======================\n");
    
    if input("Load from save file? (y/n) ").to_lowercase().starts_with("y") {
        match thaw(None) {
            Some(u) => {match mainloop(u) {
                Ok(_) => {},
                Err(error) => prout!("Fatal error: {}", error)
            }},
            None => return
        }
    } else {
        let password = input("Password (used for self-destruct and save-file encryption: ");
        let mut difficulty: u8;
        loop {
            match input("Difficulty (1=easy, 2=normal, 3=hard, 4=emeritus): ").as_str().parse::<u8>() {
                Err(_) => {prout!("Invalid difficulty."); continue},
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
            Err(e) => prout!("Fatal error: {}", e)
        }
    }

    prout!();
}


fn mainloop <'a> (mut uni: Universe) -> Result<(), &'static str> {
    //! The game's main execution loop
    
    let mut upcoming_events: Vec<enums::Event> = Vec::new();
    upcoming_events.push(Event::Supernova(rand::thread_rng().gen_range(1.5..9.0)));

    loop {
        match io::parse_args(input("\nCommad > ")) {
            CommandType::Abandon => {
                if get_yorn("Are you sure you want to abandon ship? ") {
                    prout!("");
                    
                    // TODO: Add the rest of the logic
                    uni.abandon_ship();
                }
            },
            CommandType::CallStarbase => uni.call(),
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
            CommandType::Load(file) => uni = thaw(file).unwrap(),  // TODO fix
            CommandType::LrScan => uni.lrscan(),
            CommandType::Mine => {},
            CommandType::Move(a, d) => uni.move_it(false, a, d),
            CommandType::Orbit => {},
            CommandType::Phasers(mode, targets) => {},
            CommandType::PlanetReport => {},
            CommandType::Probe(yorn, mode, deltas) => {},
            CommandType::Quit => {
                prout!("\nGoodbye.\n");
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
            prout!("\nThe last Klingon battlecruiser has been destroyed, and the invasion thwarted. Good job!");
            if uni.alive == false {
                prout!("Unfortunately, you got yourself killed along with it.");
            }
            break;
        }

        if uni.death_reason != DeathReason::None { break; }

        for e in upcoming_events.clone() {
            match e {
                Event::TractorBeam(t) => {
                    if uni.stardate >= t {
                        todo!()
                    }
                },
                Event::Supernova(t) => {
                    if uni.stardate >= t {
                        let mut randint = rand::thread_rng();
                        loop {
                            let v: usize = randint.gen_range(0..8);
                            let h: usize = randint.gen_range(0..8);

                            if !uni.get_other_quadrant(&v, &h).is_supernova {
                                uni.quadrants[v][h].is_supernova = true;
                                if (uni.qvert, uni.qhoriz) == (v, h) {
                                    uni.emergency_jump();
                                }
                                break;
                            }
                        }
                    }

                    match upcoming_events.iter().position(|x| *x==e) {
                        Some(x) => {upcoming_events.remove(x);},
                        None => {},
                    };

                    upcoming_events.push(Event::Supernova(rand::thread_rng().gen_range(1.5..9.0)));
                },
                Event::None => {},
            }
        }
    }

    prout!("\n\nThe stardate is {:.2}", uni.stardate);
    match uni.death_reason {
        DeathReason::Supernova => {
            prout!("The Enterprise has been caught in a supernova, destroying the ship and killing all aboard.");
            prout!("With the Enterprise out of the way, the Klingons proceed to conquer the Federation.");
        },
        DeathReason::MaximumEntropy => {
            prout!("The Enterprise has experienced a warp core breach, resulting in its complete destruction.");
            prout!("With the Enterprise out of the way, the Klingons proceed to conquer the Federation.");
        },
        DeathReason::NegativeSpaceWedgie => {
            prout!("The Enterprise's experimental deathray has created a rift in spacetime, through which stream mind-boggling... things with too many tentacles and too little respect for the laws of physics.");
            prout!("Although most attempts to study them result in insanity, one thing is known for certain: whether through hostility or outright indifference, they see no problem with brutally killing any \"normal\" creatures they encounter.");
        },
        DeathReason::Kaboom => {
            prout!("The Enterprise has been destroyed in honorable battle with enemy forces.");
            if uni.klingons > 0 {
                prout!("With the Enterprise out of the way, the Klingons easily conquer the Federation.");
            } else {
                prout!("Thankfully, it took the last invading Klingon ship with it. Your noble sacrifice has saved the Federation.");
            }
        },
        DeathReason::Tribble => {
            prout!("The Enterprise has been infested with Tribbles.");
            prout!("Despite your attempts to exterminate the cute vermin, they eat through the Enterprise's supplies.");
            prout!("You and your crew starve to death, leaving the Federation defenseless.");
        },
        DeathReason::Stranded => {
            prout!("The Enterprise is unable to retrieve you before leaving the system.");
            prout!("As a result, Spock (who had wisely stayed behind) takes command of the ship.");
            prout!("He defects to the Romulans, and uses the Enterprise's immense firepower to help them conquer the galaxy.");
        },
        DeathReason::TimeUp => {
            prout!("Your attempts to stop the invasion have failed.");
            if uni.klingons > 5 {
                prout!("With no other options, the Federation unconditionally surrenders to the Klingon Empire.");
                prout!("You and your crew are executed as war criminals, and the Enterprise is repurposed as a garbage scow.");
            } else {
                prout!("However, the terrible carnage you inflicted upon their forces allows the Federation to negotiate a treaty which, while highly disadvantageous, leaves its sovereignty intact.");
            }
        },
        DeathReason::NoAir => {
            prout!("The Enterprise's life support reserves have run out. As a result, you and your crew suffocate to death.");
            prout!("Without the Enterprise to help stop the invasion, the Federation falls within days.");
            prout!("After drifting through space for several centuries the derelict hulk is found by a Klingon research vessel.");
            prout!("It is taken back to their home world and turned into a museum attraction.");
        },
        DeathReason::NoGas => {
            prout!("The Enterprise has run out of fuel. As a result, the ship's systems cease to function, and you gradually suffocate to death.");
            prout!("Without the Enterprise to help stop the invasion, the Federation falls within days.");
            prout!("After drifting through space for several centuries the derelict hulk is found by a Klingon research vessel.");
            prout!("It is taken back to their home world and turned into a museum attraction.");
        },
        DeathReason::Transformation => {
            prout!("You and your crew have been mutated into strange abominations.");
            prout!("Mr. Spock alone is unaffected. As a result, he kills you and defects to the Romulans, leading them in a conquest of the galaxy.");
        },
        DeathReason::Borg => {
            prout!("The Enterprise has been assimilated by the Borg. While they find your cranial capacity sub-par, your advanced technology sparks their interest.");
            prout!("As a result, they decide to assimilate the rest of the Federation.");
            prout!("While the Federation puts up a good fight, it isn't enough.");
            prout!("If only this had occured a century later; perhaps then things would have turned out differently.");
        },
        DeathReason::EventHorizon => {
            prout!("The Enterprise has been sucked into a black hole.");
            prout!("It is crushed like a tin can, leaving the Federation defenseless.");
        },
        DeathReason::SelfDestruct => {
            prout!("Rather than let your ship fall into the Klingons' hands, you have blown it up.");
            if uni.klingons > 0 {
                prout!("Without you, the Federation soon falls to the Klingons.");
            } else {
                prout!("The blast takes out the last invading ship, saving the Federation.");
                prout!("You are remembered forever as the heroes who made the ultimate sacrifice.");
            }
        },
        DeathReason::GalaxyEdge => {
            slow_prout("Your repeated attempts to cross the negative energy barrier surrounding the galaxy have destroyed the Enterprise.", SLOW, true);
            slow_prout("Your navigation is abominable.", SLOW, true);
        },
        _ => {}
    }

    return Ok(())
}

#[allow(unused_imports)]
mod tests {
    use rand::thread_rng;

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

    #[test]
    fn test_randomness () {
        println!("{}", rand::Rng::gen_range(&mut thread_rng(), -5..5));
    }
}
