use crate::io::{abbrev, get_yorn, input};
use crate::prout;
use crate::structs::EntityType;
use crate::constants::ALGERON;

impl crate::structs::Universe {
    /// Operate the cloaking device.
    pub fn cloak (&mut self, yorn: String) {
        if self.damage.cloak > 0.0 {
            prout!("[*Engineering*] Sir, we canna' control the cloaking device until this damage is repaired.");
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
                prout!("\nA Romulan ship has observed you using your cloaking device. From now on, all Romulan ships will be hostile towards you.");
                self.doit(); // The Romulans are royally pissed.
            }
        } else {
            self.cloaked = false;
        }
    }


    /// Control the deflector shields
    pub fn shields (&mut self, raw_mode: String, raw_amount: f64) {
        if self.damage.shields > 0.0 {
            prout!("[*Tactical*] Sir, the shield control circuit thingies are broke; I can't do anything till they're repaired.");
            return;
        }

        let mode = if raw_mode == "" {
            input("[*Tactical*] Waddya wanna do: put 'em up, take 'em down, or set the energy?\n> ")
        } else { raw_mode };
        
        if abbrev(&mode, "u", "up") {
            self.shield_status = true;
        }
        else if abbrev(&mode, "d", "down") {
            self.shield_status = false;
        }
        else if abbrev(&mode, "s", "set") {
            let amount = if raw_amount.is_nan() {
                    match input("[*Tactical*] Whaddya wanna have me set 'em to?\n> ").parse::<f64>() {
                        Ok(x) => x,
                        Err(_) => {
                            println!("[*Tactical*] I, like, can't understand what ya sayin'.");
                            return;
                        }
                    }
                } else {
                    raw_amount
            };

            if amount < 0.0 {
                prout!("[*Tactical*] I can't return more energy than actually's in the capacitors.");
                return;
            }
            else if amount > 600.0 {
                prout!("[*Tactical*] The shields, like, can't hold more than 600 energy.");
                return;
            } 
            else if self.energy - (amount - self.shields) <= 0.0 {
                prout!("[*Engineering*] Captain, we don't have that much energy available.");
                return;
            }
            else {
                self.energy -= amount - self.shields;
                self.shields = amount;
            }
        } else if mode == "" {
            return;
        } else {
            prout!("[*Tactical*] Come again?");
        }
    }
}