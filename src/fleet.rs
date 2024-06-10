use std::collections::HashMap;

/// # Fleet
/// 
/// A fleet is a wrapper around constructs which are in motion. All 
/// constructs should be in a fleet at all times.
/// 
/// Fleets are Virtual if they have no constructs in them. Such empty fleets
/// can be used for 
pub struct Fleet {
    /// The ID of the fleet, should be shared with an Orbital.
    pub id: usize,

    /// The constructs in the fleet and their quantity.
    /// 
    /// Ships without blueprints have unique ids.
    pub constructs: HashMap<usize, usize>,

    /// The items stored in the fleet, broken up by the ship they're in as 
    /// well as good.
    pub storage: HashMap<usize, HashMap<usize, f64>>,
}