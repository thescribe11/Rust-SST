//! Contains all the various data structs used by the program.
use rand::{Rng, random};
use serde::{Serialize, Deserialize};
use termion::color::{Blue, Fg, Green, Red, Reset};
use crate::{constants::{DEBUG, ALGERON}, finish::DeathReason, io::{ControlMode, abbrev, get_args, get_yorn, input}, slow_prout};
use crate::prout;
use crate::damage::Damage;


/// The main data struct. It encapsulates everything else.
#[derive(Debug, Serialize, Deserialize)]
pub struct Universe {
    pub klingons: u32,
    pub score: Score,
    pub starbases: u32,
    pub ididit: bool,
    pub hit_me: bool,
    pub alive: bool,
    pub death_reason: DeathReason,
    pub leave_attempts: u8,

    pub stardate: f64,
    pub time_remaining: f64,

    pub quadrants: [[Quadrant; 8]; 8],
    pub charted: [[bool; 8]; 8],
    pub password: String,

    pub qvert: usize,
    pub qhoriz: usize,
    pub sloc: usize,

    // The Enterprise's stuff
    pub crew: u8,
    pub alert_level: Alert,
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
            hit_me: false,
            alive: true,
            death_reason: DeathReason::None,
            leave_attempts: 0,

            stardate: (100.0f64*(31.0*rand::random::<f64>()+20.0)) as f64,
            time_remaining: 0.0,

            quadrants: <[[Quadrant; 8]; 8]>::default(),
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

    pub fn doit (&mut self) {
        self.ididit = true;
        self.score.doit();
    }


    /// Get a list of the starbases' locations.
    pub fn get_starbases (&self) -> Vec<[usize; 2]> {
        let mut locs: Vec<[usize; 2]> = Vec::new();

        for vert in 0..8 {
            for horiz in 0..8 {
                if self.quadrants[vert][horiz].search(EntityType::Starbase).len() > 0 {
                    locs.push([vert, horiz]);
                }
            }
        }
        locs
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
        self.score.die();
    }

    pub fn abandon_ship (&mut self) {
        slow_prout("*AWHOOGAH*  *AWHOOGAH*");
        slow_prout("This is your captain speaking. We are abandoning ship. Please make your way to the nearest escape pod.");

        if self.damage.shuttles == 0.0 {
            prout!("You and your core crew escape in the Enterprise's shuttles, and eventually make your way to a mothballed ship - the Faerie Queen.");
            if self.damage.transporter == 0.0 {
                prout!("The Enterprise's remaining complement beam down to the nearest planet, where they are quickly captured.");
            } else {
                prout!("Unable to escape the ship, your remaining crewmembers are killed.");
            }
        }
    }

    /// Set the universe's alert level.
    pub fn set_alert(&mut self, alert_level: Alert) {
        self.alert_level = alert_level;
    }

    /// Get a reference to the universe's alert level.
    pub fn alert(&self) -> &Alert {
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
                self.time_remaining += 1.5;
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
    pub entities: Vec<(EntityType, usize, Health, Alignment)>,
    pub is_supernova: bool,
    klingons: u8,
    starbases: u8,
    stars: u8,
    romulans: u8,
}

impl Default for Quadrant {
    fn default () -> Quadrant {
        //! Creates a quadrant. A starbase, if present, is added later.
        let to_return = Quadrant {
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
}

impl Quadrant {
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

        self.starbases += 1;
    }

    /// Get vital statistics for long-range scanning
    pub fn poll_lrscan (&self) -> (u8, u8, u8) {
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
            prout!("{} Klingons killed:             +{}", &self.klingons_killed, self.klingons_killed * 150);
        }
        if self.romulans_killed > 0 {
            prout!("{} Romulans killed:             +{}", &self.romulans_killed, self.romulans_killed * 200);
        }
        if self.tholians_killed > 0 {
            prout!("{} Tholians killed:             +{}", &self.tholians_killed, self.tholians_killed * 300);
        }
        if self.others_killed > 0 {
            prout!("{} unknown entities killed:     +{}", &self.others_killed, self.others_killed * 50);
        }
        if self.bases_killed > 0 {
            prout!("{} starbases destroyed:         -{}", &self.bases_killed, self.bases_killed * 500);
        }
        if self.stars_killed > 0 {
            prout!("{} stars blown up:              -{}", &self.stars_killed, self.stars_killed * 10);
        }
        if self.ididit {
            prout!("Caught using a cloaking device: -100");
        }
        if self.ships_lost > 0 {
            prout!("{} ships lost:                  -{}", &self.ships_lost, self.ships_lost * 400);
        }
        if !self.alive {
            prout!("Penalty for getting killed:     -200")
        }

        prout!("\nTOTAL SCORE: {}", self.get_score());
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
pub enum Alert {
    Green,
    Yellow,
    Red,
}