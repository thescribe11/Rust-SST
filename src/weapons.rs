use rand::random;

use crate::{input, io::get_args, structs::EntityType};

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
                        println!("Sir, I can't fire \"{}\" torpedoes.", x);
                        return;
                    }
                }
            }
        };

        // Check for invalid cases
        if to_fire > self.torpedoes {
            println!("[*Armory*] What do you think we are, the Bank of Ferenginar?");
            return;
        } else if to_fire == 0 {
            return;
        } else if to_fire > self.torpedoes {
            println!("[*Armory*] We can't fire that many sir; we only {} left.", &self.torpedoes);
            return
        } else if to_fire > 3 {
            println!("[*Armory*] Sir, we can only fire three at a time; any more and we would melt the tubes!")
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
                            println!("[*Armory*] Sir, that doesn't make sense.");
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
                    println!("\nTorpedo misses.");
                    break;
                }

                // Process impact
                match self.get_quadrant().get_entity(torp_loc as usize) {
                    Some((t, loc, he, al)) => {
                        match t {
                            EntityType::BlackHole => {
                                println!("\n Torpedo swallowed by black hole.");
                            },
                            EntityType::Klingon => {  // Klingons are always destroyed by torpedoes, although I might want to change this later.
                                println!("\n ***Klingon at sector ({}, {}) destroyed.", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                                self.score.kill_klingon();
                                self.klingons -=1;
                            },
                            EntityType::Romulan => {
                                match random::<u8>() {
                                    0..=200 => {  // Romulan dies
                                        println!("\n ***Romulan at sector ({}, {}) destroyed.", (torp_loc/10)+1, (torp_loc%10)+1);
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
                                println!("\n ***Torpedo impacts star at sector ({}, {}), causing it to go nova.", (torp_loc/10)+1, (torp_loc%10)+1);
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
                                                println!("{}", self.get_quadrant().sectors[*i as usize]);
                                                panic!("Somehow a corrupted value has gotten into the sector map.")
                                            }
                                        }
                                        self.quadrants[self.qvert][self.qhoriz].kill_entity(&(*i as usize));
                                    }
                                }
                            }
                            EntityType::Starbase => {
                                println!("\n ***Friendly starbase at sector ({}, {}) destroyed. You murderer.", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.score.kill_starbase();
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                            },
                            EntityType::Unknown => {
                                println!("\n *** ??? at sector ({}, {}) destroyed.", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.score.kill_unknown();
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                            },
                            EntityType::Tholian => {
                                println!("\n ***Tholian at sector ({}, {}) destroyed. Good shot!", (torp_loc/10)+1, (torp_loc%10)+1);
                                self.score.kill_tholian();
                                self.quadrants[self.qvert][self.qhoriz].kill_entity(&(torp_loc as usize));
                            },
                            EntityType::Planet => {
                                println!("\n ***Planet at sector ({}, {}) destroyed. You murderer.", (torp_loc/10)+1, (torp_loc%10)+1);
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
}