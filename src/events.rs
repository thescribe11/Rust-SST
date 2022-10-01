//! Stuff to do with events.

use crate::{structs::Universe, enums::Event};
use rand::prelude::*;

pub fn gen_starbase_attack (universe: &Universe) -> [Event; 2] {
    let mut randint: ThreadRng = rand::thread_rng();
    let threatened: Vec<[usize; 2]> = universe.get_threatened_starbases();
    let mut events = [Event::None, Event::None];
    let end_date = universe.stardate+1.0;
    let which = threatened[randint.gen_range(0..threatened.len())];
    events[0] = Event::StarbaseAttack(0.0, end_date.clone(), which.clone());
    events[1] = Event::StarbaseDestroy(end_date, which);
    return events;
}