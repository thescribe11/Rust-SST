#[derive(Clone, PartialEq, Debug)]
pub enum Event {  // Time-dependent events. Add more as necessary
    TractorBeam(f64),
    StarbaseAttack(f64, f64, [usize; 2]),
    StarbaseDestroy(f64, [usize; 2]),
    Supernova(f64),
    None,
}