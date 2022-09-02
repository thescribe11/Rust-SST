use serde::{Serialize, Deserialize};
use rand::{Rng, thread_rng};
use termion::color::{Fg, Green, Reset, Red};


/// Keeps track of damage.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Damage {
    // Critical infrastructure
    pub reactors: f64,
    pub life_support: f64,
    pub warp_drive: f64,
    pub impulse_drive: f64,
    pub shields: f64,
    
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
    pub cloak: f64,
    pub computer: f64,
}

impl Damage {

    /// Instantiate a new Damage.
    pub fn new () -> Damage {
        Damage {
            shields: 0.0,
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
            cloak: 0.0,
            computer: 0.0,
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
* Shields:               {}
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
* Short-Range Sensors:   {}
* Cloaking Device:       {}
* Computer:              {}",
    form(self.reactors), 
    form(self.life_support), 
    form(self.warp_drive), 
    form(self.impulse_drive),
    form(self.shields),
    form(self.phasers), 
    form(self.torpedoes), 
    form(self.tractors), 
    form(self.deathray), 
    form(self.radio), 
    form(self.transporter), 
    form(self.shuttles),
    form(self.lrsensors),
    form(self.srsensors),
    form(self.cloak),
    form(self.computer),
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
        self.cloak         += severity * 3.0 * rng.gen::<f64>();
        self.shields       += severity * 3.0 * rng.gen::<f64>();
        self.computer      += severity * 3.0 * rng.gen::<f64>();
    }

    /// Repair damage to the ship's systems.
    pub fn repair (&mut self, elapsed: f64) {
        self.reactors -= elapsed; self.life_support -= elapsed; self.warp_drive -= elapsed; self.impulse_drive -= elapsed;
        self.phasers-= elapsed; self.torpedoes -= elapsed; self.tractors -= elapsed; self.deathray -= elapsed;
        self.radio -= elapsed; self.transporter -= elapsed; self.shuttles -= elapsed; self.lrsensors -= elapsed; 
        self.srsensors -= elapsed; self.cloak -= elapsed; self.shields -= elapsed; self.computer -= elapsed;

        if self.reactors < 0.0 { self.reactors = 0.0 };
        if self.life_support < 0.0 { self.reactors = 0.0 };
        if self.warp_drive < 0.0 { self.warp_drive = 0.0 };
        if self.impulse_drive < 0.0 { self.impulse_drive = 0.0 };
        if self.phasers < 0.0 { self.phasers = 0.0 };
        if self.torpedoes < 0.0 {self.torpedoes = 0.0 };
        if self.tractors < 0.0 { self.tractors = 0.0 };
        if self.deathray < 0.0 { self.deathray = 0.0 };
        if self.radio < 0.0 { self.radio = 0.0 };
        if self.transporter < 0.0 { self.transporter = 0.0 };
        if self.shuttles < 0.0 { self.shuttles = 0.0 };
        if self.lrsensors < 0.0 { self.lrsensors = 0.0 };
        if self.srsensors < 0.0 { self.srsensors = 0.0 };
        if self.cloak < 0.0 { self.cloak = 0.0 };
        if self.shields < 0.0 { self.shields = 0.0 };
        if self.computer < 0.0 { self.computer = 0.0 };
    }
}


fn form (item: f64) -> String {
    if item == 0.0 {
        format!("{}Operational{}", Fg(Green), Fg(Reset))
    } else {
        format!("{}{}{}", Fg(Red), item, Fg(Reset))
    }
}