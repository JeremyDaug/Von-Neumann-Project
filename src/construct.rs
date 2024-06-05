/// # Construct
/// 
/// A construct is anything the player can make. These may or may not be
/// mobile.
/// 
/// A fleet is made up of constructs. 
/// 
/// A body may only have 1 construct attached to it, but constructs can land on a planet.
pub struct Construct {
    /// The Id of the construct
    pub id: usize,
    /// The orbital location of the Construct.
    /// If a construct lands on a body, gets attached to a body, or joins with
    /// a fleet, the orbital ID is set to that of what it joins and mass is 
    /// added to it. The effects of the construct may also alter parts of a fleet.
    pub orbital: usize,
}