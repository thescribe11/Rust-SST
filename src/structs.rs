//! Contains all the various data structs used by the program.
use rand::{Rng, random};
use serde::{Serialize, Deserialize};
use termion::color::{Blue, Fg, Green, Red, Reset};
use crate::{io::{ControlMode, input, get_args}, constants::{DEBUG, ALGERON}, finish::DeathReason, slow_prout};

use crate::damage::Damage;


/// The main data struct. It encapsulates everything else.
#[derive(Debug, Serialize, Deserialize)]
pub struct Universe {
    pub klingons: u32,
    pub score: Score,
    pub starbases: u32,
    pub ididit: bool,
    pub alive: bool,
    pub death_reason: DeathReason,
    pub leave_attempts: u8,

    pub stardate: f64,
    pub time_remaining: f64,

    pub quadrants: [[Quadrant; 8]; 8],
    charted: [[bool; 8]; 8],
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
    pub cloaked: bool,
}

impl Universe {
    pub fn new (password: String, difficulty: u8) -> Universe {
        let mut randint = rand::thread_rng();
        let starbases: u32 = randint.gen_range(3..8);

        let mut to_return = Universe {
            klingons: 0,  // Updated later by polling quadrants
            score: Score::new(),
            starbases,
            ididit: false,
            alive: true,
            death_reason: DeathReason::None,
            leave_attempts: 0,

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
            cloaked: false,
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

    pub fn get_starbases (&self) -> u32 {
        self.starbases
    }

    pub fn kill_starbase (&mut self) {
        self.starbases -= 1;
    }

    pub fn is_quadrant_accessible (&self, vert: usize, horiz: usize) -> bool {
        self.quadrants[vert][horiz].is_supernova.clone()
    }

    pub fn sector (&self, loc: &usize) -> u8 {
        self.quadrants[self.qvert][self.qhoriz].sectors[*loc]
    }

    pub fn get_quadrant (&self) ->  Quadrant {
        self.quadrants[self.qvert][self.qhoriz].clone()
    }

    pub fn get_other_quadrant (&self, qvert: &usize, qhoriz: &usize) -> Quadrant {
        self.quadrants[*qvert][*qhoriz].clone()
    }

    pub fn die (&mut self, reason: DeathReason) {
        //! The ship has been destroyed. Game over.

        self.alive = false;
        self.death_reason = reason;
        self.score.lose_ship();
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

        println!("    1 2 3 4 5 6 7 8 9 10");
        println!("  ┏━━━━━━━━━━━━━━━━━━━━━┓");

        if self.damage.srsensors == 0.0 { // Chart quadrant, but only if the short-range sensors are undamaged.
            self.charted[self.qvert][self.qhoriz] = true;
        }

        let mut viewable_coords: Vec<usize> = vec![self.sloc, self.sloc+10];
        if self.damage.srsensors > 0.0 {  // Limit the the player's vision to the Enterprise's immediate vicinity
            if self.sloc / 10 != 0 {
                viewable_coords.push(self.sloc-10);
                if self.sloc % 10 != 0 {
                    viewable_coords.extend_from_slice(&[self.sloc-11, self.sloc-1, self.sloc+9]);
                }
                if self.sloc % 10 != 9 {
                    viewable_coords.extend_from_slice(&[self.sloc-9, self.sloc+1, self.sloc+11]);
                }
            }
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

                if self.damage.srsensors > 0.0 && !viewable_coords.contains(&index) {
                    printing = false;
                }

                if printing {
                    print!(" {}", match self.sector(&index) {
                        0 => String::from("."),
                        1 => String::from("*"),
                        2 => format!("{}B{}", Fg(Blue), Fg(Reset)),
                        3 => format!("{}K{}", Fg(Red), Fg(Reset)),
                        4 => format!("{}R{}", Fg(Green), Fg(Reset)),
                        5 => String::from(" "),
                        6 => format!("{}t{}", Fg(Red), Fg(Reset)),
                        7 => format!("{}?{}", Fg(Green), Fg(Reset)),
                        8 => format!("{}E{}", Fg(Blue), Fg(Reset)),
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
                1 => println!(" Condition:     {}", match self.alert_level {
                    Alert::Red => "RED",
                    Alert::Yellow => "Yellow",
                    Alert::Green => "Green",
                }),
                2 => println!(" Position:      Sector {} {} of quadrant {} {}", self.sloc/10+1, self.sloc%10+1, &self.qvert+1, &self.qhoriz+1),
                3 => println!(" Life Reserves: {}; reserves: {:.2} days", 
                    match self.on_life_reserve {
                        false => String::from("Active"),
                        true => format!("{}OFFLINE{}", Fg(Red), Fg(Reset)),
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

        if self.damage.shuttles == 0.0 {
            println!("You and your core crew escape in the Enterprise's shuttles, and eventually make your way to a mothballed ship - the Faerie Queen.");
            if self.damage.transporter == 0.0 {
                println!("The Enterprise's remaining complement beam down to the nearest planet, where they are quickly captured.");
            } else {
                println!("Unable to escape the ship, your remaining crewmembers are killed.");
            }
        }
    }

    pub fn lrscan (&mut self) {
        //! Perform a long-range sensor scan.
        //! It won't give you fine details about
        //! quadrants, but you can get some basic
        //! information.

        if self.damage.lrsensors > 0.0 {
            println!("[*Mr. Spock*] Sir, the long range sensors are inoperable due to damage.");
            return;
        }

        println!("Long-range sensor scan for quadrant {} {}:", self.qvert+1, self.qhoriz+1);

        for i in self.qvert as i32-1..=self.qvert as i32+1 {
            for j in self.qhoriz as i32-1..=self.qhoriz as i32 +1 {
                if i<0 || i>7 || j<0 || j>7 {  // Galactic border
                    print!("   -1");
                    continue; // Don't try to chart out-of-bounds areas.
                }
                
                else if self.quadrants[i as usize][j as usize].is_supernova {  // Supernova
                    print!(" 1000")
                } else {  // Regular quadrant
                    let (x, y, z) = self.quadrants[i as usize][j as usize].poll_lrscan();
                    print!("  {}{}{}", x, y, z);
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

    pub fn torpedo (&mut self, num: Option<u8>, deltas: Vec<u8>) {
        //! Fire torpedoes

        // Get # of torpedoes to fire
        let to_fire = match num {
            Some(i) => i,
            None => {
                let x = input("How many torpedoes would you like to fire? ");
                match x.parse::<u8>() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("Sir, I can't fire \"{}\" torpedoes.", x);
                        return;
                    }
                }
            }
        };

        // Check for invalid cases
        if to_fire > self.torpedoes {
            println!("[*Armory*] What do you think we are, the Bank of Ferenginar?");
            return;
        } else if to_fire == 0 {
            return;
        } else if to_fire > self.torpedoes {
            println!("[*Armory*] We can't fire that many sir; we only {} left.", &self.torpedoes);
            return
        } else if to_fire > 3 {
            println!("[*Armory*] Sir, we can only fire three at a time; any more and we would melt the tubes!")
        }

        let _d = match deltas.len() {  // Get a firing solution if there isn't already one.
            0 => {
                let mut _d: Vec<u8> = Vec::new();
                let mut index = 0;
                while index < to_fire as usize {
                    print!("Input direction for torpedo #{}: ", index+1);
                    match get_args(input("")) {
                        Some(v) => {
                            for i in v {
                                _d.push(i);
                            }
                            index += 1;
                        },
                        None => {
                            println!("[*Armory*] Sir, that doesn't make sense.");
                            return
                        }
                    }
                }
                _d
            },
            _ => deltas
        };

        // Parse firing solution
        let solution: Vec<i8> = _d.into_iter().map(|x| match x%8 {
            1 => 1,
            2 => -9,
            3 => -10,
            4 => -11,
            5 => -1,
            6 => 9,
            7 => 10,
            8 => 11,
            _ => panic!("_d shouldn't contain any value which isn't between 0 and 8 post-modulo!")
        }).collect::<Vec<i8>>();

        let mut torp_num = 0;
        // Fire torpedoes
        for delta in solution {
            torp_num += 1;
            let mut torp_loc: i8 = self.sloc as i8;
            print!("\nTrack for torpedo #{}: ", &torp_num);

            // Simulate torpedo
            loop {
                torp_loc += delta;
                if torp_loc < 0 || torp_loc > 99 {
                    println!("\nTorpedo misses.");
                    break;
                }

                // Process impact
                match self.get_quadrant().get_entity(torp_loc as usize) {
                    Some((t, loc, he, al)) => {
                        match t {
                            EntityType::BlackHole => {
                                println!("\n Torpedo swallowed by black hole.");
                            },
                            EntityType::Klingon => {  // Klingons are always destroyed by torpedoes, although I might want to change this later.
                                println!("\n ***Klingon at sector ({}, {}) destroyed.", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                                self.score.kill_klingon();
                                self.klingons -=1;
                            },
                            EntityType::Romulan => {
                                match random::<u8>() {
                                    0..=200 => {  // Romulan dies
                                        println!("\n ***Romulan at sector ({}, {}) destroyed.", (torp_loc/10)+1, (torp_loc%10)+1);
                                        self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                                        self.score.kill_romulan();
                                    },
                                    201..=255 => {
                                        self.quadrants[self.qvert][self.qhoriz].damage_entity(torp_loc as usize, 500);
                                        self.score.kill_romulan()
                                    }
                                }
                            },
                            EntityType::Star => {
                                println!("\n ***Torpedo impacts star at sector ({}, {}), causing it to go nova.", (torp_loc/10)+1, (torp_loc%10)+1);
                                for i in &[torp_loc-11, torp_loc-10, torp_loc-9, torp_loc-1, torp_loc, torp_loc+1, torp_loc+9, torp_loc+10, torp_loc+11] {
                                    if *i > -1 && *i < 100 {
                                        match self.get_quadrant().sectors[*i as usize] {
                                            0 => {},  // Empty space; do nothing.
                                            1 => self.score.kill_star(),
                                            2 => self.score.kill_starbase(),
                                            3 => self.score.kill_klingon(),
                                            4 => self.score.kill_romulan(),
                                            5 => continue,  // Novas don't do anything to black holes either.
                                            6 => self.score.kill_tholian(),
                                            7 => self.score.kill_unknown(),
                                            8 => {},
                                            _ => {
                                                println!("{}", self.get_quadrant().sectors[*i as usize]);
                                                panic!("Somehow a corrupted value has gotten into the sector map.")
                                            }
                                        }
                                        self.quadrants[self.qvert][self.qhoriz].kill_entity(&(*i as usize));
                                    }
                                }
                            }
                            EntityType::Starbase => {
                                println!("\n ***Friendly starbase at sector ({}, {}) destroyed. You murderer.", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.score.kill_starbase();
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                            },
                            EntityType::Unknown => {
                                println!("\n *** ??? at sector ({}, {}) destroyed.", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.score.kill_unknown();
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                            },
                            EntityType::Tholian => {
                                println!("\n ***Tholian at sector ({}, {}) destroyed. Good shot!", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.score.kill_tholian();
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                            },
                            EntityType::Planet => {
                                println!("\n ***Planet at sector ({}, {}) destroyed. You murderer.", (torp_loc/10)+1, (torp_loc%10)+1);
                            },
                        }
                        // The torpedo has, of course, blown up.
                        break;
                    }
                    None => print!(", ({}, {})", (torp_loc/10)+1, (torp_loc%10)+1)
                }
            }
        }
    }

    /// Set the universe's alert level.
    pub fn set_alert_level(&mut self, alert_level: Alert) {
        self.alert_level = alert_level;
    }

    /// Get a reference to the universe's alert level.
    pub fn alert_level(&self) -> &Alert {
        &self.alert_level
    }

    pub fn add_time (&mut self, diff: f64) {
        self.time_remaining -= diff;
        self.stardate += diff;
        self.damage.repair(diff);
    }


    /// Kill an enemy at the specified location
    pub fn kill_enemy (&mut self, qvert: &usize, qhoriz: &usize, loc: &usize) {
        let enemy = match self.quadrants[*qvert][*qhoriz].get_entity(loc.clone()) {
            Some(e) => e,
            None => return,
        };

        self.quadrants[*qvert][*qhoriz].kill_entity(&*loc);

        match enemy.0 {
            EntityType::Klingon => {
                self.score.kill_klingon();
                self.klingons -= 1;
                self.add_time(-1.5);
            },
            EntityType::Romulan => self.score.kill_romulan(),
            EntityType::Tholian => self.score.kill_tholian(),
            EntityType::Unknown => self.score.kill_unknown(),
            _ => {}
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quadrant {
    pub sectors: Vec<u8>,  // [u8; 100] would be more efficient, but it doesn't play well with Serde.
    entities: Vec<(EntityType, usize, Health, Alignment)>,
    is_supernova: bool,
    klingons: u8,
    starbases: u8,
    stars: u8,
    romulans: u8,
}

impl Quadrant {
    fn default () -> Quadrant {
        //! Creates a quadrant. A starbase, if present, is added later.

        let mut randint = rand::thread_rng();

        let mut to_return = Quadrant {
            sectors: {let mut x = Vec::new(); x.extend_from_slice(&[0u8; 100]); x},  // 1 = star, 2 = starbase, 3 = klingon, 4 = romulan, 5 = black hole, 6 = tholian, 7 = unknown entity, 8 = player's ship
            entities: Vec::new(),
            is_supernova: false,
            klingons: 0,
            starbases: 0,
            stars: 0,
            romulans: 0,
        };

        return to_return
    }

    fn init (&mut self, difficulty: u8) -> u32 {
        //! Initialize the quadrant

        let mut randint = rand::thread_rng();
        let mut klingons = 0;
        let mut romulans = 0;
        let stars = 0;

        // Difficulty level stuff
        let max_klingons: i32 = randint.gen_range(0..match difficulty {
            1 => 4,
            2 => 5,
            3 => 7,
            4 => 10,
            _ => panic!("This code should be unreachable.")
        }) - 1; // TODO: Change this to account for difficulty
        let max_romulans = randint.gen_range(-1..match difficulty {
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
                        Health::new(randint.gen_range(150..=300), true),
                        Alignment::Enemy
                    ));
                },
                6 => self.sectors[i] = if romulans < max_romulans {
                    romulans += 1;
                    self.entities.push((
                        EntityType::Romulan,
                        i,
                        Health::new(randint.gen_range(250..600), true),
                        Alignment::Enemy
                    ));
                    4
                } else {1},      // Romulan
                7..=8 => {
                    self.sectors[i] = 5;
                    self.entities.push((
                        EntityType::BlackHole,
                        i,
                        Health::new(i32::MAX, false),
                        Alignment::Neutral
                    ));
                },  // Black hole.
                9..=13 => if stars < 10 {
                    self.sectors[i] = 1;
                    self.stars += 1;
                    self.entities.push((
                        EntityType::Star,
                        i,
                        Health::new(i32::MAX, false),
                        Alignment::Neutral,
                    ))
                }, // Star
                _ => {}
            }  
        }

        self.klingons = klingons as u8;
        self.romulans = romulans as u8;

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
                   self.entities.push((EntityType::Starbase, location, Health::new(2500, false), Alignment::Neutral));
                   break
                },  // Sector unoccupied; add a starbase and return
               _ => continue  // Sector occupied; continue searching for an empty spot
           }
        }
    }

    /// Get vital statistics for long-range scanning
    fn poll_lrscan (&self) -> (u8, u8, u8) {
        (self.klingons, self.starbases, self.stars)
    }

    /// Get a specific enemy based on its location in the sector
    pub fn get_entity (&self, location: usize) -> Option<(EntityType, usize, Health, Alignment)> {
        for i in self.entities.clone() {
            if i.1 == location {
                return Some(i)
            }
        };

        return None
    }

    /// Get all enemies of that type
    pub fn search (&self, entity: EntityType) -> Vec<(EntityType, usize, Health, Alignment)> {
        let mut to_return: Vec<(EntityType, usize, Health, Alignment)> = Vec::new();

        for i in self.entities.clone() {
            if i.0 == entity {
                to_return.push(i);
            }
        }

        return to_return
    }

    /// Kill an enemy
    pub fn kill_entity (&mut self, location: &usize) {
        for e in 0..self.entities.len() {
            if &self.entities[e].1 == location {
                

                self.entities.remove(e);
                break;
            }
        }
        self.sectors[*location] = 0;
    }

    /// Apply damage to an enemy.
    pub fn damage_entity (&mut self, location: usize, amount: i32) -> Option<bool> {
        for e in 0..self.entities.len() {
            if self.entities[e].1 == location {
                self.entities[e].2.amount -= amount
            }
        }

        return None
    }

    /// Get one of the quadrant's sectors
    pub fn sector(&self, location: &usize) -> u8 {
        self.sectors[*location].clone()
    }

    /// Determine whether the quadrant is part of the Romulan Neutral Zone
    pub fn neutral_zone (&self) -> bool {
        match self.romulans {
            0 => false,
            _ => {
                if self.klingons == 0 {
                    true
                } else {
                    false
                }  
            }
        }
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
    others_killed: i32,
    ididit: bool,  // Whether or not you've been caught violating the Treaty of Algeron
    alive: bool,
    ships_lost: i32,
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
            others_killed: 0,
            ididit: false,
            alive: true,
            ships_lost: 0,
        }
    }

    pub fn get_score (&self) -> i32 {
        return self.klingons_killed * 150
            +
            self.romulans_killed * 200
            +
            self.tholians_killed * 300
            +
            self.others_killed * 50
            -
            self.bases_killed * 500
            -
            self.stars_killed * 10
            -
            match self.ididit {
                true => 100,
                false => 0,
            }
            -
            match self.alive {
                true => 0,
                false => 200,
            }
            -
            self.ships_lost * 400  // Losing a ship is a big dill.
    }

    pub fn print_score (&self) {
        if self.klingons_killed > 0 {
            println!("{} Klingons killed:             +{}", &self.klingons_killed, self.klingons_killed * 150);
        }
        if self.romulans_killed > 0 {
            println!("{} Romulans killed:             +{}", &self.romulans_killed, self.romulans_killed * 200);
        }
        if self.tholians_killed > 0 {
            println!("{} Tholians killed:             +{}", &self.tholians_killed, self.tholians_killed * 300);
        }
        if self.others_killed > 0 {
            println!("{} unknown entities killed:     +{}", &self.others_killed, self.others_killed * 50);
        }
        if self.bases_killed > 0 {
            println!("{} starbases destroyed:         -{}", &self.bases_killed, self.bases_killed * 500);
        }
        if self.stars_killed > 0 {
            println!("{} stars blown up:              -{}", &self.stars_killed, self.stars_killed * 10);
        }
        if self.ididit {
            println!("Caught using a cloaking device: -100");
        }
        if self.ships_lost > 0 {
            println!("{} ships lost:                  -{}", &self.ships_lost, self.ships_lost * 400);
        }
        if !self.alive {
            println!("Penalty for getting killed:     -200")
        }

        println!("\nTOTAL SCORE: {}", self.get_score());
    }

    pub fn kill_klingon (&mut self) {
        self.klingons_killed += 1;
    }

    pub fn kill_romulan (&mut self) {
        self.romulans_killed += 1;
    }

    pub fn kill_tholian (&mut self) {
        self.tholians_killed += 1;
    }

    pub fn kill_starbase (&mut self) {
        self.bases_killed += 1;
    }

    pub fn kill_star (&mut self) {
        self.stars_killed += 1;
    }

    pub fn kill_planet (&mut self) {
        self.planets_killed += 1;
    }

    pub fn kill_unknown (&mut self) {
        self.others_killed += 1;
    }

    pub fn doit (&mut self) {
        self.ididit = true;
    }

    pub fn die (&mut self) {
        self.alive = false;
    }

    pub fn lose_ship (&mut self) {
        self.ships_lost += 1;
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
    BlackHole,
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Health {
    pub amount: i32,
    pub is_enemy: bool,
}
impl Health {
    fn new(amount: i32, alignment: bool) -> Health {
        Health { amount, is_enemy: alignment }
    }
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