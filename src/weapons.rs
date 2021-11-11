use rand::{random, thread_rng, Rng};
use termion::color::{Fg, Red, Reset};
use crate::prout;

use crate::{finish::DeathReason, input, io::{extra_slow_prout, get_args, get_yorn, slow_prout}, structs::EntityType};

impl crate::structs::Universe {
    pub fn torpedo (&mut self, num: Option<u8>, deltas: Vec<u8>) {
        //! Fire torpedoes

        // Get # of torpedoes to fire
        let to_fire = match num {
            Some(i) => i,
            None => {
                let x = input("How many torpedoes would you like to fire? ");
                match x.parse::<u8>() {
                    Ok(i) => i,
                    Err(_) => {
                        prout!("Sir, I can't fire \"{}\" torpedoes.", x);
                        return;
                    }
                }
            }
        };

        // Check for invalid cases
        if to_fire > self.torpedoes {
            prout!("[*Armory*] What do you think we are, the Bank of Ferenginar?");
            return;
        } else if to_fire == 0 {
            return;
        } else if to_fire > self.torpedoes {
            prout!("[*Armory*] We can't fire that many sir; we only {} left.", &self.torpedoes);
            return
        } else if to_fire > 3 {
            prout!("[*Armory*] Sir, we can only fire three at a time; any more and we would melt the tubes!")
        }

        let _d = match deltas.len() {  // Get a firing solution if there isn't already one.
            0 => {
                let mut _d: Vec<u8> = Vec::new();
                let mut index = 0;
                while index < to_fire as usize {
                    print!("Input direction for torpedo #{}: ", index+1);
                    match get_args(input("")) {
                        Some(v) => {
                            for i in v {
                                _d.push(i);
                            }
                            index += 1;
                        },
                        None => {
                            prout!("[*Armory*] Sir, that doesn't make sense.");
                            return
                        }
                    }
                }
                _d
            },
            _ => deltas
        };

        // Parse firing solution
        let solution: Vec<i8> = _d.into_iter().map(|x| match x%8 {
            1 => 1,
            2 => -9,
            3 => -10,
            4 => -11,
            5 => -1,
            6 => 9,
            7 => 10,
            8 => 11,
            _ => panic!("_d shouldn't contain any value which isn't between 0 and 8 post-modulo!")
        }).collect::<Vec<i8>>();

        let mut torp_num = 0;
        // Fire torpedoes
        for delta in solution {
            torp_num += 1;
            let mut torp_loc: i8 = self.sloc as i8;
            print!("\nTrack for torpedo #{}: ", &torp_num);

            // Simulate torpedo
            loop {
                torp_loc += delta;
                if torp_loc < 0 || torp_loc > 99 {
                    prout!("\nTorpedo misses.");
                    break;
                }

                // Process impact
                match self.get_quadrant().get_entity(torp_loc as usize) {
                    Some((t, loc, he, al)) => {
                        match t {
                            EntityType::BlackHole => {
                                prout!("\n Torpedo swallowed by black hole.");
                            },
                            EntityType::Klingon => {  // Klingons are always destroyed by torpedoes, although I might want to change this later.
                                prout!("\n ***Klingon at sector ({}, {}) destroyed.", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                                self.score.kill_klingon();
                                self.klingons -=1;
                            },
                            EntityType::Romulan => {
                                match random::<u8>() {
                                    0..=200 => {  // Romulan dies
                                        prout!("\n ***Romulan at sector ({}, {}) destroyed.", (torp_loc/10)+1, (torp_loc%10)+1);
                                        self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                                        self.score.kill_romulan();
                                    },
                                    201..=255 => {
                                        self.quadrants[self.qvert][self.qhoriz].damage_entity(torp_loc as usize, 500);
                                        self.score.kill_romulan()
                                    }
                                }
                            },
                            EntityType::Star => {
                                prout!("\n ***Torpedo impacts star at sector ({}, {}), causing it to go nova.", (torp_loc/10)+1, (torp_loc%10)+1);
                                for i in &[torp_loc-11, torp_loc-10, torp_loc-9, torp_loc-1, torp_loc, torp_loc+1, torp_loc+9, torp_loc+10, torp_loc+11] {
                                    if *i > -1 && *i < 100 {
                                        match self.get_quadrant().sectors[*i as usize] {
                                            0 => {},  // Empty space; do nothing.
                                            1 => self.score.kill_star(),
                                            2 => self.score.kill_starbase(),
                                            3 => self.score.kill_klingon(),
                                            4 => self.score.kill_romulan(),
                                            5 => continue,  // Novas don't do anything to black holes either.
                                            6 => self.score.kill_tholian(),
                                            7 => self.score.kill_unknown(),
                                            8 => {},
                                            _ => {
                                                prout!("{}", self.get_quadrant().sectors[*i as usize]);
                                                panic!("Somehow a corrupted value has gotten into the sector map.")
                                            }
                                        }
                                        self.quadrants[self.qvert][self.qhoriz].kill_entity(&(*i as usize));
                                    }
                                }
                            }
                            EntityType::Starbase => {
                                prout!("\n ***Friendly starbase at sector ({}, {}) destroyed. You murderer.", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.score.kill_starbase();
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                            },
                            EntityType::Unknown => {
                                prout!("\n *** ??? at sector ({}, {}) destroyed.", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.score.kill_unknown();
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                            },
                            EntityType::Tholian => {
                                prout!("\n ***Tholian at sector ({}, {}) destroyed. Good shot!", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.score.kill_tholian();
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                            },
                            EntityType::Planet => {
                                prout!("\n ***Planet at sector ({}, {}) destroyed. You murderer.", (torp_loc/10)+1, (torp_loc%10)+1);
                            },
                        }
                        // The torpedo has, of course, blown up.
                        break;
                    }
                    None => print!(", ({}, {})", (torp_loc/10)+1, (torp_loc%10)+1)
                }
            }
        }
    }


    pub fn deathray (&mut self) {
        if self.damage.deathray > 0.0 {
            prout!("[*Tactical*] Sir, the deathray's, like, damaged. I can't, like, fire it in this condition.");
            return
        }
        else if self.get_quadrant().search(EntityType::Klingon).len() == 0
            && self.get_quadrant().search(EntityType::Romulan).len() == 0
            && self.get_quadrant().search(EntityType::Tholian).len() == 0
            && self.get_quadrant().search(EntityType::Unknown).len() == 0 {
                prout!("[*Mr. Spock*] Captain, there are no enemies in this quadrant.");
                return
        }
        else if self.energy < 100.1 {
            prout!("[*Engineering*] Sir, we do na' have enough energy to fire that infernal thing.");
            return
        }

        if !get_yorn("[*Mr. Spock*] The deathray is still experimental. If we use it there is a large chance that the Enterprise will be destroyed. Are you sure you want to proceed?\n> ") {
            return
        }

        prout!("[*Mr. Spock*] As you wish.\n");
        slow_prout("WHOOee ... WHOOee ... WHOOee ... WHOOee");
        prout!("The crew scrambles in emergency preparation.");
        prout!("Spock and Scotty ready the deathray and prepare to channel all the ship's power to the device.");
        
        prout!("[*Mr. Spock*] Preparations are complete, captain.");
        prout!("[*Cpt. Kirk*] Fire!");

        slow_prout("WHIRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR");
        match thread_rng().gen_range(0..16) {
            0..=10 => {
                slow_prout("[*Mr. Sulu*] Captain, it's working!");
                for entity in self.get_quadrant().entities {
                    match entity.0 {
                        EntityType::Star => continue,
                        EntityType::Starbase => continue,
                        EntityType::Klingon 
                            | EntityType::Romulan 
                            | EntityType::Unknown 
                            | EntityType::Tholian => self.kill_enemy(&self.qvert.clone(), &mut self.qhoriz.clone(), &entity.1),
                        EntityType::Planet => continue,
                        EntityType::BlackHole => continue,
                    }
                }
                if random::<f64>() > 0.06 {
                    prout!("[*Mr. Spock*] Captain, the experimental deathray has been rendered inoperable.");
                    self.damage.deathray = 40.0;
                }
            },
            11 | 12 => {
                print!("[*Mr. Sulu*]");
                slow_prout("Captain, it's working!");
                extra_slow_prout("    ");

                print!("{}", Fg(Red));
                slow_prout("***RED ALERT! RED ALERT!***");
                slow_prout("***WARP CORE BREACH IMMINENT!***");
                slow_prout("***RED ALERT! RED A*L********************************");
                prout!("*********************** BOOOM ***********************");
                prout!("*****************************************************");
                print!("{}", Fg(Reset));
                self.death_reason = DeathReason::MaximumEntropy;
            },
            13 => {
                slow_prout("[*Mr. Sulu*] Captain, Yagsdsadfagag, brascscha!\n");
                prout!("[*Lt. Uhura*] Graeeek! Graeeek!\n");
                prout!("[*Mr. Spock*] Fascinating! It would seem that all the humans aboard have been transformed into strange mutations.");
                prout!("[*Mr. Spock*] Thankfully, Vulcans do not appear to be affected.\n");
                prout!("[*Cpt. Kirk*] Raauuch!");
                self.death_reason = DeathReason::Transformation;
            },
            14 => {
                slow_prout("[*Mr. Sulu*] Captain, it's working!");
                extra_slow_prout("    \n");
                slow_prout("[*Mr. Spock*] Captain, I am getting some illogical sensor readings.");
                slow_prout("[Cont.] There appears to be a wormhole nearby, but that's impossible; the starcharts do not show any in this area.");
                slow_prout("[Cont.] Interesting... there appears to be a massive cube-shaped ship coming through it.");
                slow_prout("[*Mr. Sulu*] Look at the size of that thing!");

                extra_slow_prout("      ");
                slow_prout("[*Lt. Uhura*] Captain, it's hailing us.");
                slow_prout("*click* We are the Borg. Lower your shields and surrender your ship. Your biological and technological distinctiveness will be added to our own. Your culture will adapt to service us. Resistance is futile");
                self.death_reason = DeathReason::Borg;
            },
            15 => {
                slow_prout("[*Mr. Sulu*] Um... Captain, it appears to be making tribbles?");
                self.death_reason = DeathReason::Tribble;
            },
            _ => {
                panic!("This should be inaccessible!")
            }
        }
    }
}