use crate::constants::{DEBUG, COLUMNS};
use crate::structs::{Universe};

use std::fs::File;
use std::io::{Read, Write, stdin, stdout};
use std::{thread, time};
use std::fmt::Debug;

use clearscreen::clear;
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


pub fn get_yorn (prompt: &str) -> bool {
    let i: String = match prompt {
        "" => input("Are you sure? (y/n) "),
        _ => input(prompt)
    }.to_lowercase();

    return abbrev(&i, "y", "yes")
}


pub fn abbrev (what: &String, least: &str, full: &str) -> bool {
    //! Check if `what` is an abbreviation of `full` and starts with `least`.

    return what.starts_with(&least) && full.contains(what)
}


pub fn strip<U: std::clone::Clone + std::fmt::Debug> (what: &mut Vec<U>, start: usize, end: usize) -> Option<Vec<U>> {
    //! Strip unnecessary elements from a Vec

    if (what.len() < end) || start > end {  // Ensure that the requested elements can be removed
        println!("Oops! You're trying to strip elements that aren't there.");
        return None
    }
    Some(what[start..end].to_vec())
}

fn convert_vec <T> (i: Vec<String>) -> Option<Vec<T>> 
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    //! Take a Vec<String> and turn it into Vec<U>.

    let mut to_return: Vec<T> = Vec::new();
    for item in i {
        match item.parse::<T>() {
            Ok(val) => to_return.push(val),
            Err(e) => {
                if DEBUG {
                    println!("{:#?}", e);
                }
                return None
            }
        }
    }

    Some(to_return)
}

pub fn get_args <T> (i: String) -> Option<Vec<T>> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    //! Wrapper around `convert_vec()`

    let raw_parts: Vec<String> = i.split(' ')    
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|element| String::from(element))
        .collect();

    convert_vec::<T>(raw_parts)
}

pub fn input(prompt: &str) -> String {
    //! A thin wrapper around std::io::stdin, meant to emulate Python's `input()` function

    let input = &mut String::new();

    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(input).unwrap();
    return input.trim_end().to_string();
}


pub fn slow_prout <T> (prompt: T) where T: ToString {
    let mut index = 0;
    for i in prompt.to_string().chars() {
        print!("{}", i);
        index += 1;

        if index >= COLUMNS {
            print!("\n");
            index = 0;
        }
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(20))
    }
    println!();
}


pub fn thaw () -> Option<Universe>{
    //! Thaw a game.
    //!
    //! The .sst file type is as follows:
    //!
    //! (password)0x1e(json data for Universe object)

    let mut save_file: File;
    let uni: Universe;

    loop {
        let temp = File::open(input("Save file: "));
        match temp {
            Ok(p) => {save_file = p; break;},
            Err(_) => {println!("Unable to find save file.\n"); continue;}
        };
    }

    let pass = input("Password: ");
    let mut enc_data = String::new();
    match save_file.read_to_string(&mut enc_data) {
        Ok(_) => {},
        Err(_) => {eprintln!("\nERROR: The save file is corrupted."); return None}
    }

    let raw_parts: Vec<String> = enc_data.split("\0")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|element| String::from(element))
        .collect();
    
    if raw_parts[0] != pass {
        eprintln!("That password is incorrect. Goodbye.");
        return None
    }
    
    uni = match from_str(raw_parts[1].as_str()) {
        Ok(data) => {data},
        Err(_) => {println!("\nERROR: The save file is corrupted."); return None}
    };
    
    return Some(uni)
}

pub fn freeze (uni: &Universe) {
    let mut file = match File::create(input("Filename: ")) {
        Ok(f) => f,
        Err(e) => {
            if DEBUG { println!("{}", e) }
            println!("Alas, it is impossible to create a file in that location.");
            return;
        }
    };

    match file.write_all((uni.password.clone() + "\0" + to_string(uni).unwrap().as_str()).as_bytes()) {
        Ok(_) => {},
        Err(_) => println!("I'm sorry, but that file cannot be written to.")
    }
        
}


pub fn em_exit (uni: Universe) {
    //! Emergency exit.
    //! 
    //! Saves the state, clears the screen, and then exits the program.

    let mut file = match File::create("emsave.sst") {
        Ok(f) => f,
        Err(e) => {
            if DEBUG { println!("{}", e) }
            println!("ERROR: Unable to save.");
            return;
        }
    };

    match file.write_all((uni.password.clone() + "\0" + to_string(&uni).unwrap().as_str()).as_bytes()) {
        Ok(_) => {},
        Err(_) => println!("ERROR: Unable to save.")
    }

    clear().unwrap();
}


pub fn parse_args <'a> (raw_input: String) -> CommandType {
    //! Parse input
    let mut tokens: Vec<String> = raw_input.split(' ').map(|s| s.to_lowercase()).collect();

    if tokens[0].len() == 0 {
        return CommandType::Error
    }

    if tokens[0] == "abandon" {
        return CommandType::Abandon
    }
    else if tokens[0] == "call" {
        return CommandType::CallStarbase
    }
    else if abbrev(&tokens[0], "ca", "capture") {
        return CommandType::Capture
    }
    else if abbrev(&tokens[0], "cl", "cloak") {
        match tokens.len() {
            1 => return CommandType::Cloak(String::from("")),
            2 => return CommandType::Cloak(tokens[1].clone()),
            _ => {
                println!("[*Engineering*] Uh... sir, have you been taking your pills lately?");
                return CommandType::Error
            }
        }
    }
    else if abbrev(&tokens[0], "comm", "commands") {
        return CommandType::Commands
    }
    else if abbrev(&tokens[0],"comp", "computer") {
        return CommandType::Computer
    }
    else if tokens[0].starts_with("da") && "damage".contains(&tokens[0].as_str()) {
        return CommandType::Damage
    }
    else if tokens[0].starts_with("dea") {
        match tokens[0].as_str() {
            "deathray" => return CommandType::DeathRay,
            _ => {
                println!("Due to its awesome power (and tendency to explode in your face), the \"deathray\" command cannot be abbreviated.");
                return CommandType::Error
            }
        }
    }
    else if tokens[0].starts_with("des") {
        match tokens[0].as_str() {
            "destruct" => return CommandType::Destruct,
            _ => {
                println!("[*COMPUTER*] I'm sorry, but to prevent accidents Starfleet doesn't allow this command to be abbreviated.");
            }
        }
    }
    else if abbrev(&tokens[0], "d","dock") {
        return CommandType::Dock
    }
    else if tokens[0].clone() == "emexit" {
        return CommandType::EmExit
    }
    else if abbrev(&tokens[0], "fr", "freeze") {
        match tokens.len() {
            1 => return CommandType::Freeze(String::new()),
            2 => return CommandType::Freeze(tokens[1].clone()),
            _ => {
                println!("Huh?");
                return CommandType::Error
            }
        }
    }
    else if abbrev(&tokens[0], "h", "help") {
        match tokens.len() {
            1 => return CommandType::Help(String::new()),
            2 => return CommandType::Help(tokens[1].clone()),
            _ => {
                println!("Hold yer horses! I can only give you help on one thing at a time.");
                return CommandType::Error
            }
        }
    }
    else if abbrev(&tokens[0], "i", "impulse") {
        match tokens.len() {
            1 => return CommandType::Impulse(None, None),
            2..=3 => {
                let angle = match tokens[1].parse::<f64>() {
                    Ok(a) => a,
                    Err(_) => {
                        println!(r#"[*Helm*] Sir, "second to the right, turn left after the sun and then straight on till morning" isn't a valid direction."#);
                        return CommandType::Error
                    }
                };

                return CommandType::Impulse(Some(angle), match tokens.len() {
                    2 => None,
                    3 => Some(match tokens[2].parse::<f64>() {
                        Ok(v) => v,
                        Err(_) => {
                            println!("[*Helm*] That isn't a distance.");
                            return CommandType::Error
                        }
                    }),
                    _ => panic!("This shouldn't be happening!")
                })
            },
            _ => {
                println!("[*Helm*] Sir, please say that again more slowly.");
                return CommandType::Error
            }
        }
    }
    else if tokens[0].clone() == "load" {
        return CommandType::Load(match tokens.len() {
            1 => {input("Enter save file name > ")},
            2 => tokens[1].clone(),
            _ => {
                println!("Invalid arguments.");
                return CommandType::Error
            }
        })
    }
    else if abbrev(&tokens[0],"lrs", "lrscan") {
        return CommandType::LrScan;
    }
    else if abbrev(&tokens[0], "mi", "mine") {
        return CommandType::Mine;
    }
    else if abbrev(&tokens[0], "m", "move") {
        match tokens.len() {
            1 => return CommandType::Move(None, None),
            2..=3 => {
                let angle = match tokens[1].parse::<f64>() {
                    Ok(a) => a,
                    Err(_) => {
                        println!(r#"[*Helm*] Sir, "second to the right, turn left after the sun and then straight on till morning" isn't a valid direction."#);
                        return CommandType::Error
                    }
                };

                let _x = tokens.len();
                return CommandType::Move(Some(angle), match tokens.len() {
                    2 => None,
                    3 => Some(match tokens[2].parse::<f64>() {
                        Ok(v) => v,
                        Err(_) => {
                            println!("[*Helm*] That isn't a distance.");
                            return CommandType::Error
                        }
                    }),
                    _ => panic!("This shouldn't be happening!")
                })
            },
            _ => {
                println!("[*Helm*] Sir, can you please say that again more slowly?");
                return CommandType::Error
            }
        }
    }
    else if abbrev(&tokens[0], "o", "orbit") {
        return CommandType::Orbit;
    }
    else if abbrev(&tokens[0], "ph", "phasers") {
        let mut total_energy: f32;

        let mut mode: ControlMode = match &tokens[1] {
            a if abbrev(&a, "a", "automatic") => {
                ControlMode::Auto
            },
            m if abbrev(&m, "m", "manual") => {
                ControlMode::Manual
            },
            n if is_numeric(&n) => {
                ControlMode::Undefined
            },
            _ => {
                println!("[*Fire Control*] Pull the other one; it's got bells on.");
                return CommandType::Error
            }
        };

        if mode == ControlMode::Undefined {
            mode = ControlMode::Auto;
            total_energy = match tokens[1].parse::<f32>() {
                Ok(i) => i,
                Err(_) => {
                    println!("[*Fire Control*] Sir, I can't fire non-numeric amounts of energy.");
                    return CommandType::Error
                }
            };
        } else {
            tokens.remove(0);
            tokens.remove(0);
            let tokens: Vec<f32> = match convert_vec(tokens) {
                Some(v) => v,
                None => {
                    println!("[*Fire Control*] Sir, that firing solution is invalid.");
                    return CommandType::Error
                }
            };
            return CommandType::Phasers(mode, tokens)
        }
    }
    else if abbrev(&tokens[0], "pl", "planets") {
        return CommandType::PlanetReport
    }
    else if abbrev(&tokens[0], "pr", "probe") {
        let mut armed: bool = false;
        let mode = match &tokens[1] {
            i if abbrev(i, "m", "manual") => ControlMode::Manual,
            i if abbrev(i, "a", "automatic") => ControlMode::Auto,
            i if abbrev(i, "ar", "armed") => {
                armed = true;
                tokens.remove(1);
                match &tokens[1] {
                    i if abbrev(i, "m", "manual") => ControlMode::Manual,
                    i if abbrev(i, "a", "automatic") => ControlMode::Auto,
                    _ => {
                        println!("[*Shuttle Bay*] Huh?");
                        return CommandType::Error
                    }
                }
            },
            _ => {
                println!("[*Shuttle Bay*] Huh?");
                return CommandType::Error
            }
        };
        let tokens: Vec<i32> = match convert_vec(match strip(&mut tokens.clone(), 2, tokens.len() + 0) {
            Some(t) => t.to_vec(),
            None => Vec::new()
            }) {
                Some(t) => t,
                None => {
                    println!("[*Shuttle Bay*] Those aren't valid destination coordinates.");
                    return CommandType::Error
                }
            };

        return CommandType::Probe(armed, mode, tokens)
    }
    else if tokens[0].clone() == "quit" {
        if DEBUG {
            return CommandType::Quit
        } else {
            if abbrev(&input("Are you sure you want to quit? "), "y", "yes") {
                return CommandType::Quit
            }
        }
            
    }
    else if abbrev(&tokens[0], "st", "status") {
        return CommandType::Report
    }
    else if abbrev(&tokens[0], "req", "request") {
        return CommandType::Request(tokens[1].clone())
    }
    else if abbrev(&tokens[0], "r", "rest") {
        match tokens.len() {
            1 => return CommandType::Rest(f32::NAN),
            2 => return CommandType::Rest(match tokens[1].parse::<f32>(){
                Ok(i) => i,
                Err(_) => {
                    println!("[*Mr. Spock*] Sir, that isn't a number.");
                    return CommandType::Error
                }
            }),
            _ => {
                println!("[*Mr. Spock*] Sir, that is illogical.");
                return CommandType::Error;
            }
        }
    }
    else if abbrev(&tokens[0], "sc", "score") {
        return CommandType::Score
    }
    else if abbrev(&tokens[0], "se", "sensors") {
        return CommandType::SensorScan
    }
    else if abbrev(&tokens[0], "s", "shields") {
        match tokens.len() {
            1 => return CommandType::Shields(String::new(), f32::NAN),
            _ => return CommandType::Shields(match tokens[1].clone() {
                u if "up".contains(&u) => "u".to_string(),
                d if "down".contains(&d) => "d".to_string(),
                t if "transfer".contains(&t) => "t".to_string(),
                _ => {
                    println!("[*Shield Control*] Say again, sir?");
                    return CommandType::Error
                }}, // End arg 1
                match tokens.len() {
                    2 => f32::NAN,
                    3 => match tokens[2].parse::<f32>() {
                        Ok(n) => n,
                        Err(_) => {
                            println!("[*Shield Control*] Sir, I can't make out what you're saying.");
                            return CommandType::Error
                        }
                    },
                    _ => {
                        println!("[*Shield Control*] What was that, sir?");
                        return CommandType::Error                        
                    }
                } // End arg 2
            )  // End return
        }
    }
    else if abbrev(&tokens[0], "shu", "shuttle") {
        return CommandType::Shuttle
    }
    else if abbrev(&tokens[0], "srs", "srscan") {
        return CommandType::SrScan
    }
    else if abbrev(&tokens[0], "ma", "map") || abbrev(&tokens[0], "sta", "starchart") || abbrev(&tokens[0], "ch", "chart") {
        return CommandType::StarChart
    }
    else if abbrev(&tokens[0], "t", "torpedoes") || abbrev(&tokens[0], "pho", "photons") {
        let mut to_fire: Option<u8> = None;
        let mut directions: Vec<u8> = Vec::new();
        
        match tokens.len() {
            1 => return CommandType::Torpedo(None, Vec::new()),
            2 => return CommandType::Torpedo(match tokens[1].parse::<u8>() {
                    Ok(i) => Some(i),
                    Err(_) => {
                        println!("[*Armory*] Huh?");
                        return CommandType::Error
                    }
                }, Vec::new()),
            _ => {
                to_fire = match tokens[1].parse::<u8>() {
                    Ok(i) => Some(i),
                    Err(_) => {
                        println!("*Armory*] Sir?");
                        return CommandType::Error;
                    }
                };
                tokens = match strip(&mut tokens.clone(), 2, tokens.len()+0) {
                    Some(t) => t.to_vec(),
                    None => Vec::new()
                };
                directions = match convert_vec(tokens) {
                    Some(t) => t,
                    None => {
                        println!("[*Armory*] Sir, that firing solution makes no sense!");
                        return  CommandType::Error
                    }
                }
            }
        }
        return CommandType::Torpedo(to_fire, directions)
    }
    else if abbrev(&tokens[0], "tr", "transporter") {
        return CommandType::Transporter(match tokens.len() {
            1 => 2,
            2 => match &tokens[2] {
                y if abbrev(y, "y", "yes") => 1,
                n if abbrev(n, "n", "no") => 0,
                _ => {
                    println!("[*Transporter Room*] I didn't quite catch that.");
                    return CommandType::Error
                }
            },
            _ => {
                println!("[*Transporter Room*] Um... would you mind saying that again sir?");
                return CommandType::Error
            }
        })
    }
    else if abbrev(&tokens[0], "w", "warp") {
        match tokens.len() {
            1 => return CommandType::Warp(f64::NEG_INFINITY),
            _ => return CommandType::Warp(match tokens[1].parse::<f64>() {
                Ok(i) => i,
                Err(_) => {
                    println!("[*Helm*] Sir, that isn't a valid warp factor.");
                    return CommandType::Error
                }
            })
        }
    }

    // At this point we can assume that it isn't a valid command
    println!("[*Mr. Spock*] Captain, that is not a valid command.");
    return CommandType::Error
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommandType {
    // Commands are sorted alphabetically for convenience.
    Abandon,
    CallStarbase,
    Capture,
    Cloak(String),
    Commands,
    Computer,
    Damage,
    DeathRay,
    Destruct,
    Dock,
    EmExit,
    Error,
    Freeze(String),
    Help(String),
    Impulse(Option<f64>, Option<f64>),
    Load(String),
    LrScan,
    Mine,
    Move(Option<f64>, Option<f64>),
    Orbit,
    Phasers(ControlMode, Vec<f32>),
    PlanetReport,
    Probe(bool, ControlMode, Vec<i32>),
    Quit,
    Report,
    Request(String),
    Rest(f32),
    Score,
    SensorScan,
    Shields(String, f32),
    Shuttle,
    SrScan,
    StarChart,
    Torpedo(Option<u8>, Vec<u8>),
    Transporter(u8),
    Warp(f64)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ControlMode {
    Manual,
    Auto,
    Undefined,
}