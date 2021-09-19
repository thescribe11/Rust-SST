//! Contains all the various data structs used by the program.
extern crate rand;
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::iter::FromIterator;
use crate::{Input::ControlMode, constants::{DEBUG, ALGERON}, prout, slow_prout};


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Damage {
    // Critical infrastructure
    reactors: u32,
    life_support: u32,
    warp_drive: u32,
    impulse_drive: u32,
    
    // Weapons
    phasers: u32,
    torpedoes: u32,
    tractors: u32,
    deathray: u32,

    // Accessories
    radio: u32,
    transporter: u32,
    shuttles: u32,
    lrsensors: u32,
    srsensors: u32,
}

impl Damage {
    fn new () -> Damage {
        Damage {
            reactors: 0,
            life_support: 0,
            warp_drive: 0,
            impulse_drive: 0,
            phasers: 0,
            torpedoes: 0,
            tractors: 0,
            deathray: 0,
            radio: 0,
            transporter: 0,
            shuttles: 0,
            lrsensors: 0,
            srsensors: 0,
        }
    }

    pub fn print_damage (&self) {
        println!(
"******** DAMAGE REPORT ********
* == Core systems ==
* Reactor Core:          {}
* Life support systems:  {}    
* Warp drive:            {}
* Impulse drive:         {}
*
* == Weapons ==
* Phasers:               {}
* Photon torpedoes:      {}
* Tractor beams:         {}
* Experimental Deathray: {}
*
* == Peripheral Systems ==
* Subspace Radio:        {}
* Transporters:          {}
* Shuttles:              {}
* Long-Range Sensors:    {}
* Short-Range Sensors:   {}", 
    self.reactors as f32 / 10f32, 
    self.life_support as f32 / 10f32, 
    self.warp_drive as f32 / 10f32, 
    self.impulse_drive as f32 / 10f32, 
    self.phasers as f32 / 10f32, 
    self.torpedoes as f32 / 10f32, 
    self.tractors as f32 / 10f32, 
    self.deathray as f32 / 10f32, 
    self.radio as f32 / 10f32, 
    self.transporter as f32 / 10f32, 
    self.shuttles as f32 / 10f32,
    self.lrsensors as f32 / 10f32,
    self.srsensors as f32 / 10f32
        )
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Universe {
    pub klingons: u32,
    pub score: Score,
    pub starbases: u32,
    pub ididit: bool,

    stardate: f64,
    time_remaining: f64,

    pub quadrants: [[Quadrant; 8]; 8],
    charted: [[bool; 8]; 8],
    
    pub player_name: Vec<String>,
    pub password: String,

    pub qvert: usize,
    pub qhoriz: usize,
    pub sloc: usize,

    // The Enterprise's stuff
    pub crew: u8,
    alert_level: Alert,
    pub energy: f64,
    pub deuterium: bool,
    pub life_reserves: f64,
    pub on_life_reserve: bool,
    pub docked: bool,
    pub torpedoes: u8,
    pub shields: f64,
    pub shield_status: bool,
    pub damage: Damage,

    pub warp_factor: f64,
}

impl Universe {
    pub fn new (player_name: Vec<String>, password: String, difficulty: u8) -> Universe {
        let mut randint = rand::thread_rng();
        let starbases: u32 = randint.gen_range(3..8);

        let mut to_return = Universe {
            klingons: 0,  // Updated later by polling quadrants
            score: Score::new(),
            starbases,
            ididit: false,

            stardate: (100.0f64*(31.0*rand::random::<f64>()+20.0)) as f64,
            time_remaining: 0.0,

            // Since the compiler won't let me .copy() the Vec in Universe...
            quadrants: [[Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],],
            charted: [[false; 8]; 8],

            player_name,
            password,
            
            qvert: randint.gen_range(0..8),
            qhoriz: randint.gen_range(0..8),
            sloc: randint.gen_range(0..64),

            crew: 100,
            energy: 3000f64,
            deuterium: false,
            life_reserves: 10.0,
            on_life_reserve: false,
            docked: false,
            torpedoes: 10,
            shields: 600f64,
            damage: Damage::new(),
            alert_level: Alert::Green,
            warp_factor: 4.0,
            shield_status: false,
        };

        to_return.quadrants[to_return.qvert][to_return.qhoriz].sectors[to_return.sloc] = 8;

        for i in 0..8 {
            for j in 0..8 {
                to_return.klingons += to_return.quadrants[i][j].init(difficulty);
            }
        }

        to_return.time_remaining = 14f64; // TODO: Implement an algorithm if I decide to have different game lengths

        let mut starbased: Vec<(usize, usize)> = Vec::new();
        let mut i: u32 = 0;
        while i < starbases {
            let (vert, horiz) = (randint.gen_range(0..8), randint.gen_range(0..8));

            match starbased.contains(&(vert, horiz)) {
                true => continue,
                false => {
                    to_return.quadrants[vert][horiz].add_starbase(); 
                    starbased.push((vert,horiz)); 
                    i+=1;
                }
            }
        }

        std::mem::drop(starbased);
        std::mem::drop(i);  // In case I want to use it later on

        return to_return
    }

    pub fn dididoit (&self) -> bool {
        self.ididit
    }

    pub fn doit (&mut self) {
        self.ididit = true;
    }

    pub fn get_klingons (&self) -> u32 {
        self.klingons
    }

    pub fn kill_klingons (&mut self, to_kill: i32) {
        if self.klingons as i32 - to_kill < 0 {
            self.klingons = 0;
        } else {
            self.klingons -= to_kill as u32
        }
    }

    pub fn get_starbases (&self) -> u32 {
        self.starbases
    }

    pub fn kill_starbase (&mut self) {
        self.starbases -= 1;
    }

    pub fn is_quadrant_accessible (&self, vert: usize, horiz: usize) -> bool {
        self.quadrants[vert][horiz].is_supernova.clone()
    }

    pub fn sector(&self, loc: &usize) -> u8 {
        self.quadrants[self.qvert][self.qhoriz].sectors[*loc]
    }

    pub fn get_quadrant(&self) -> Quadrant {
        self.quadrants[self.qvert][self.qhoriz].clone()
    }

    pub fn srscan (&mut self) {
        //! Perform a short-range sensor scan

        let quad = self.get_quadrant();
        if (quad.search(EntityType::Romulan).len() > 0)
        || quad.search(EntityType::Tholian).len() > 0 
        || quad.search(EntityType::Unknown).len() > 0
        || self.on_life_reserve {
            self.alert_level = Alert::Yellow;
        }
        if quad.search(EntityType::Klingon).len() > 0
        || quad.search(EntityType::Romulan).len() > 0 && self.ididit {
            self.alert_level = Alert::Red
        }

        prout("    1 2 3 4 5 6 7 8 9 10");
        prout("  ┏━━━━━━━━━━━━━━━━━━━━━┓");

        if self.damage.srsensors == 0 { // Chart quadrant, but only if the short-range sensors are undamaged.
            self.charted[self.qvert][self.qhoriz] = true;
        }

        let mut index = 0;
        for vert in 0..10 {
            print!("{}{}", vert+1, match vert {
                0..=8 => " ┃",
                _ => "┃",
            });

            let mut printing: bool;
            
            for _horiz in 0..10 {
                printing = true;

                if self.damage.srsensors > 0 {  // Limit the the player's vision to the Enterprise's immediate vicinity
                    if ![self.sloc-11, self.sloc-10, self.sloc-9, self.sloc-1, self.sloc, self.sloc+1, self.sloc+9, self.sloc+10, self.sloc+11].contains(&index) {
                        printing = false;
                    }
                } 

                if printing {
                    print!(" {}", match self.sector(&index) {
                        0 => '.',
                        1 => '*',
                        2 => 'B',
                        3 => 'K',
                        4 => 'R',
                        5 => ' ',
                        6 => 't',
                        7 => '?',
                        8 => 'E',
                        _ => panic!("It appears that the program has managed to put an impossible value in the sector srscan table. Please contact the developer with a bug report.")
                    });
                } else {
                    print!(" ?");
                }

                index += 1;
            }

            print!(" ┃");

            match vert {
                0 => println!(" Stardate:      {:.2}", self.stardate),
                1 => println!(" Condition:     {:?}", self.alert_level),
                2 => println!(" Position:      Sector {} {} of quadrant {} {}", self.sloc/10+1, self.sloc%10+1, &self.qvert+1, &self.qhoriz+1),
                3 => println!(" Life Reserves: {}; reserves: {:.2} days", 
                    match self.on_life_reserve {
                        false => "Active",
                        true => "OFFLINE"
                    }, self.life_reserves
                ),
                4 => println!(" Warp Factor:   {}", self.warp_factor),
                5 => println!(" Energy:        {:.2}", self.energy),
                6 => println!(" Torpedoes:     {}", self.torpedoes),
                7 => println!(" Shields:       {}, {} energy remaining", match self.shield_status {
                    true => "UP",
                    false => "DOWN",
                }, self.shields),
                8 => println!(" Klingons:      {}", &self.klingons),
                9 => println!(" Time Left:     {:.2}", self.time_remaining),
                _ => println!()
            }
        }
        println!("  ┗━━━━━━━━━━━━━━━━━━━━━┛");
    }

    pub fn abandon_ship (&mut self) {
        slow_prout("*AWHOOGAH*  *AWHOOGAH*");
        slow_prout("This is your captain speaking. We are abandoning ship. Please make your way to the nearest escape pod.");

        if self.damage.shuttles == 0 {
            prout("You and your core crew escape in the Enterprise's shuttles, and eventually make your way to a mothballed ship - the Faerie Queen.");
            if self.damage.transporter == 0 {
                prout("The Enterprise's remaining complement beam down to the nearest planet, where they are quickly captured.");
            } else {
                prout("Unable to escape the ship, your remaining crewmembers are killed.");
            }
        }
    }

    pub fn lrscan (&mut self) {
        //! Perform a long-range sensor scan.
        //! It won't give you fine details about
        //! quadrants, but you can get some basic
        //! information.

        if self.damage.lrsensors > 0 {
            prout("[*Mr. Spock*] Sir, the long range sensors are inoperable due to damage.");
            return;
        }

        println!("Long-range sensor scan for quadrant {} {}:", self.qvert+1, self.qhoriz+1);

        for i in self.qvert as i32-1..=self.qvert as i32+1 {
            for j in self.qhoriz as i32-1..=self.qhoriz as i32 +1 {
                if i<0 || i>7 || j<0 || j>7 {  // Galactic border
                    print!("  -1 ");
                    continue; // Don't try to chart out-of-bounds areas.
                }
                
                else if self.quadrants[i as usize][j as usize].is_supernova {  // Supernova
                    print!(" 1000")
                } else {  // Regular quadrant
                    let (x, y, z) = self.quadrants[i as usize][j as usize].poll_lrscan();
                    print!(" {}{}{}", x, y, z);
                }
                self.charted[i as usize][j as usize] = true;
            }
            println!();
        }
    }

    pub fn starchart (&self) {
        println!("     1   2   3   4   5   6   7   8");
        println!("  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");

        for vert in 0..8 {
            print!("{} ┃ ", &vert+1);

            for horiz in 0..8 {
                if self.charted[vert][horiz] {
                    let (k,b,s) = self.quadrants[vert][horiz].poll_lrscan();
                    print!("{}{}{} ", k, b, s);
                } else {
                    print!("??? ");
                }
            }

            println!("┃");
        }
        println!("  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quadrant {
    sectors: Vec<u8>,  // [u8; 100] would be more efficient, but it doesn't play well with Serde.
    entities: Vec<(EntityType, usize, Health, Alignment)>,
    is_supernova: bool,
    klingons: u8,
    starbases: u8,
    stars: u8,
}

impl Quadrant {
    fn default () -> Quadrant {
        //! Creates a quadrant. A starbase, if present, is added later.

        let mut randint = rand::thread_rng();

        let mut to_return = Quadrant {
            sectors: Vec::from_iter([0u8; 100]),  // 1 = star, 2 = starbase, 3 = klingon, 4 = romulan, 5 = black hole, 6 = tholian, 7 = unknown entity, 8 = player's ship
            entities: Vec::new(),
            is_supernova: false,
            klingons: 0,
            starbases: 0,
            stars: 0,
        };

        return to_return
    }

    fn init (&mut self, difficulty: u8) -> u32 {
        // Initialize the quadrant

        let mut randint = rand::thread_rng();
        let mut klingons = 0;
        let mut romulans = 0;

        // Difficulty level stuff
        let mut max_klingons: i32 = randint.gen_range(0..match difficulty {
            1 => 4,
            2 => 5,
            3 => 7,
            4 => 10,
            _ => panic!("This code should be unreachable.")
        }) - 1; // TODO: Change this to account for difficulty
        let mut max_romulans = randint.gen_range(-1..match difficulty {
            1..=2 => 2,
            3..=4 => 4,
            _ => panic!("This code should be unreachable.")
        });

        for i in 0..100 {
            if self.sectors[i] != 0 {
                continue;
            }
            match randint.gen_range(1..=100) {
                0..=5 => if klingons < max_klingons {  // Klingon
                    self.sectors[i] = 3; 
                    klingons += 1;
                    self.entities.push((
                        EntityType::Klingon,
                        i,
                        Health::Health(randint.gen_range(150..=300)),
                        Alignment::Enemy
                    ));
                },
                6 => self.sectors[i] = if romulans < max_romulans {
                    romulans += 1;
                    self.entities.push((
                        EntityType::Romulan,
                        i,
                        Health::Health(randint.gen_range(250..500)),
                        Alignment::Enemy
                    ));
                    4
                } else {1},      // Romulan
                7..=8 => self.sectors[i] = 5,  // Black hole.
                9..=13 => {self.sectors[i] = 1; self.stars += 1}, // Star
                _ => {}
            }  
        }

        self.klingons = klingons as u8;

        return klingons as u32
    }

    fn add_starbase(&mut self) {
        //! Add a starbase to the quadrant

        let mut randint = rand::thread_rng();

        loop {
           let location = randint.gen_range(0..100);
           
           match self.sectors[location] {
               0 => {
                   self.sectors[location] = 2;
                   self.entities.push((EntityType::Starbase, location, Health::Health(2500), Alignment::Neutral));
                   break
                },  // Sector unoccupied; add a starbase and return
               _ => continue  // Sector occupied; continue searching for an empty spot
           }
        }
    }

    fn poll_lrscan (&self) -> (u8, u8, u8) {
        (self.klingons, self.starbases, self.stars)
    }

    pub fn get_entity (&self, location: usize) -> Option<(EntityType, usize, Health, Alignment)> {
        for i in self.entities.clone() {
            if i.1 == location {
                return Some(i)
            }
        };

        return None
    }

    pub fn search (&self, entity: EntityType) -> Vec<(EntityType, usize, Health, Alignment)> {
        let mut to_return: Vec<(EntityType, usize, Health, Alignment)> = Vec::new();

        for i in self.entities.clone() {
            if i.0 == entity {
                to_return.push(i);
            }
        }

        return to_return
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    klingons_killed: i32,
    romulans_killed: i32,
    tholians_killed: i32,
    planets_killed: i32,
    bases_killed: i32,
    stars_killed: i32,
    ididit: bool,
    alive: bool,
}

impl Score {
    fn new () -> Score {
        Score {
            klingons_killed: 0,
            romulans_killed: 0,
            tholians_killed: 0,
            planets_killed: 0,
            bases_killed: 0,
            stars_killed: 0,
            ididit: false,
            alive: true,
        }
    }

    fn get_score (&self) -> i32 {
        return self.klingons_killed * 100
            +
            self.romulans_killed * 150
            +
            self.tholians_killed * 300
            -
            self.bases_killed * 500
            -
            self.stars_killed * 10
            -
            match self.ididit {
                true => 300,
                false => 0,
            }
            -
            match self.alive {
                true => 0,
                false => 100,
            }
    }
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntityType {
    Star,
    Starbase,
    Klingon,
    Romulan,
    Unknown,
    Tholian,
    Planet,
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Health {
    Health(u16),
    Inf
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Alignment {
    Neutral,
    Enemy
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
enum Alert {
    Green,
    Yellow,
    Red,
}