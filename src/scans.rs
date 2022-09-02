use crate::structs::{EntityType, Alert};
use termion::color::{Blue, Fg, Green, Red, Reset, Yellow, LightBlue};


impl crate::structs::Universe {
    pub fn srscan (&mut self) {
        //! Perform a short-range sensor scan

        self.set_alert(Alert::Green);
        let quad = self.get_quadrant();
        if (quad.search(EntityType::Romulan).len() > 0)
        || quad.search(EntityType::Tholian).len() > 0 
        || quad.search(EntityType::Unknown).len() > 0
        || self.on_life_reserve {
            self.set_alert(Alert::Yellow);
        }
        if quad.search(EntityType::Klingon).len() > 0
        || quad.search(EntityType::Romulan).len() > 0 && self.ididit {
            self.set_alert(Alert::Red);
        }

        println!("    1 2 3 4 5 6 7 8 9 10");
        println!("  ┏━━━━━━━━━━━━━━━━━━━━━┓");

        if self.damage.srsensors == 0.0 { // Chart quadrant, but only if the short-range sensors are undamaged.
            self.charted[self.qvert][self.qhoriz] = true;
        }

        let mut viewable_coords: Vec<usize> = vec![self.sloc, self.sloc+10];
        if self.damage.srsensors > 0.0 {  // Limit the the player's vision to the Enterprise's immediate vicinity
            if self.sloc / 10 != 0 {
                viewable_coords.push(self.sloc-10);
                if self.sloc % 10 != 0 {
                    viewable_coords.extend_from_slice(&[self.sloc-11, self.sloc-1, self.sloc+9]);
                }
                if self.sloc % 10 != 9 {
                    viewable_coords.extend_from_slice(&[self.sloc-9, self.sloc+1, self.sloc+11]);
                }
            }
        }

        let mut index = 0;
        for vert in 0..10 {
            print!("{}{}", vert+1, match vert {
                0..=8 => " ┃",
                _ => "┃",
            });

            let mut printing: bool;
            
            for _horiz in 0..10 {
                printing = true;

                if self.damage.srsensors > 0.0 && !viewable_coords.contains(&index) {
                    printing = false;
                }

                if printing {
                    print!(" {}", match self.sector(&index) {
                        0 => String::from("."),
                        1 => String::from("*"),
                        2 => format!("{}B{}", Fg(Blue), Fg(Reset)),
                        3 => format!("{}K{}", Fg(Red), Fg(Reset)),
                        4 => format!("{}R{}", Fg(Green), Fg(Reset)),
                        5 => String::from(" "),
                        6 => format!("{}t{}", Fg(Red), Fg(Reset)),
                        7 => format!("{}?{}", Fg(Green), Fg(Reset)),
                        8 => {
                            if self.cloaked {
                                format!("{}E{}", Fg(LightBlue), Fg(Reset))
                            } else {
                                format!("{}E{}", Fg(Blue), Fg(Reset))
                            }
                        },
                        _ => panic!("It appears that the program has managed to put an impossible value in the sector srscan table. Please contact the developer with a bug report.")
                    });
                } else {
                    print!(" ?");
                }

                index += 1;
            }

            print!(" ┃");

            match vert {
                0 => println!(" Stardate:      {:.2}", self.stardate),
                1 => println!(" Condition:     {}{}", match self.alert() {
                    Alert::Red => format!("{}RED{}", Fg(Red), Fg(Reset)),
                    Alert::Yellow => format!("{}Yellow{}", Fg(Yellow), Fg(Reset)),
                    Alert::Green => format!("{}Green{}", Fg(Green), Fg(Reset)),
                }, match self.cloaked {
                    true => "; cloaked",
                    false => "",
                }),
                2 => println!(" Position:      Sector {} {} of quadrant {} {}", self.sloc/10+1, self.sloc%10+1, &self.qvert+1, &self.qhoriz+1),
                3 => println!(" Life Reserves: {}; reserves: {:.2} days", 
                    match self.on_life_reserve {
                        false => String::from("Active"),
                        true => format!("{}OFFLINE{}", Fg(Red), Fg(Reset)),
                    }, self.life_reserves
                ),
                4 => println!(" Warp Factor:   {}", self.warp_factor),
                5 => println!(" Energy:        {:.2}", self.energy),
                6 => println!(" Torpedoes:     {}", self.torpedoes),
                7 => println!(" Shields:       {}, {} energy remaining", match self.shield_status {
                    true => "UP",
                    false => "DOWN",
                }, self.shields),
                8 => println!(" Klingons:      {}", &self.klingons),
                9 => println!(" Time Left:     {:.2}", self.time_remaining),
                _ => println!()
            }
        }
        println!("  ┗━━━━━━━━━━━━━━━━━━━━━┛");
    }

    pub fn lrscan (&mut self) {
        //! Perform a long-range sensor scan.
        //! It won't give you fine details about
        //! quadrants, but you can get some basic
        //! information.

        if self.damage.lrsensors > 0.0 {
            println!("[*Mr. Spock*] Sir, the long range sensors are inoperable due to damage.");
            return;
        }

        println!("Long-range sensor scan for quadrant {} {}:", self.qvert+1, self.qhoriz+1);

        for i in self.qvert as i32-1..=self.qvert as i32+1 {
            for j in self.qhoriz as i32-1..=self.qhoriz as i32 +1 {
                if i<0 || i>7 || j<0 || j>7 {  // Galactic border
                    print!("   -1");
                    continue; // Don't try to chart out-of-bounds areas.
                }
                
                else if self.quadrants[i as usize][j as usize].is_supernova {  // Supernova
                    print!(" 1000")
                } else {  // Regular quadrant
                    let (k, b, s) = self.quadrants[i as usize][j as usize].poll_lrscan();
                    print!("  ");
                    if k > 0 {
                        print!("{}{}{}", Fg(Red), k, Fg(Reset));
                    } else {
                        print!("{}{}{}", Fg(Green), k, Fg(Reset));
                    }

                    if b > 0 {
                        print!("{}{}{}", Fg(Blue), b, Fg(Reset));
                    } else {
                        print!("{}", b);
                    }

                    print!("{}", s);
                }
                self.charted[i as usize][j as usize] = true;
            }
            println!();
        }
    }

    /// Print out a chart of the known galaxy.
    pub fn starchart (&self) {
        println!("     1   2   3   4   5   6   7   8");
        println!("  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");

        for vert in 0..8 {
            print!("{} ┃ ", &vert+1);

            for horiz in 0..8 {
                if self.charted[vert][horiz] {
                    let (k,b,s) = self.quadrants[vert][horiz].poll_lrscan();

                    if k > 0 {
                        print!("{}{}{}", Fg(Red), k, Fg(Reset));
                    } else {
                        print!("{}{}{}", Fg(Green), k, Fg(Reset));
                    }

                    if b > 0 {
                        print!("{}{}{}", Fg(Blue), b, Fg(Reset));
                    } else {
                        print!("{}", b);
                    }

                    print!("{} ", s);
                } else {
                    print!("{}???{} ", Fg(Yellow), Fg(Reset));
                }
            }

            println!("┃");
        }
        println!("  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
    }
}

pub fn get_vicinity (loc: usize) -> Vec<usize> {
    let mut vicinity: Vec<usize> = Vec::new();
    if loc % 10 != 0 {
        vicinity.push(loc - 1);
        if loc > 9 {
            vicinity.push(loc-11);
        }
        if loc < 90 {
            vicinity.push(loc+9);
        }
    }
    if loc % 10 != 9 {
        vicinity.push(loc + 1);
        if loc > 9 {
            vicinity.push(loc -9);
        }
        if loc < 90 {
            vicinity.push(loc+11);
        }
    }
    if loc > 9 {
        vicinity.push(loc-10);
    }
    if loc < 90 {
        vicinity.push(loc+10);
    }

    return vicinity;
}