use crate::constants::{DEBUG, COLUMNS};
use crate::structs::{Enterprise, Universe};

use std::fs::{File};
use std::io::{Read, Write, stdin, stdout};

use serde_json::{to_string, from_str};


pub fn is_numeric (s: &String) -> bool {
    // Check if a string is a number

    for c in s.chars() {
        if !c.is_numeric() {
            return false
        }
    }

    return true
}

fn convert_vec<U> (i: Vec<String>) -> Option<Vec<U>> where U: std::str::FromStr {
    //! Take a Vec<String> and turn it into Vec<U>.

    let to_return: Vec<U> = Vec::new();
    for item in i {
        if let Ok(val) = item.parse::<U>() {
            to_return.push(val)
        } else {
            return None
        }
    }

    Some(to_return)
}

pub fn convert_to_str (s: &Vec<String>) -> Vec<&str> {
    //! Convenience function to turn vecs of String into vecs of &str
    return s.iter().map(|x| x.as_str()).collect::<Vec<&str>>()
}


pub fn input(prompt: &str) -> String {
    //! A thin wrapper around std::io::stdin, meant to simulate Python's `input()` function

    let input = &mut String::new();

    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(input).unwrap();
    return input.trim_end().to_string();
}


pub fn prout (prompt: &str) {
    //! Print out text, wrapping at constants::COLUMNS
    //!
    //! Note that this function assumes `prompt` does
    //! not contain any weird multi-point characters. Output
    //! containing these characters may result in unexpected
    //! behavior.

    let mut column_index = 0;

    for c in prompt.chars() {
        print!("{}", c);
        column_index += 1;

        if column_index >= COLUMNS {
            print!("\n");
            column_index = 0;
        }
    }

    print!("\n");
}


pub fn thaw () -> Option<(Enterprise, Universe)>{
    //! Thaw a game.
    //!
    //! The .sst file type is as follows:
    //!
    //! (password)0x1e(json data for Enterprise object)\0x1e(json data for Universe object)

    let mut save_file: File;
    let ent: Enterprise; 
    let uni: Universe;

    println!("{:#?}", std::env::current_dir());

    loop {
        let temp = File::open(input("Save file: "));
        match temp {
            Ok(p) => {save_file = p; break;},
            Err(_) => {prout("Unable to find save file.\n"); continue;}
        };
    }

    let pass = input("Password: ");
    let mut enc_data = String::new();
    match save_file.read_to_string(&mut enc_data) {
        Ok(_) => {},
        Err(_) => {prout("\nERROR: The save file is corrupted."); return None}
    }

    let raw_parts: Vec<String> = enc_data.split("\0x1e")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|element| String::from(element))
        .collect();
    
    if raw_parts[0] != pass {
        prout("That password is incorrect. Goodbye.");
        return None
    }
    
    ent = match from_str(raw_parts[1].as_str()) {
        Ok(data) => {data},
        Err(_) => {prout("\nERROR: The save file is corrupted."); return None}
    };
    uni = match from_str(raw_parts[2].as_str()) {
        Ok(data) => data,
        Err(_) => {prout("\nERROR: The save file is corrupted."); return None}
    };

    return Some((ent, uni))
}

pub fn freeze (ent: &Enterprise, uni: &Universe) {
    let mut file = match File::create(input("Filename: ")) {
        Ok(f) => f,
        Err(e) => {
            if DEBUG { println!("{}", e) }
            prout("Alas, it is impossible to create a file in that location.");
            return;
        }
    };

    match file.write_all((uni.password.clone() + "\0x1e" + to_string(ent).unwrap().as_str() + "\0x1e" + to_string(uni).unwrap().as_str()).as_bytes()) {
        Ok(_) => {},
        Err(_) => prout("I'm sorry, but that file cannot be written to.")
    }
        
}


pub fn em_exit (ent: Enterprise, uni: Universe) {
    let mut file = match File::create("emsave.sst") {
        Ok(f) => f,
        Err(e) => {
            if DEBUG { println!("{}", e) }
            println!("ERROR: Unable to save.");
            return;
        }
    };

    match file.write_all((uni.password.clone() + "\0x1e" + to_string(&ent).unwrap().as_str() + "\0x1e" + to_string(&uni).unwrap().as_str()).as_bytes()) {
        Ok(_) => {},
        Err(_) => println!("ERROR: Unable to save.")
    }
}


pub fn parse_args (raw_input: String) -> CommandType<'static> {
    //! Parse input
    let tokens: Vec<String> = raw_input.split(' ').map(|s| s.to_lowercase()).collect();

    if tokens[0].len() == 0 {
        return CommandType::Error
    }

    if tokens[0].starts_with('a') && "abandon".contains(&tokens[0].as_str()) {
        return CommandType::Abandon
    }
    else if tokens[0].contains("call") {
        return CommandType::CallStarbase
    }
    else if tokens[0].starts_with("cl") && "cloak".contains(&tokens[0].as_str()) {
        match tokens.len() {
            1 => return CommandType::Cloak(""),
            2 => return CommandType::Cloak(stringify!(tokens[1])),
            _ => {
                prout("Uh... sir, have you been taking your pills lately?");
                return CommandType::Error
            }
        }
    }
    else if tokens[0].starts_with("comm") && "commands".contains(&tokens[0].as_str()) {
        return CommandType::Commands
    }
    else if tokens[0].starts_with("comp") && "computer".contains(&tokens[0].as_str()) {
        return CommandType::Computer
    }
    else if tokens[0].starts_with("da") && "damage".contains(&tokens[0].as_str()) {
        return CommandType::Damage
    }
    else if tokens[0].starts_with("dea") {
        match stringify!(tokens[0]) {
            "deathray" => return CommandType::DeathRay,
            _ => {
                prout("Due to its awesome power (and tendency to explode in your face), the \"deathray\" command cannot be abbreviated.")
            }
        }
    }
    else if tokens[0].starts_with("des") {
        match stringify!(tokens[0]) {
            "destruct" => return CommandType::Destruct,
            _ => {
                prout("I'm sorry, but to prevent accidents Starfleet doesn't allow this command to be abbreviated.");
            }
        }
    }
    else if tokens[0].starts_with("do") && "dock".contains(&tokens[0].as_str()) {
        return CommandType::Dock
    }
    else if tokens[0] == "emexit" {
        return CommandType::EmExit
    }
    else if tokens[0].starts_with("fr") && "freeze".contains(&tokens[0].as_str()) {
        match tokens.len() {
            1 => return CommandType::Freeze(""),
            2 => return CommandType::Freeze(stringify!(tokens[1])),
            _ => {
                prout("Huh?");
                return CommandType::Error
            }
        }
    }
    else if tokens[0].starts_with('h') && "help".contains(&tokens[0].as_str()) {
        match tokens.len() {
            1 => return CommandType::Help(""),
            2 => return CommandType::Help(stringify!(tokens[1])),
            _ => {
                prout("Hold yer horses! I can only give you help on one thing at a time.");
                return CommandType::Error
            }
        }
    }
    else if tokens[0].starts_with('i') && "impulse".contains(&tokens[0].as_str()) {
        match tokens.len() {
            1 => return CommandType::Impulse(ControlMode::Undefined, vec!()),
            2..=6 => {
                let mode = if tokens[1].starts_with('a') && "automatic".contains(&tokens[1]) {
                    ControlMode::Auto
                } else if tokens[1].starts_with('m') && "manual".contains(&tokens[1]) {
                    ControlMode::Manual
                } else {
                    println!("{} is not a valid movement type.", &tokens[1]);
                    return CommandType::Error
                };

                tokens.remove(0);
                return CommandType::Impulse(mode, tokens)
            }
        }
    }
    else if tokens[0] == "load" {
        return CommandType::Load(match tokens.len() {
            1 => {input("Enter save file name > ")},
            2 => tokens[1],
            _ => {
                prout("Invalid arguments.");
                return CommandType::Error
            }
        })
    }
    else if tokens[0].starts_with("lrs") && "lrscan".contains(&tokens[0]) {
        return CommandType::LrScan;
    }
    else if tokens[0].starts_with("mi") && "mine".contains(&tokens[0]) {
        return CommandType::Mine;
    }
    else if tokens[0].starts_with("mo") && "move".contains(&tokens[0]) {
        match tokens.len() {
            1 => return CommandType::Move(ControlMode::Undefined, vec!()),
            2..=6 => {let mode = if tokens[1].starts_with('a') && "automatic".contains(&tokens[1]) {
                    ControlMode::Auto
                } else if tokens[1].starts_with('m') && "manual".contains(&tokens[1]) {
                    ControlMode::Manual
                } else {
                    println!("{} is not a valid movement type.", &tokens[1]);
                    return CommandType::Error
                };

                tokens.remove(0);
                return CommandType::Move(mode, tokens)
            }
        }
    }
    else if tokens[0].starts_with("o") && "orbit".contains(&tokens[0]) {
        return CommandType::Orbit;
    }
    else if tokens[0].starts_with("ph") && "phasers".contains(&tokens[0]) {
        let mut total_energy: f32;

        let mut mode: ControlMode = match tokens[1] {
            a if a.starts_with('a') && "automatic".contains(&a) => {
                ControlMode::Auto
            },
            m if m.starts_with('m') && "manual".contains(&m) => {
                ControlMode::Manual
            },
            n if is_numeric(&n) => {
                ControlMode::Undefined
            },
            _ => {
                prout("Pull the other one; it's got bells on.");
                return CommandType::Error
            }
        };

        if mode == ControlMode::Undefined {
            mode = ControlMode::Auto;
            total_energy = match tokens[1].parse::<f32>() {
                Ok(i) => i,
                Err(_) => {
                    prout("Sir, that isn't a number.");
                    return CommandType::Error
                }
            };
        } else {
            tokens.remove(0);
            tokens.remove(0);
            let tokens: Vec<f32> = match convert_vec(tokens) {
                Some(v) => v,
                None => {
                    prout("Sir, that firing solution doesn't make sense!");
                    return CommandType::Error
                }
            };
            return CommandType::Phasers(mode, tokens)
        }
    }


    return CommandType::Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommandType<'a> {
    // Commands are sorted alphabetically for convenience.
    Abandon,
    CallStarbase,
    Capture,
    Cloak(&'a str),
    Commands,
    Computer,
    Damage,
    DeathRay,
    Destruct,
    Dock,
    EmExit,
    Error,
    Freeze(&'a str),
    Help(&'a str),
    Impulse(ControlMode, Vec<String>),
    Load(String),
    LrScan,
    Mine,
    Move(ControlMode, Vec<String>),
    Orbit,
    Phasers(ControlMode, Vec<f32>),
    PlanetReport,
    Probe(&'a str),
    Quit(&'a str),
    Report,
    Request,
    Rest(f32),
    Score,
    SensorScan,
    Shields(&'a str, f32),
    Shuttle,
    SrScan,
    StarChart,
    Status,
    Torpedo(u8, u8),
    Transporter(bool),
    Warp(i32)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ControlMode {
    Manual,
    Auto,
    Undefined,
}