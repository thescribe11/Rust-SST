//! Contains all the various data structs used by the program.
extern crate rand;
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::iter::FromIterator;


#[derive(Debug, Serialize, Deserialize)]
pub struct Enterprise {
    // Vital statistics
    crew: u8,

    // Engineering
    energy: u32,
    energy_reserves: u32,
    deuterium: bool,

    // Life support
    life_reserves: u8,
    on_life_reserve: bool,
    
    // Docking
    docked: bool,

    // Weapons
    torpedoes: u8,

    // Damage
    damage: Damage,
}

impl Enterprise {
    pub fn new () -> Enterprise {
        Enterprise {
            crew: 150,

            energy: 3000,
            energy_reserves: 500,
            deuterium: false,

            life_reserves: 5,
            on_life_reserve: false,

            docked: false,

            torpedoes: 10,

            damage: Damage::new()
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
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
            shuttles: 0
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
", 
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
    self.shuttles as f32 / 10f32
        )
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Universe {
    pub klingons: u32,
    pub score: Score,
    pub starbases: u32,
    pub ididit: bool,

    pub quadrants: [[Quadrant; 8]; 8],
    
    pub player_name: Vec<String>,
    pub password: String,
}

impl Universe {
    pub fn new (player_name: Vec<String>, password: String) -> Universe {
        let mut randint = rand::thread_rng();
        let starbases: u32 = randint.gen_range(3..8);

        let mut to_return = Universe {
            klingons: 0,  // Updated later by polling quadrants
            score: Score::new(),
            starbases,
            ididit: false,

            // Since the compiler won't let me .copy() the Vec in Universe...
            quadrants: [[Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],
            [Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default(),Quadrant::default()],],

            player_name,
            password
        };


        for i in 0..8 {
            for j in 0..8 {
                to_return.klingons += to_return.quadrants[i][j].init();
            }
        }

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
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Quadrant {
    sectors: Vec<u8>,  // [u8; 100] would be more efficient, but it doesn't play well with Serde.
    entities: Vec<(EntityType, usize, Health, Alignment)>,
    is_supernova: bool,
}

impl Quadrant {
    fn default () -> Quadrant {
        //! Creates a quadrant. A starbase, if present, is added later.

        let mut randint = rand::thread_rng();

        let mut to_return = Quadrant {
            sectors: Vec::from_iter([0u8; 100]),  // 1 = star, 2 = starbase, 3 = klingon, 4 = romulan, 5 = black hole, 6 = tholian, 7 = unknown entity
            entities: Vec::new(),
            is_supernova: false
        };

        return to_return
    }

    fn init (&mut self) -> u32 {
        // Initialize the quadrant

        let mut randint = rand::thread_rng();
        let mut klingons: u32 = 0;

        for i in 0..100 {
            match randint.gen_range(1..=20) {
                0..=3 => {
                    self.sectors[i] = 3; 
                    klingons += 1;
                    self.entities.push((
                        EntityType::Klingon,
                        i,
                        Health::Health(randint.gen_range(150..=300)),
                        Alignment::Enemy
                    ))
                },  // Klingon
                4 => self.sectors[i] = 4,      // Romulan
                5..=8 => self.sectors[i] = 5,  // Black hole.
                9..=14 => self.sectors[i] = 1, // Star

                _ => {}
            }  
        }

        return klingons
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

    fn poll_lrscan (&self) -> (u32, u32, u32) {
        unimplemented!()
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