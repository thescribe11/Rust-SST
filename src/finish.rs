use serde::{Serialize, Deserialize};
use termion::terminal_size;

use crate::{constants::DEBUG, io::{extra_slow_prout, get_yorn, input, slow_prout}};
use rand::{Rng, thread_rng};


impl crate::Universe {
    pub fn self_destruct (&mut self) {
        if !get_yorn("\n[* Mr. Spock *] Captain, are you sure you want to activate the self destruct?\n> ") {
            return;
        }

        if input("\nPassword: ") != self.password { return };

        slow_prout("[*Computer*] Self-destruct in 10 seconds.");
        print!(" "); extra_slow_prout('9');
        print!("  "); extra_slow_prout('8');
        print!("   "); extra_slow_prout('7');
        print!("    "); extra_slow_prout('6');
        print!("     "); extra_slow_prout('5');
        print!("      "); extra_slow_prout('4');
        print!("       "); extra_slow_prout('3');
        print!("        "); extra_slow_prout('2');
        print!("         "); extra_slow_prout('1');

        if DEBUG || thread_rng().gen_range(0..5) == 0 {
            slow_prout("            Goodbye, cruel world!\n");
        }

        let (width, half) = match terminal_size() {  // Terminal width, half the terminal width accounting for some text
            Ok(x) => (x.0, (x.0 - 10) >> 1),
            Err(_) => (80, 35),  // If the terminal width is inaccessible, assume IBM standard 80 columns
        };

        let mut star_string: String = String::new();
        for _ in 0..width {
            star_string.push('*');
        }

        let mut boom_string: String = String::new();
        for _ in 0..half {
            boom_string.push('*');
        }
        boom_string.push_str(" BOOOOOOM ");
        for _ in 0..half {
            boom_string.push('*');
        }

        slow_prout(&star_string);
        slow_prout(&boom_string);
        slow_prout(&star_string);

        self.death_reason = DeathReason::SelfDestruct;

        let mut immediate_vicinity: Vec<usize> = Vec::new();
        if self.sloc % 10 > 0 {
            immediate_vicinity.push(self.sloc-1);
            if self.sloc / 10 > 0 {
                immediate_vicinity.push(self.sloc-11);
            }
            if self.sloc / 10 < 9 {
                immediate_vicinity.push(self.sloc + 9);
            }
        }
        if self.sloc % 10 < 9 {
            immediate_vicinity.push(self.sloc + 1);
            if self.sloc / 10 > 0 {
                immediate_vicinity.push(self.sloc - 9);
            }
            if self.sloc / 10 < 9 {
                immediate_vicinity.push(self.sloc + 11);
            }
        }
        if self.sloc / 10 > 0 {
            immediate_vicinity.push(self.sloc-10);
        }
        if self.sloc / 10 < 9 {
            immediate_vicinity.push(self.sloc + 10);
        }

        for sector in immediate_vicinity {
            match self.get_quadrant().sector(&sector) {
                0 => continue,
                1 => self.score.kill_star(),
                2 => self.score.kill_starbase(),
                3 => self.score.kill_klingon(),
                4 => self.score.kill_romulan(),
                6 => self.score.kill_tholian(),
                7 => self.score.kill_unknown(),
                _ => {},
            }
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DeathReason {
    Supernova, // Blown up by a supernova
    MaximumEntropy,  // Warp core breach
    NegativeSpaceWedgie,  // Eldritch abomination
    Kaboom,   // Ship blown up
    Tribble,  // Tribble infestation
    Stranded, // You are stranded on a planet, leaving Mr. Spock in charge
    TimeUp,   // You don't stop the invasion in time
    NoAir,    // Life support reserves depleted
    NoGas,    // No fuel
    Transformation,  // Mutations
    Borg,     // We are the Borg. You will be assimilated. Resistance is futile.
    EventHorizon,  // Crushed by a black hole
    SelfDestruct,  // Goodbye, cruel world!
    GalaxyEdge,       // Your navigation is abominable.

    None, // Still alive
    GG, // You won
}