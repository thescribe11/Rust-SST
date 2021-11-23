#[derive(Clone, PartialEq)]
pub enum Event {  // Time-dependent events. Add more as necessary
    TractorBeam(f64),
    Supernova(f64),
    None,
}