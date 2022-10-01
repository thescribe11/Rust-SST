use rand::{random, thread_rng, Rng};
use termion::color::{Fg, Red, Reset};
use crate::constants::KLINGON_KO_TIME;
use crate::io::{wait, ControlMode, abbrev};
use crate::prout;
use crate::structs::{Alignment, Health};
use supports_unicode::Stream;

use crate::{finish::DeathReason, input, io::{slow_prout, get_args, get_yorn, SLOW}, structs::EntityType};

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
        let solution: Vec<i8> = _d.into_iter().map(|x| match x%9 {
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
            self.torpedoes -= 1;
            print!("\nTrack for torpedo #{}: ({}, {})", &torp_num, self.sloc/10+1, self.sloc%10+1);

            // Simulate torpedo
            loop {
                torp_loc += delta;
                if torp_loc < 0 || torp_loc > 99 {
                    prout!("\nTorpedo misses.");
                    break;
                }

                // Process impact
                match self.get_quadrant().get_entity(torp_loc as usize) {
                    Some((t, _, _, _)) => {
                        match t {
                            EntityType::BlackHole => {
                                prout!("\n Torpedo swallowed by black hole.");
                            },
                            EntityType::Klingon => {  // Klingons are always destroyed by torpedoes, although I might want to change this later.
                                prout!("\n ***Klingon at sector ({}, {}) destroyed.", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                                self.score.kill_klingon();
                                self.time_remaining += KLINGON_KO_TIME;
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
                                        self.quadrants[self.qvert][self.qhoriz].damage_entity(&(torp_loc as usize), 500.0);
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


    /// Fire the experimental death ray!
    /// 
    /// Returns:
    ///     - `true` if successful
    ///     - `false` otherwise
    pub fn deathray (&mut self) -> bool {
        if self.damage.deathray > 0.0 {
            prout!("[*Tactical*] Sir, the deathray's, like, damaged. I can't, like, fire it in this condition.");
            return false
        }
        else if self.get_quadrant().search(EntityType::Klingon).len() == 0
            && self.get_quadrant().search(EntityType::Romulan).len() == 0
            && self.get_quadrant().search(EntityType::Tholian).len() == 0
            && self.get_quadrant().search(EntityType::Unknown).len() == 0 {
                prout!("[*Mr. Spock*] Captain, there are no enemies in this quadrant to fire at.");
                return false
        }
        else if self.energy < 100.1 {
            prout!("[*Engineering*] Sir, we donna' have enough energy to fire that infernal thing.");
            return false
        }

        if !get_yorn("[*Mr. Spock*] The deathray is still experimental. If we use it there is a large chance that the Enterprise will be destroyed. Are you sure you want to proceed?\n> ") {
            return false
        }

        prout!("[*Mr. Spock*] As you wish.\n");
        slow_prout("WHOOee ... WHOOee ... WHOOee ... WHOOee", SLOW, true);
        prout!("The crew scrambles in emergency preparation.");
        prout!("Spock and Scotty ready the deathray and prepare to channel all the ship's power to the device.\n");
        
        prout!("[*Mr. Spock*] Preparations are complete, captain.");
        prout!("[*Cpt. Kirk*] Fire!");

        slow_prout("WHIRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR", SLOW, true);
        match thread_rng().gen_range(0..18) {
            0..=10 => {
                slow_prout("[*Mr. Sulu*] Captain, it's working!", SLOW, true);
                for entity in self.get_quadrant().entities {
                    match entity.0 {
                        EntityType::Star => continue,
                        EntityType::Starbase => continue,
                        EntityType::Klingon 
                            | EntityType::Romulan 
                            | EntityType::Unknown 
                            | EntityType::Tholian => self.kill_enemy(self.qvert, self.qhoriz, entity.1),
                        EntityType::Planet => continue,
                        EntityType::BlackHole => continue,
                    }
                }
                if random::<f64>() > 0.06 {
                    prout!("[*Mr. Spock*] Captain, the experimental deathray has been rendered inoperable.");
                    self.damage.deathray = 40.0;
                }
                return true
            },
            11 | 12 => {
                print!("[*Mr. Sulu*]");
                slow_prout("Captain, it's working!", SLOW, true);
                wait(4);

                print!("{}", Fg(Red));
                slow_prout("***RED ALERT! RED ALERT!***", SLOW, true);
                slow_prout("***WARP CORE BREACH IMMINENT!***", SLOW, true);
                slow_prout("***RED ALERT! RED A*L********************************", SLOW, true);
                prout!("*********************** BOOOM ***********************");
                prout!("*****************************************************");
                print!("{}", Fg(Reset));
                self.die(DeathReason::MaximumEntropy);
            },
            13 => {
                slow_prout("[*Mr. Sulu*] Captain, Yagsdsadfagag, brascscha!\n", SLOW, true);
                prout!("[*Lt. Uhura*] Graeeek! Graeeek!\n");
                prout!("[*Mr. Spock*] Fascinating! It would seem that all the humans aboard have been transformed into strange mutations.");
                prout!("[*Mr. Spock*] Thankfully, Vulcans do not appear to be affected.\n");
                prout!("[*Cpt. Kirk*] Raauuch!");
                wait(3);
                self.die(DeathReason::Transformation);
            },
            14 => {
                slow_prout("[*Mr. Sulu*] Captain, it's working!", SLOW, true);
                wait(5);
                prout!();
                slow_prout("[*Mr. Spock*] Captain, I am getting some illogical sensor readings.", SLOW, true);
                slow_prout("[Cont.] There appears to be a wormhole nearby, but that's impossible; the starcharts do not show any in this area.", SLOW, true);
                slow_prout("[Cont.] Interesting... there appears to be a massive cube-shaped ship coming through it.", SLOW, true);
                slow_prout("[*Mr. Sulu*] Look at the size of that thing!", SLOW, true);

                wait(4);
                prout!();
                slow_prout("[*Lt. Uhura*] Captain, it's hailing us.", SLOW, true);
                slow_prout("*click* We are the Borg. Lower your shields and surrender your ship. Your biological and technological distinctiveness will be added to our own. Your culture will adapt to service us. Resistance is futile.", SLOW, true);
                wait(1);
                self.die(DeathReason::Borg);
            },
            15 => {
                slow_prout("[*Mr. Sulu*] Um... Captain, it appears to be making tribbles?", SLOW, true);
                wait(1);
                self.die(DeathReason::Tribble);
            },
            16 | 17 => {
                if supports_unicode::on(Stream::Stdout) {
                    slow_prout("[*Mr. Sulu*] That's weird. It ḋ̵͉͊oesn't app̞ear tǫ be do̷̮͑̍ing anythH̵̤̙̥͇̚Ȩ̷̳̣͖̊ ̶̩̻͍͎̒C̴̖͔͈͗̓̍ͅO̶̢̦̳͓̅̑̈͠M̷̢͠È̴̙͌S̶̢͓̗̃͆́ GAAAAAAH AAAA GAH HELP PLEaD̶̡͍̖͍̫̲̤̔̔̏͒̍̒̋̐̕̚Ǒ̵̡̥͍͖̭̅͒̀̈́́́̆́̑͘̚͠͝ͅ ̶̢̧̦̰̝̬̝̟̰̩̝͎̹͐̋̂̐̇̏̂̐́̈́̐͌̇̾͘͠͠N̸̘̹͖̭̪̺̪͙̟͔̻͎͗̍͋͑͒̇̓̌̓̃̃̂̕͝Ơ̵͖̻̠̜͇͇̻̿̉̽͒̆̆̑̿̿͋̔̇T̷̗̤̭̞͙͚̆̀͋̉ͅ ̸̼̹̦͖̱͔͖̮̩͇̖͎̼̦̯̠̊̈̉͆̓̅̉͝͠ͅR̴̡̼͙͚͚̟̗͇͓̻̳̰͙͇̺̈́̑̽͛̇̊̓͒͘͘͘͝E̵͙̋S̷̨͝Ȉ̸̫Ṡ̶̥T̷̗̈́ GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH!", SLOW, true);
                } else {
                    prout!("Eldritch abominations are incompatible with this old hunk of junk. Get out of the stone age and buy a computer that supports unicode.");
                    wait(3);
                }
                
                self.die(DeathReason::NegativeSpaceWedgie)
            }
            _ => {
                panic!("This should be inaccessible!")
            }
        }

        return false  // You've got 99 problems, and the Klingons are no longer one of them.
    }


    /// Fire phasers
    /// 
    /// `mode`: Fire control mode.
    ///     - `Auto`: Computer automatically chooses targets
    ///     - `Manual`: Manually specify targeting solution
    ///     - `Undefined`: The user has yet to specify the targeting solution
    /// 
    /// `targets`: Sectors to fire at.
    pub fn phasers (&mut self, mut mode: ControlMode, mut total_energy: f64) {
        let mut targets: Vec<(usize, f64, EntityType, f64)> = Vec::new();
        let mut overheat: bool = false;
        let mut randint = rand::thread_rng();

        if self.damage.phasers > 0.0 {
            prout!("[*Tactical*] Sorry bro, but the phasers are, like, damaged. I can't, like, fire them until they're repaired.");
            return;
        }

        if mode == ControlMode::Undefined {
            let raw: String;
            if self.damage.computer < 0.5 {
                raw = input("Automatic or Manual targeting? ");
            } else {
                prout!("[*Tactical*] Captain, the compewter's, like, damaged; I can only give you, like, manual targetting.\n");
                raw = String::from("manual");
            };
            let raw = raw.to_ascii_lowercase();

            if abbrev(&raw, "a", "automatic") {
                mode = ControlMode::Auto;
            }
            else if abbrev(&raw, "m", "manual") {
                mode = ControlMode::Manual;
            }
            else {
                prout!("[*Tactical*] Sir, that, like, doesn't make any sense.");
                return;
            }
        }

        if total_energy.is_nan() && mode == ControlMode::Auto {
            let raw = input("Energy to fire: ");
            let to_fire = match raw.parse::<f64>() {
                Ok(v) => v,
                Err(_) => {
                    prout!("[*Tactical*] Sir, that, like, doesn't make any sense.");
                    return;
                }
            };

            // For safety purposes, you cannot fire amounts of energy which would leave you at less than 200 energy.
            if self.shield_status && self.energy - to_fire < 350.0 {
                prout!("[*Engineering*] Captain, we donna ha' enough power ta fire the phasers wi' high-speed shield control.");
                if self.energy - to_fire > 200.0 {
                    prout!("[*Engineering*] ...that said, we *could* do it if we manually lower the shields.");
                }
                return;
            }
            else if self.energy - to_fire < 200.0 {
                prout!("[*Engineering*] Captain, we donna ha' the power to do that.");
                return;
            }

            // You can't use the phasers to create energy ex nihilo
            if to_fire < 0.0 {
                prout!("[*Tactical*] Captain, we, like, can't fire negative amounts of, like, energy.");
                return;
            }
            total_energy = to_fire;
        }

        let enemies: Vec<(EntityType, usize, Health, Alignment)> = self.get_quadrant().enemies();
        if mode == ControlMode::Auto {
            let average: f64 = total_energy / enemies.len() as f64;
            for i in enemies {
                targets.push(
                    (i.1.clone(), calc_ablation(i.1, self.sloc, average), i.0, average)
                )
            }
        }
        else if mode == ControlMode::Manual {
            for enemy in 0..enemies.len() {
                prout!("\n* TARGET: {} at {}-{}", enemies[enemy].0, (enemies[enemy].1 / 10)+1, (enemies[enemy].1 % 10)+1);
                let to_fire: f64 = match input("How much energy to fire? ").parse() {
                    Ok(v) => {
                        if v < 0.0 {
                            prout!("[*Tactical*] Captain, we, like, can't fire negative amounts of, like, energy.");
                            return;
                        }
                        if self.shield_status && self.energy - v < 350.0 {
                            prout!("[*Engineering*] Captain, we donna ha' enough power ta fire the phasers wi' high-speed shield control.");
                            if self.energy - v > 200.0 {
                                prout!("[*Engineering*] ...that said, we *could* do it if we manually lower the shields.");
                            }
                            return;
                        }
                        else if self.energy - v < 200.0 {
                            prout!("[*Engineering*] Captain, we donna ha' the power to do that.");
                            return;
                        }
                        if [420.0, 69.0].contains(&v) {
                            prout!("Nice!");
                        }
                        v
                    },
                    Err(_) => {
                        prout!("[*Tactical*] Captain, that, like, doesn't make any sense.");
                        return;
                    }
                };

                targets.push((enemies[enemy].1, calc_ablation(self.sloc, enemies[enemy].1, to_fire), enemies[enemy].0, to_fire));
            }
        }

        // Now that we've established the firing solution, let's cry havoc and let slip the dogs of war!
        prout!();  // First off, visually seperate the firing solution from the actual combat.
        for i in targets {
            if i.1 > 500.0 && randint.gen_range(0..100) > 75 {  // Firing lots of energy may cause the phasers to overheat.
                overheat = true;
            }
            prout!("{:.2} unit hit on {} at sector {}-{}.", &i.1, i.2, i.0/10 + 1, i.0 % 10 + 1);

            match self.quadrants[self.qvert][self.qhoriz].damage_entity(&i.0, i.1) {
                Some(v) => {
                    prout!("*** {} at sector {}-{} destroyed!", v, (i.0 / 10)+1, (i.0 % 10)+1);
                    self.kill_enemy(self.qvert, self.qhoriz, i.0);
                },
                None => {}   
            }

            self.energy -= i.3;
            if self.energy <= 0.0 {
                self.alive = false;
                self.death_reason = DeathReason::NoGas;
            }
        }

        if overheat {
            prout!("\n[*Tactical*] Rad! ...uh, captain, the phasers have, like, overheated.");
            self.damage.phasers += randint.gen_range(0.1..1.0);
        }
    }
}


pub fn calc_ablation (from: usize, to: usize, energy: f64) -> f64 {
    let mut randint = rand::thread_rng();
    let to_return = energy - (randint.gen_range(0..10) as f64
        + (  // Hypotenuse squared, to simulate ablation
            usize::abs_diff(from % 10, to % 10).pow(2) as f64
            + usize::abs_diff(from / 10, to / 10).pow(2) as f64
        )
    );
    if to_return < 0.0 {
        return 0.0
    }
    return to_return
}