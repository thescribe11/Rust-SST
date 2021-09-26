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
mod Input;
mod constants;
mod events;

use Input::{input, freeze, thaw, CommandType, em_exit, get_yorn, slow_prout};
use structs::Universe;
use events::DeathReason;


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
            None => {eprintln!("\nSayonara, sucker"); return}
        }
    } else {
        let raw_player_name = input("Player name (in format <first> <last>): ");
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

        match mainloop(Universe::new(raw_player_name.split(' ').into_iter().map(|e| String::from(e)).collect(), password, difficulty)) {
            Ok(_) => {},
            Err(e) => println!("Fatal error: {}", e)
        }
    }
}


fn mainloop <'a> (mut uni: Universe) -> Result<(), &'static str> {
    //! The game's main execution loop
    
    loop {
        match Input::parse_args(input("\nCommad > ")) {
            CommandType::Abandon => {
                if get_yorn("Are you sure you want to abandon ship? ") {
                    println!("");
                    
                    // TODO: Add the rest of the logic
                    uni.abandon_ship();
                }
            },
            CommandType::CallStarbase => {},
            CommandType::Capture => {},
            CommandType::Cloak(yorn) => {},
            CommandType::Commands => {},
            CommandType::Computer => {},
            CommandType::Damage => uni.damage.print_damage(),
            CommandType::DeathRay => {},
            CommandType::Destruct => {},
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
            CommandType::Move(mode, deltas) => uni.move_it(false, mode, deltas),
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
            CommandType::Rest(duration) => {},
            CommandType::Score => uni.score.print_score(),
            CommandType::SensorScan => {},
            CommandType::Shields(m, amount) => {},
            CommandType::Shuttle => {},
            CommandType::SrScan => uni.srscan(),
            CommandType::StarChart => uni.starchart(),
            CommandType::Torpedo(num, deltas) => uni.torpedo(num, deltas),
            CommandType::Transporter(qubit) => {},
            CommandType::Warp(factor) => {}
        }

        if uni.klingons == 0 {
            println!("\nThe last Klingon battlecruiser has been destroyed, and the invasion thwarted. Good job!");
            break;
        }
    }

    return Ok(())
}

mod tests {
    use crate::Input::{parse_args, CommandType, ControlMode};

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

        assert_eq!(parse_args(String::from("move a 1 1")), CommandType::Move(ControlMode::Auto, vec![1, 1]));
        assert_eq!(parse_args(String::from("im a 1 1")), CommandType::Impulse(ControlMode::Auto, vec![1, 1]));

        assert_eq!(parse_args(String::from("probe arm auto 1 1")), CommandType::Probe(true, ControlMode::Auto, vec![1, 1]));
    }

    #[test]
    fn test_scans () {
        let mut uni = crate::Universe::new(String::from("James Kirk").split(' ').into_iter().map(|e| String::from(e)).collect(), String::from("asdf"), 1u8);
        uni.srscan();
        uni.lrscan();
        uni.starchart();
    }
}