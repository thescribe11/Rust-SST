use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;
use crate::io::{get_args, get_yorn, slow_prout, SLOW, EXTRA_SLOW};
use crate::finish::DeathReason;
use crate::{prout};
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
        dv /= bigger; dh /= bigger;  // This introduces some inaccuracies, but they're insignificant.
        
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
        if distance < 1.0 {
            prout!("[*Helm*] Sir, that's an invalid distance.");
            return;
        }

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

            // Check if the player has run out of time or gas.
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
                        slow_prout("\n***RED ALERT! RED ALERT!***", SLOW, true);
                        slow_prout("\nThe Enterprise is pulled into a black hole, crushing it like a tin can.", SLOW, true);
                        self.die(DeathReason::EventHorizon);
                    },
                    _ => {
                        panic!("AAAAAAH! Something unexpected happened while checking for collisions!")
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

        if !self.is_quadrant_accessible(self.qvert, self.qhoriz) {
            self.emergency_jump();
            return;
        }

        if self.get_quadrant().neutral_zone() && self.damage.radio == 0.0 {
            prout!("\n[*Lt. Uhura*] Captain, a Romulan ship is hailing us. I'll put it on audio.");
            if self.ididit {
                // The Romulans are royally pissed; skip the pleasantries.
                slow_prout("*click* DIE, TREACHEROUS HUMAN SCUM!!!", SLOW, true);
            } else {
                // Courteously threaten to destroy the Enterprise.
                slow_prout("*click* Captain, I'm afraid you're violating the Romulan Neutral Zone. Please leave, lest your situation become... terminally unpleasant.", SLOW, true);
            }
        }
    }


    /// Change the Enterprise's location without dickering around with the old coordinates
    pub fn non_movement_place_ship (&mut self, new_qvert: usize, new_qhoriz: usize, new_sloc: usize) {
        self.quadrants[self.qvert][self.qhoriz].sectors[self.sloc] = 0;  // Remove Enterprise from old quadrant
        
        if !self.is_quadrant_accessible(self.qvert, self.qhoriz) {  // Check for a supernova or tholian webbing
            self.emergency_jump();
            return;
        }

        self.qvert = new_qvert; self.qhoriz = new_qhoriz; self.sloc = new_sloc;  // Change the Enterprise's coords
        self.quadrants[self.qvert][self.qhoriz].sectors[self.sloc] = 8;  // Place Enterprise in new quadrant

        if self.get_quadrant().neutral_zone() && self.damage.radio == 0.0 {  // Check to see if the new quadrant is part of the Neutral Zone
            prout!("\n[*Lt. Uhura*] Captain, a Romulan ship is hailing us. I'll put it on audio.");
            if self.ididit {
                // The Romulans are royally pissed; skip the pleasantries.
                slow_prout("*click* DIE, TREACHEROUS HUMAN SCUM!!!", SLOW, true);
            } else {
                // Courteously threaten to destroy the Enterprise.
                slow_prout("*click* Captain, I'm afraid you're violating the Romulan Neutral Zone. Please leave, lest your situation become... terminally unpleasant.", SLOW, true);
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
            slow_prout("\nYOU HAVE ATTEMPTED TO CROSS THE NEGATIVE ENERGY BARRIER AT THE EDGE OF THE GALAXY.\nTHE THIRD TIME YOU TRY TO DO THIS YOU WILL BE DESTROYED.", SLOW, true);
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


    /// Risk a long-range transport to the nearest starbase.
    /// Doesn't work if the subspace radio is damaged.
    pub fn call (&mut self) {
        if self.damage.radio > 1.8 {  // Can't send data
            prout!("[*Lt. Uhura*] Captain, the subspace radio is inoperable.");
            return;
        } else if self.starbases == 0 {
            slow_prout("[*Lt. Uhura*] I'm sorry captain... nobody's responding to our distress calls.", SLOW, true);
            return;
        }
        let available = self.get_starbases();  // Since the compiler complains about temporary values otherwise
        let selected = match available.choose(&mut rand::thread_rng()) {
            Some(s) => s,
            None => {
                slow_prout("[*Lt. Uhura*] I'm sorry captain... nobody's responding to our distress calls.", SLOW, true);
                return;
            }
        };
        
        if crate::DEBUG {
            prout!("Selected starbase: {:?}", selected);
        }
        if !get_yorn("[*Lt. Sulu*] Captain, are you sure this is a good idea?\n> ") {
            return;
        }

        let starbase_loc = self.get_quadrant().search(crate::structs::EntityType::Starbase)[0].1;
        
        let mut unoccupied: Vec<usize> = Vec::new();
        for sector in crate::scans::get_vicinity(starbase_loc) {
            if self.get_other_quadrant(&selected[0], &selected[1]).sector(&sector) == 0 {
                unoccupied.push(sector);
            }
        }

        self.sloc = match unoccupied.choose(&mut rand::thread_rng()) {
            Some(s) => *s,
            None => {
                slow_prout("[*Lt. Uhura*] Captain, a starbase received our distress call. The base commandant sends his condolences; he doesn't have any open births.", SLOW, true);
                return;
            }
        };

        self.qvert = selected[0]; self.qhoriz = selected[1];
    }


    pub fn emergency_jump (&mut self) {
        //! Attempt to get away from a supernova.

        slow_prout("**AWHOOGAH**   **AWHOOGAH**", SLOW, true);
        slow_prout("[*Comp.*] SUPERNOVA DETECTED", SLOW, true);
        slow_prout("[*Comp.*] ENGAGING EMERGENCY ENGINE OVERRIDE", SLOW, true);
        slow_prout("  ", EXTRA_SLOW, true);

        if self.damage.computer > 0.15 {
            slow_prout("[*Comp.*] !ERROR!: CONTROL INTERLINKS INOPERABLE!                      ", SLOW, true);
            prout!("******************* BOOM *******************");
            self.die(DeathReason::Supernova);
            return;
        }

        let mut randint = thread_rng();
        let dvert: i32 = randint.gen_range(-2..2);
        let dhoriz = randint.gen_range(-2..2);
        
        let chosen_vert = self.qvert as i32 + dvert;
        let chosen_horiz = self.qhoriz as i32 + dhoriz;

        if chosen_vert < 0 || chosen_vert > 7 
          || chosen_horiz < 0 || chosen_horiz > 7 
          || randint.gen::<f32>() < 0.2 
          || !self.is_quadrant_accessible(chosen_vert as usize, chosen_horiz as usize){
            slow_prout("[*Comp.*] OVERRIDE FAILED.", SLOW, false);

            if crate::DEBUG || randint.gen_range(0..5) == 0 {  // Continuing the self-aware computer joke
                slow_prout(" SO LONG, AND THANKS FOR ALL THE FISH.", SLOW, false);
            }

            slow_prout("       ", EXTRA_SLOW, true);  // Give the player some time to consider things
            prout!("******************* BOOM *******************");
            self.die(DeathReason::Supernova);
            return;
        }

        self.energy -= (100 + 100*chosen_vert+100*chosen_horiz) as f64;
        if self.energy < 0.0 {
            self.die(DeathReason::NoGas);
            return;
        }

        let mut new_sloc: usize = 0;
        let mut spot_found = false;
        while !spot_found {
            new_sloc = randint.gen_range(0..100);
            if self.get_other_quadrant(&(chosen_vert as usize), &(chosen_horiz as usize)).sector(&new_sloc) == 0 {
                self.sloc = new_sloc;
                spot_found = true;
            }
        }

        self.non_movement_place_ship(chosen_vert as usize, chosen_horiz as usize, new_sloc)
    }
}