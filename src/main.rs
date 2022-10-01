/*
The main file, containing the main function and the game loop.

STYLE GUIDE:
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

extern crate supports_unicode;

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
        let difficulty: u8;
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
    
    let mut randint = rand::thread_rng();

    let mut upcoming_events: Vec<enums::Event> = Vec::new();
    upcoming_events.push(Event::Supernova(randint.gen_range(uni.stardate+1.5..uni.stardate+9.0)));
    if uni.get_difficulty() > 1 {
        upcoming_events.extend_from_slice(events::gen_starbase_attack(&uni).as_ref())  // TODO: Make `when` a random value once done testing.
    }
    let mut last_time: f64;
    let mut did_something: bool = false;  // Determines whether enemies attack. Necessary since scans etc. are a no-cost action.

    loop {
        last_time = uni.stardate;
        match io::parse_args(input("\nCommad > ")) {
            CommandType::Abandon => {
                if get_yorn("Are you sure you want to abandon ship? ") {
                    prout!("");
                    
                    // TODO: Add the rest of the logic
                    uni.abandon_ship();
                }
            },
            CommandType::CallStarbase => uni.call(),
            CommandType::Capture => {},  // TODO add capturing Klingons
            CommandType::Cloak(yorn) => uni.cloak(yorn),
            CommandType::Commands => {},  // TODO add commands list printout
            CommandType::Computer => {},  // TODO add ship's computer
            CommandType::Damage => uni.damage.print_damage(),
            CommandType::DeathRay => {
                if uni.deathray() {
                    did_something = true;
                }
            },
            CommandType::Debug(req) => {
                if &req == "events" {
                    prout!("Event stack: {:#?}", &upcoming_events);
                }
                if &req == "damage" {
                    simulate_damage(&mut uni);
                }
                else {
                    prout!("ERROR: Improper usage.");
                    prout!("\nSyntax: DEBUG argument");
                    prout!("Supported arguments: events")
                }
            }
            CommandType::Destruct => uni.self_destruct(),
            CommandType::Dock => {},  // TODO add starbase docking
            CommandType::EmExit => {
                em_exit(uni);
                return Ok(())
            },
            CommandType::Error => continue,
            CommandType::Freeze(file) => freeze(file, &uni),
            CommandType::Help(what) => {},  // TODO add help messages
            CommandType::Impulse(mode, deltas) => {
                uni.move_it(true, mode, deltas);
                did_something = true;
            },
            CommandType::Thaw(file) => uni = thaw(file).unwrap(),  // TODO fix
            CommandType::LrScan => uni.lrscan(),
            CommandType::Mine => {},  // TODO add dilithium crystal mining
            CommandType::Move(a, d) => uni.move_it(false, a, d),
            CommandType::Orbit => {},  // TODO add planet orbiting
            CommandType::Phasers(mode, energy) => {
                uni.phasers(mode, energy as f64);
                did_something = true;
            },
            CommandType::PlanetReport => {},
            CommandType::Probe(yorn, mode, deltas) => {},
            CommandType::Quit => {
                prout!("\nGoodbye.\n");
                return Ok(())
            },
            CommandType::Report => {},  // TODO add status reports
            CommandType::Request(what) => {},  // TODO add requests? I don't remember what this is.
            CommandType::Rest(duration) => uni.rest(duration),
            CommandType::Score => uni.score.print_score(),
            CommandType::SensorScan => {},  // TODO add planet scan
            CommandType::Shields(mode, amount) => uni.shields(mode, amount),
            CommandType::Shuttle => {},  // TODO add shuttles
            CommandType::SrScan => uni.srscan(),
            CommandType::StarChart => uni.starchart(),
            CommandType::Torpedo(num, deltas) => uni.torpedo(num, deltas),
            CommandType::Transporter(qubit) => {},  // TODO add transporters
            CommandType::Warp(factor) => uni.change_warp(factor),
        }

        if did_something {  // The player has done a non-free action, so the Klingons get to shoot back
            for enemy in uni.get_quadrant().enemies() {
                let distance: usize = (uni.sloc/10).abs_diff(enemy.1 / 10) + (uni.sloc%10).abs_diff(enemy.1 % 10);
            }
        }

        if uni.klingons == 0 {
            prout!("\nThe last Klingon battlecruiser has been destroyed, and the invasion thwarted. Good job!");
            if uni.alive == false {
                prout!("Unfortunately, you got yourself killed along with it.");
            }
            break;
        }

        if uni.death_reason != DeathReason::None { break; }

        if uni.stardate != last_time {  // Don't bother checking if no time has elapsed
            let mut sub_event = false;
            let mut e = 0;
            while e < upcoming_events.len() {
                match upcoming_events[e] {
                    Event::None => {},
                    Event::StarbaseAttack(begin, end, loc) => {
                        println!("Processing starbase attack.");
                        if begin <= uni.stardate && uni.stardate <= end 
                            && uni.damage.radio <= 0.3 
                            && uni.quadrants[loc[0]][loc[1]].starbase_threatened() {  // I wish Rust would allow you to chain comparison operators.
                                prout!("\n[*Lt. UHURA*] Captain, we just received a distress call from the starbase in quadrant {} {}. The base is under attack by Klingons, and can only hold out until stardate {:.2}.", loc[0] + 1, loc[1]+1, end);
                                upcoming_events.remove(e.clone());
                                sub_event = true;
                        }
                    },
                    Event::StarbaseDestroy(t, loc) => {
                        if uni.stardate >= t {
                            if uni.quadrants[loc[0]][loc[1]].starbase_threatened() {
                                uni.kill_starbase(loc.clone());
                                if uni.damage.radio <= 0.3 {
                                    slow_prout(format!("[*Lt. UHURA*] Sir, an APB just came in from Starfleet. The Klingons have destroyed the starbase in quadrant {} {}. I'm sorry sir.", loc[0], loc[1]), SLOW, true);
                                }
                            }
                            upcoming_events.remove(e.clone());
                            sub_event = true;
                        }
                    },
                    Event::TractorBeam(t) => {
                        if uni.stardate >= t {
                            upcoming_events.remove(e.clone());
                            sub_event = true;
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

                            upcoming_events.remove(e.clone());
                            sub_event = true;
                            let t = uni.stardate;
                            upcoming_events.push(Event::Supernova(rand::thread_rng().gen_range(t+3.0..t+9.0)));
                        }
                    }
                }
                if !sub_event {
                    e += 1;
                }
                sub_event = false;  // reset sub_event for next iteration
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
            prout!("The Enterprise's experimental deathray has created a rift in spacetime, through which stream mind-boggling... *things* with too many tentacles and too little respect for the laws of physics.");
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
            prout!("You and your crew have mutated into strange abominations.");
            prout!("Mr. Spock alone is unaffected. As a result, he kills you and defects to the Romulans, leading them in a conquest of the galaxy.");
        },
        DeathReason::Borg => {
            prout!("The Enterprise has been assimilated by the Borg. While they find your cranial capacity sub-par, your advanced technology sparks their interest.");
            prout!("As a result, they decide to assimilate the rest of the Federation.");
            prout!("While the Federation puts up a good fight, it isn't enough.");
            prout!("If only this had occured a century later; perhaps then things would have turned out differently.");
        },
        DeathReason::EventHorizon => {
            prout!("The Enterprise has blundered into a black hole.");
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


fn simulate_damage (uni: &mut Universe) {
    prout!("AVAILABLE SYSTEMS:");
    prout!(" - SHIELDS");
    prout!(" - REACTORS");
    prout!(" - LIFE_SUPPORT");
    prout!(" - WARP_DRIVE");
    prout!(" - IMPULSE_DRIVE");
    prout!(" - PHASERS");
    prout!(" - TORPEDOES");
    prout!(" - TRACTORS");
    prout!(" - DEATHRAY");
    prout!(" - RADIO");
    prout!(" - TRANSPORTER");
    prout!(" - SHUTTLES");
    prout!(" - LRSENSORS");
    prout!(" - SRSENSORS");
    prout!(" - CLOAK");
    prout!(" - COMPUTER");
    let which = input("Which system? ").to_lowercase();
    let amount = match input("How much damage? ").parse::<f64>() {
        Ok(v) => v,
        Err(e) => {
            prout!("Error converting value: {}", e);
            return
        }
    };

    match which.as_str() {
        "shields" => uni.damage.shields += amount,
        "reactors" => uni.damage.reactors += amount,
        "life_support" => uni.damage.life_support += amount,
        "warp_drive" => uni.damage.warp_drive += amount,
        "impulse_drive" => uni.damage.impulse_drive += amount,
        "phasers" => uni.damage.phasers += amount,
        "torpedoes" => uni.damage.torpedoes += amount,
        "tractors" => uni.damage.tractors += amount,
        "deathray" => uni.damage.deathray += amount,
        "radio" => uni.damage.radio += amount,
        "transporter" => uni.damage.transporter += amount,
        "shuttles" => uni.damage.shuttles += amount,
        "lrsensors" => uni.damage.lrsensors += amount,
        "srsensors" => uni.damage.srsensors += amount,
        "cloak" => uni.damage.cloak += amount,
        "computer" => uni.damage.computer += amount,
        _ => {
            prout!("ERROR: Invalid option.");
        }
    }
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
