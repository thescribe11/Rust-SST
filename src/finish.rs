use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum DeathReason {
    MaximumEntropy,  // Warp core breach
    NegativeSpaceWedgie,  // Eldritch abomination
    Kaboom,   // Ship blown up
    Tribble,  // Tribble infestation
    Stranded, // You are stranded on a planet, leaving Mr. Spock in charge
    TimeUp,   // You don't stop the invasion in time
    NoAir,    // Life support reserves depleted
    NoGas,    // No fuel
    Transformation,  // Mutations
    Borg,     // We are the Borg. You will be assimilated. Resistance is futile.
    Dalek,    // EXTERMINATE!
    EventHorizon,  // Crushed by a black hole
    SelfDestruct,  // Goodbye, cruel world!
    GalaxyEdge,       // Your navigation is abominable.

    None, // Still alive
}