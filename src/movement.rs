use crate::Input::slow_prout;
use crate::finish::DeathReason;
use crate::{input, Input::abbrev};
use crate::constants::DEBUG;

impl crate::structs::Universe {
    pub fn move_it (&mut self, use_impulse: bool, angle: Option<f64>, distance: Option<f64>) {
        //! Move the Enterprise.
        //! Speed should be `self.warp_factor` for normal warp movement.
        //! Set it to 0.5 for impulse (sublight) drive.

        // Check to make sure the drive the player wants to move isn't damaged or unusable
        if use_impulse && self.damage.impulse_drive > 0 {
            println!("[*Engineering*] Sir, the Impulse Drive is inoperable. We canna' use it.");
            return
        } else if !use_impulse && self.damage.warp_drive > 0 {
            println!("[*Engineering*] Sir, the warp drive is damaged! Using it right now would blow the ship to smithereens!");
            return
        } else if !use_impulse && self.cloaked {
            println!("[*Engineering*] We canna' use the warp drive while the claoking device is active!");
            println!("[*Engineering*] ... that said, I could probably give you impulse.");
            return;
        }

        let (mut dv, mut dh) = match angle {
            Some(x) => (-x.to_radians().sin(), x.to_radians().cos()),
            None => {
                let raw = input("Direction: ");
                if raw.is_empty() {
                    return;
                }

                match raw.parse::<f64>() {
                    Ok(x) => (-x.to_radians().sin(), x.to_radians().cos()),
                    Err(_) => {
                        println!("[*Helm*] That isn't an angle.");
                        return;
                    }
                }
            }
        };

        let bigger = match dv.abs() > dh.abs() {  // Controls which of the directions is used as the increment
            true => dv,
            false => dh
        }.abs();
        dv /= bigger; dh /= bigger;  // This introduces some errors, but they're insignificant.

        if DEBUG {
            println!("Δvert: {}, Δhoriz: {}", &dv, &dh);
        }
        
        let distance = match distance {
            Some(x) => x,
            None => {
                let raw = input("Distance: ");
                if raw.is_empty() {
                    return;
                }

                match raw.parse::<f64>() {
                    Ok(x) => x,
                    Err(_) => {
                        println!("[*Helm*] \"Second to the right and straight on till morning\" isn't a valid course.");
                        return;
                    }
                }
            }
        };

        let power = match use_impulse {
            false => 1.05 * self.warp_factor.powi(3) * (self.shield_status as u8) as f64 * (distance * bigger).round(),  // Shamelessly lifted from the Almy version
            true => 20.0 + 100.0 * distance
        };

        if power >= self.energy {
            println!("[*Mr. Spock*] Captain, we do not have sufficient power to complete that manuever.");
            if use_impulse {
                println!("According to my calculations, we can only go {:.2} quadrants before we run out of power.", (self.energy - 20.0) * 10.0);
            }
            else if self.shield_status || (0.5*power) > self.energy {
                let iwarp = (self.energy/(distance+0.05)).powf(1.0/3.0);
                if iwarp > 0.0 {
                    println!("That said, we could do it at warp {}{}", iwarp, match self.shield_status {
                        false => ".",
                        true => ", provided we lower the shields."
                    });
                }
            }
            return;
        }

        let time = match use_impulse {
            true => distance / 0.095,
            false => 10.0*distance/self.warp_factor
        };

        if time > (self.time_remaining * 0.8) {  // Ask for confirmation if the trip takes more than 80% of the remaining time
            match use_impulse {
                true => {
                    println!("[*Mr. Spock*] Captain, we can only go 0.95 sectors per stardate under impulse power. Are you sure we dare spend the time?");
                    if !abbrev(&input("> ").to_lowercase(), "y", "yes") {
                        return
                    }
                },
                false => {
                    println!("[*Mr. Spock*] Sir, that would take us {:.2}% of our remaining time. Are you sure this is wise?", 100.0*time/self.time_remaining);
                    if !abbrev(&input("> ").to_lowercase(), "y", "yes") {
                        return
                    }
                }
            }
        }

        let mut interquad  = false;  // Has the Enterprise gone to a different quadrant?

        let (mut nsvert, mut nshoriz, mut nqvert, mut nqhoriz) = ((self.sloc / 10) as f64, self.sloc as f64 % 10 as f64, self.qvert as i32, self.qhoriz as i32);

        // Having finished with the prerequisite data acquisition and confirmation, actually move the ship.
        for _i in 0..(distance * bigger).round() as usize {
            nshoriz += dh;
            nsvert += dv;

            self.energy -= 1.05 * self.warp_factor.powi(3) * ((self.shield_status as u8 + 1) as f64);
            
            if nshoriz > 9.0 {
                interquad = true;
                nqhoriz += 1;
                nshoriz = 0.0;
            } else if nshoriz < 0.0 {
                interquad = true;
                nqhoriz -= 1;
                nshoriz = 9.0;
            }

            if nsvert > 9.0 {
                interquad = true;
                nqvert += 1;
                nsvert = 0.0;
            } else if nsvert < 0.0 {
                interquad = true;
                nqvert -= 1;
                nsvert = 9.0
            }

            if nqvert < 0 || nqvert > 7 || nqhoriz < 0 || nqhoriz > 7 {
                // Whoopsies! The player tried to leave the galaxy.
                self.NOPE();

                if nqvert < 0 {
                    nqvert += 1;
                    nsvert = 0.0;
                }
                if nqvert > 7 {
                    nqvert -= 1;
                    nsvert = 99.0;
                }
                if nqhoriz < 0 {
                    nqhoriz += 1;
                    nshoriz = 0.0;
                }
                if nqhoriz > 7 {
                    nqhoriz -= 1;
                    nshoriz = 99.0;
                }
                
                break;
            }

            let newloc = ((nsvert * 10.0) + nshoriz).round() as usize;
            println!("Newloc: {}", newloc+1);

            println!("{}, {}", nsvert+1.0, nshoriz+1.0);

            if !interquad {
                match self.get_other_quadrant(&(nqvert as usize), &(nqhoriz as usize)).sector(&newloc) {
                    0 => continue,
                    1 | 2 => {  // Neutral or inanimate object
                        println!("WARNING: Course blocked by object at sector {} {}", nsvert.round() as i32 + 1, nshoriz.round() as i32 + 1);
                        let stop_energy = 95.0 * self.warp_factor;
                        println!("Emergency stop requires {} units of energy.", stop_energy);
                        self.energy -= stop_energy;

                        if self.energy <= 0.0 {
                            self.death_reason = DeathReason::NoGas;
                            return;
                        }

                        nsvert -= dv; nshoriz -= dh;  // Undo a step of the move
                        break;
                    },
                    i if [3, 4, 6, 7].contains(&i) => {  // Enemy. Ramming speed!
                        let enemy_type: f64 = match i {
                            3 => 1.0,
                            4 => 1.5,
                            6 => 0.5,
                            _ => 0.8,
                        };

                        self.quadrants[nqvert as usize][nqhoriz as usize].kill_entity(&newloc);
                    },
                    5 => {  // Black hole
                        slow_prout("***RED ALERT! RED ALERT!***");
                        slow_prout("\nThe Enterprise is pulled into a black hole, crushing it like a tin can.");
                        self.die(DeathReason::EventHorizon);
                    },
                    _ => {
                        panic!("AAAAAAH!")
                    }
                }
            }
        }

        let (old_sloc, old_qvert, old_qhoriz) = (self.sloc.clone(), self.qvert.clone(), self.qhoriz.clone());
        self.sloc = ((nsvert*10.0) + nshoriz).round() as usize;
        self.qvert = nqvert as usize;
        self.qhoriz = nqhoriz as usize;

        self.place_ship(old_qvert, old_qhoriz, old_sloc);
    }  // End move_it

    pub fn place_ship (&mut self, old_qvert: usize, old_qhoriz: usize, old_sloc: usize) {
        println!("oqvert: {}; oqhoriz: {}; osloc: {}", old_qvert, old_qhoriz, old_sloc);
        println!("nqvert: {}; nqhoriz: {}; nsloc: {}", self.qvert as i32, self.qhoriz as i32, self.sloc as i32);
        self.quadrants[self.qvert][self.qhoriz].sectors[self.sloc] = 8;
        self.quadrants[old_qvert][old_qhoriz].sectors[old_sloc] = 0;
    }
}