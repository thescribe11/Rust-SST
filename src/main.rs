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

use Input::{input, freeze, thaw, CommandType, em_exit, get_yorn, prout};
use structs::{Enterprise, Universe};


fn main() {
    println!("\n============ SUPER STAR TREK ============\n");
    
    if input("Load from save file? (y/n) ").to_lowercase().starts_with("y") {
        match thaw() {
            Some((e, u)) => {match mainloop(e, u) {
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

        match mainloop(Enterprise::new(), Universe::new(raw_player_name.split(' ').into_iter().map(|e| String::from(e)).collect(), password)) {
            Ok(_) => {},
            Err(e) => println!("Fatal error: {}", e)
        }
    }
}


fn mainloop <'a> (mut ent: Enterprise, mut uni: Universe) -> Result<(), &'static str> {
    //! The game's main execution loop
    
    loop {
        match Input::parse_args(input("Commad > ")) {
            CommandType::Abandon => {
                if get_yorn("Are you sure you want to abandon ship? ") {
                    prout("")
                }
            },
            CommandType::CallStarbase => {},
            CommandType::Capture => {},
            CommandType::Cloak(yorn) => {},
            CommandType::Commands => {},
            CommandType::Computer => {},
            CommandType::Damage => {},
            CommandType::DeathRay => {},
            CommandType::Destruct => {},
            CommandType::Dock => {},
            CommandType::EmExit => {},
            CommandType::Error => continue,
            CommandType::Freeze(file) => {},
            CommandType::Help(what) => {},
            CommandType::Impulse(mode, deltas) => {},
            CommandType::Load(file) => {},
            CommandType::LrScan => {},
            CommandType::Mine => {},
            CommandType::Move(mode, deltas) => {},
            CommandType::Orbit => {},
            CommandType::Phasers(mode, targets) => {},
            CommandType::PlanetReport => {},
            CommandType::Probe(yorn, mode, deltas) => {},
            CommandType::Quit => {
                prout("\nGoodbye.\n");
                return Ok(())
            },
            CommandType::Report => {},
            CommandType::Request(what) => {},
            CommandType::Rest(duration) => {},
            CommandType::Score => {},
            CommandType::SensorScan => {},
            CommandType::Shields(m, amunt) => {},
            CommandType::Shuttle => {},
            CommandType::SrScan => {},
            CommandType::StarChart => {},
            CommandType::Torpedo(num, deltas) => {},
            CommandType::Transporter(qubit) => {},
            CommandType::Warp(factor) => {}
        }
    }
}