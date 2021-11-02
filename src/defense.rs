use crate::io::{abbrev, get_yorn};
use crate::structs::EntityType;
use crate::constants::ALGERON;

impl crate::structs::Universe {
    /// Operate the cloaking device.
    pub fn cloak (&mut self, yorn: String) {
        if self.damage.cloak > 0.0 {
            println!("[*Engineering*] Sir, we canna' control the cloaking device until this damage is repaired.");
            return
        }

        if !self.cloaked {
            if !abbrev(&yorn, "y", "yes") && self.stardate > ALGERON {
                if !get_yorn("[*Mr. Spock*] Captain, using the cloaking device would violate the Treaty of Algeron. If the Romulans catch us using it, it will bring their ire upon us. Are you sure you want to use it?\n> ") {
                    return
                }
            }

            self.cloaked = true;
            if self.get_quadrant().search(EntityType::Romulan).len() > 0 {  // Check for Romulans
                println!("\nA Romulan ship has observed you using your cloaking device. From now on, all Romulan ships will be hostile towards you.");
                self.ididit = true; // The Romulans are royally pissed.
            }
        } else {
            self.cloaked = false;
        }
    }
}