use serde::{Serialize, Deserialize};
use rand::{Rng, thread_rng};


/// Keeps track of damage.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Damage {
    // Critical infrastructure
    pub reactors: f64,
    pub life_support: f64,
    pub warp_drive: f64,
    pub impulse_drive: f64,
    
    // Weapons
    pub phasers: f64,
    pub torpedoes: f64,
    pub tractors: f64,
    pub deathray: f64,

    // Accessories
    pub radio: f64,
    pub transporter: f64,
    pub shuttles: f64,
    pub lrsensors: f64,
    pub srsensors: f64,
}

impl Damage {

    /// Instantiate a new Damage.
    pub fn new () -> Damage {
        Damage {
            reactors: 0.0,
            life_support: 0.0,
            warp_drive: 0.0,
            impulse_drive: 0.0,
            phasers: 0.0,
            torpedoes: 0.0,
            tractors: 0.0,
            deathray: 0.0,
            radio: 0.0,
            transporter: 0.0,
            shuttles: 0.0,
            lrsensors: 0.0,
            srsensors: 0.0,
        }
    }


    /// Present the current state in a human-readable format.
    pub fn print_damage (&self) {
        println!(
"\n******** DAMAGE REPORT ********
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
    self.reactors, 
    self.life_support, 
    self.warp_drive, 
    self.impulse_drive, 
    self.phasers, 
    self.torpedoes, 
    self.tractors, 
    self.deathray, 
    self.radio, 
    self.transporter, 
    self.shuttles,
    self.lrsensors,
    self.srsensors
        )
    }

    /// Brace for impact!
    pub fn add_ramming_damage (&mut self, severity: f64) {
        let mut rng = thread_rng();

        // Systematically damage every system of the ship.
        self.reactors      += severity * 3.0 * rng.gen::<f64>();
        self.life_support  += severity * 3.0 * rng.gen::<f64>();
        self.warp_drive    += severity * 3.0 * rng.gen::<f64>();
        self.impulse_drive += severity * 3.0 * rng.gen::<f64>();
        self.phasers       += severity * 3.0 * rng.gen::<f64>();
        self.torpedoes     += severity * 3.0 * rng.gen::<f64>();
        self.tractors      += severity * 3.0 * rng.gen::<f64>();
        self.deathray      += severity * 3.0 * rng.gen::<f64>();
        self.radio         += severity * 3.0 * rng.gen::<f64>();
        self.transporter   += severity * 3.0 * rng.gen::<f64>();
        self.shuttles      += severity * 3.0 * rng.gen::<f64>();
        self.lrsensors     += severity * 3.0 * rng.gen::<f64>();
        self.srsensors     += severity * 3.0 * rng.gen::<f64>();
    }

    /// Repair damage to the ship's systems.
    pub fn repair (&mut self, elapsed: f64) {
        self.reactors -= elapsed; self.life_support -= elapsed; self.warp_drive -= elapsed; self.impulse_drive -= elapsed;
        self.phasers-= elapsed; self.torpedoes -= elapsed; self.tractors -= elapsed; self.deathray -= elapsed;
        self.radio -= elapsed; self.transporter -= elapsed; self.shuttles -= elapsed; self.lrsensors -= elapsed; 
        self.srsensors -= elapsed;

        if self.reactors < 0.0 { self.reactors = 0.0 };
        if self.life_support < 0.0 { self.reactors = 0.0 };
        if self.warp_drive < 0.0 { self.warp_drive = 0.0 };
        if self.impulse_drive < 0.0 { self.impulse_drive = 0.0 };
        if self.phasers < 0.0 { self.phasers = 0.0 };
    }
}