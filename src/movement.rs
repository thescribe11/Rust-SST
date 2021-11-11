use rand::Rng;
use crate::io::{get_args, get_yorn, slow_prout};
use crate::finish::DeathReason;
use crate::prout;
use crate::{input, io::abbrev};

impl crate::structs::Universe {
    pub fn move_it (&mut self, use_impulse: bool, angle: Option<f64>, distance: Option<f64>) {
        //! Move the Enterprise.
        //! Speed should be `self.warp_factor` for normal warp movement.
        //! Set it to 0.5 for impulse (sublight) drive.

        // Check to make sure the drive the player wants to move isn't damaged or unusable
        if use_impulse && self.damage.impulse_drive > 3.5 {
            prout!("[*Engineering*] Sir, the Impulse Drive is inoperable. We canna' use it.");
            return
        } else if !use_impulse && self.damage.warp_drive > 2.0 {
            prout!("[*Engineering*] The warp coils are smashed! Using it right now would blow the ship to smithereens!");
            return
        } else if !use_impulse && self.damage.warp_drive > 0.0 && self.warp_factor > 2.5 {
            prout!("[*Engineering*] The warp engines are damaged, sir; I can only give you warp 2.5.");
            return
        }
         else if !use_impulse && self.cloaked {
            prout!("[*Engineering*] We canna' use the warp drive while the claoking device is active!");
            if self.damage.impulse_drive < 3.5 {
                prout!("[*Engineering*] ... that said, I could probably give you impulse.");
            }
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
                        prout!("[*Helm*] That isn't an angle.");
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
                        prout!("[*Helm*] \"Second to the right and straight on till morning\" isn't a valid course.");
                        return;
                    }
                }
            }
        };

        let power = match use_impulse {
            false => 1.05 * self.warp_factor.powi(2) * (self.shield_status as u8 + 1) as f64 * (distance * bigger).round(),  // Shamelessly lifted from the Almy version
            true => 20.0 + 100.0 * distance
        };

        if power >= self.energy {
            prout!("[*Mr. Spock*] Captain, we do not have sufficient power to complete that manuever.");
            if use_impulse {
                prout!("According to my calculations, we can only go {:.2} quadrants before we run out of power.", (self.energy - 20.0) * 10.0);
            }
            else if self.shield_status || (0.5*power) > self.energy {
                let iwarp = (self.energy/(distance+0.05)).powf(1.0/3.0);
                if iwarp > 0.0 {
                    prout!("That said, we could do it at warp {}{}", iwarp, match self.shield_status {
                        false => ".",
                        true => ", provided we lower the shields."
                    });
                }
            }
            return;
        }

        let time = match use_impulse {
            true => distance*bigger / 0.095,
            false => distance*bigger / self.warp_factor
        };

        if time > (self.time_remaining * 0.8) {  // Ask for confirmation if the trip takes more than 80% of the remaining time
            match use_impulse {
                true => {
                    prout!("[*Mr. Spock*] Captain, we can only go 0.95 sectors per stardate under impulse power. Are you sure we dare spend the time?");
                    if !abbrev(&input("> ").to_lowercase(), "y", "yes") {
                        return
                    }
                },
                false => {
                    prout!("[*Mr. Spock*] Sir, that would take us {:.2}% of our remaining time. Are you sure this is wise?", 100.0*time/self.time_remaining);
                    if !abbrev(&input("> ").to_lowercase(), "y", "yes") {
                        return
                    }
                }
            }
        }

        let mut interquad  = false;  // Has the Enterprise gone to a different quadrant?
        let (mut nsvert, mut nshoriz, mut nqvert, mut nqhoriz) = ((self.sloc / 10) as f64, self.sloc as f64 % 10 as f64, self.qvert as i32, self.qhoriz as i32);
        let mut interrupted = false;

        if use_impulse {
            self.energy -= 20.0;  // Initial warmup cost.
        }

        // Having finished with the prerequisite data acquisition and confirmation, actually move the ship.
        for _i in 0..(distance * bigger).round() as usize {
            nshoriz += dh;
            nsvert += dv;

            // Subtract time and energy.
            match use_impulse {
               false => {
                   self.energy -= 1.05 * self.warp_factor.powi(2) * ((self.shield_status as u8 + 1) as f64);
                   self.add_time(3.0*bigger/self.warp_factor);
                },
               true => {
                   self.energy -= 100.0;
                   self.add_time(0.95);
               }
            }

            // Check to make sure that the player hasn't run out of time or gas.
            if self.time_remaining <= 0.0 {
                self.death_reason = DeathReason::TimeUp;
                break;
            } else if self.energy <= 0.0 {
                self.death_reason = DeathReason::NoGas;
                break;
            }

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

            if !interquad {
                match self.get_other_quadrant(&(nqvert as usize), &(nqhoriz as usize)).sector(&newloc) {
                    0 => continue,
                    1 | 2 => {  // Neutral or inanimate object
                        interrupted = true;
                        prout!("\nWARNING: Course blocked by object at sector {} {}", nsvert.round() as i32 + 1, nshoriz.round() as i32 + 1);
                        let stop_energy = 95.0 * self.warp_factor;
                        prout!("Emergency stop requires {} units of energy.", stop_energy);
                        self.energy -= stop_energy;

                        if self.energy <= 0.0 {
                            self.death_reason = DeathReason::NoGas;
                            return;
                        }

                        nsvert -= dv; nshoriz -= dh;  // Undo a step of the move
                        break;
                    },
                    i if [3, 4, 6, 7].contains(&i) => {  // Enemy. Ramming speed!
                        interrupted = true;
                        self.ram(i, &nqvert, &nqhoriz, &newloc);
                        break;
                    },
                    5 => {  // Black hole
                        interrupted = true;
                        slow_prout("\n***RED ALERT! RED ALERT!***");
                        slow_prout("\nThe Enterprise is pulled into a black hole, crushing it like a tin can.");
                        self.die(DeathReason::EventHorizon);
                    },
                    _ => {
                        panic!("AAAAAAH! Something unexpected occured!")
                    }
                }
            }
        }

        if self.warp_factor > 6.0 && !use_impulse && !interrupted{
            if ((6.0-self.warp_factor).powf(2.0) * (distance * bigger) / (2.0/3.0)) > rand::random() {
                // Whoopsies! The warp engine has been damaged.
                self.damage.warp_drive += rand::random::<f64>() * distance / 10.0;
            }

            if !interrupted && ((6.0-self.warp_factor).powf(2.3) * (distance * bigger) / (2.0/3.0)) > rand::random() {
                let amount: f64 = if rand::random::<f64>() > 0.5 {
                    rand::thread_rng().gen_range(-10.0..1.0)
                } else {
                    rand::thread_rng().gen_range(1.0..10.0)
                };

                self.time_remaining += amount;
                self.stardate += amount;
            }
        }

        let (old_sloc, old_qvert, old_qhoriz) = (self.sloc.clone(), self.qvert.clone(), self.qhoriz.clone());
        self.sloc = ((nsvert*10.0) + nshoriz).round() as usize;
        self.qvert = nqvert as usize;
        self.qhoriz = nqhoriz as usize;
        self.hit_me = true;

        self.place_ship(old_qvert, old_qhoriz, old_sloc);
    }  // End move_it

    /// Place the ship in a new location after movement, shockwave knockback, etc.
    /// 
    /// Args:
    /// - old_qvert: usize. The ship's old vertical location in the galaxy.
    /// - old_qhoriz: usize. The ship's old horizontal location in the galaxy.
    /// - old_sloc: usize. The ship's old location in the sector.
    pub fn place_ship (&mut self, old_qvert: usize, old_qhoriz: usize, old_sloc: usize) {
        self.quadrants[self.qvert][self.qhoriz].sectors[self.sloc] = 8;
        self.quadrants[old_qvert][old_qhoriz].sectors[old_sloc] = 0;

        if self.get_quadrant().neutral_zone() && self.damage.radio == 0.0 {
            prout!("\n[*Lt. Uhura*] Captain, a Romulan ship is hailing us. I'll put it on audio.");
            if self.ididit {
                // The Romulans are royally pissed; skip the pleasantries.
                prout!("*click* DIE, TREACHEROUS HUMAN SCUM!!!");
            } else {
                // Courteously threaten to destroy the Enterprise.
                prout!("*click* Captain, I'm afraid you're violating the Romulan Neutral Zone. Please leave, lest your situation become... terminally unpleasant.");
            }
        }
    }

    /// Ram an enemy ship.
    ///
    /// Args:
    /// - loc: usize
    pub fn ram (&mut self, i: u8, nqvert: &i32, nqhoriz: &i32, nloc: &usize) {
        let enemy_type: f64 = match i {
            3 => 1.0,
            4 => 1.5,
            6 => 0.5,
            _ => 0.8,
        };

        self.quadrants[*nqvert as usize][*nqhoriz as usize].kill_entity(&nloc);
        self.kill_enemy(&(*nqvert as usize), &(*nqhoriz as usize), &*nloc);

        self.damage.add_ramming_damage(enemy_type);

        prout!("***Enemy ship at ({}, {}) destroyed in collision.", (nloc / 10) + 1, (nloc % 10) + 1);
    }

    /// The player is attempting to leave the galaxy.
    ///
    /// I can't let that happen, so the player gets two warnings.
    ///
    /// The third time they attempt this I will be forced to concede
    /// the abominableness of their navigation, and so kill 'em.
    #[allow(non_snake_case)]
    pub fn NOPE (&mut self) {
        if self.leave_attempts < 2 {
            slow_prout("\nYOU HAVE ATTEMPTED TO CROSS THE NEGATIVE ENERGY BARRIER AT THE EDGE OF THE GALAXY.\nTHE THIRD TIME YOU TRY TO DO THIS YOU WILL BE DESTROYED.");
            self.leave_attempts += 1;
        } else {
            self.alive = false;
            self.score.lose_ship();
            self.death_reason = DeathReason::GalaxyEdge;
        }
    }


    /// Change the ship's warp factor
    pub fn change_warp (&mut self, mut new_factor: f64) {
        if new_factor == f64::NEG_INFINITY {
            new_factor = match input("New warp factor: ").parse::<f64>() {
                Ok(f) => f,
                Err(_) => {
                    prout!("[*Helm*] Say again, sir?");
                    return
                }
            }
        }

        if (new_factor <= 0.0) | (new_factor > 10.0) {
            prout!("[*Engineering*] Do you think I'm God? I canna' change the laws of physics!");
            prout!("[*Mr. Spock*] Captain, we can only go up to warp 10.");
            return
        } else if new_factor > 6.0 {
            // Speeds greater than warp 6 risk damage to the warp engines.
            if !get_yorn("[*Mr. Spock*] Sir, we'd risk damaging the warp engines at that speed. Are you sure the risk is worth it?\n> ") {
                return
            }
        }

        self.warp_factor = new_factor;
    }


    /// Take a rest for a while.
    pub fn rest (&mut self, mut duration: f64) {
        if duration.is_nan() {
            duration = match get_args::<f64>(input("\nHow much time would you like to skip? ")) {
                Some (d) => match d.len() {
                    1 => d[0],
                    _ => {
                        prout!("Huh?");
                        return
                    }
                },
                None => {
                    return;
                }
            };
        }

        if duration < 0.0 {
            prout!("[*Mr. Spock*] Captain, need I remind you that under normal conditions we always go forwards in time?");
            return
        } else if duration >= self.time_remaining {
            if get_yorn("Captain, that would take more than our remaining time. Are you sure you wish to do this?\n> ") {
                self.death_reason = DeathReason::TimeUp;
            } else {
                return;
            }
        }

        self.add_time(duration);
        self.hit_me = true;  // Time has elapsed, so the baddies get to attack.
    }

}