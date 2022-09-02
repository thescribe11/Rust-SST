#[derive(Clone, PartialEq, Debug)]
pub enum Event {  // Time-dependent events. Add more as necessary
    TractorBeam(f64),
    StarbaseAttack(f64, (usize, usize)),
    StarbaseDestroy(f64, (usize, usize)),
    Supernova(f64),
}