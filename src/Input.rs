use crate::constants::{DEBUG};
use crate::structs::{Enterprise, Universe};

use std::fs::{File};
use std::io::{Read, Write, stdin, stdout};

use serde_json::{to_string, from_str};


pub fn input(prompt: &str) -> String {
    //! A thin wrapper around std::io::stdin, meant to simulate Python's `input()` function

    let input = &mut String::new();

    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(input).unwrap();
    return input.trim_end().to_string();
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
            Err(_) => {println!("Unable to find save file.\n"); continue;}
        };
    }

    let pass = input("Password: ");
    let mut enc_data = String::new();
    match save_file.read_to_string(&mut enc_data) {
        Ok(_) => {},
        Err(_) => {println!("\nERROR: The save file is corrupted."); return None}
    }

    let raw_parts: Vec<String> = enc_data.split("\0x1e")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|element| String::from(element))
        .collect();
    
    if raw_parts[0] != pass {
        println!("That password is incorrect. Goodbye.");
        return None
    }
    
    ent = match from_str(raw_parts[1].as_str()) {
        Ok(data) => {data},
        Err(_) => {println!("\nERROR: The save file is corrupted."); return None}
    };
    uni = match from_str(raw_parts[2].as_str()) {
        Ok(data) => data,
        Err(_) => {println!("\nERROR: The save file is corrupted."); return None}
    };

    return Some((ent, uni))
}

pub fn freeze (ent: &Enterprise, uni: &Universe) {
    let mut file = match File::create(input("Filename: ")) {
        Ok(f) => f,
        Err(e) => {
            if DEBUG { println!("{}", e) }
            println!("Alas, it is impossible to create a file in that location.");
            return;
        }
    };

    match file.write_all((uni.password.clone() + "\0x1e" + to_string(ent).unwrap().as_str() + "\0x1e" + to_string(uni).unwrap().as_str()).as_bytes()) {
        Ok(_) => {},
        Err(_) => println!("I'm sorry, but that file cannot be written to.")
    }
        
}


pub fn parse_args (raw_input: String) -> (CommandType, Vec<String>){
    //! Parse input
    let tokens: Vec<String> = raw_input.split(' ').map(|s| s.to_lowercase()).collect();

    if tokens[0].starts_with("q") {}
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


#[derive(Clone, Copy)]
pub enum CommandType {
    Quit,
    Freeze,
    LrScan,
    SrScan,
    StarChart,
    Status,
    Damage,
    Move,
    Warp,
    Impulse,
    Shields,
    Phasers,
    Report,
    Computer,
    Torpedo,
    Dock,
    Rest,
    CallStarbase,
    Abandon,
    Destruct,
    SensorScan,
    Orbit,
    Transporter,
    Shuttle,
    Mine,
    Load,
    PlanetReport,
    Request,
    DeathRay,
    Probe,
    EmExit,
    Help,
    Cloak,
    Capture,
    Score,
    Commands,
    Error
}