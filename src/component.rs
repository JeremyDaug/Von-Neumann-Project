use std::collections::HashMap;

/// # Component
/// 
/// A Component is the building block of functionality for constructs. 
/// Everything they do is attached to a specific component they have.
/// 
/// TODO break this up into separate subclasses based around functionality and either turn this into a trait or make it a wrapper for the others.
pub struct Component {
    // identity stuff
    /// The unique id for the component. No relation to other Ids.
    pub id: usize,
    /// The name of the component.
    pub name: String,
    // TODO add image token here.

    // Construction section
    /// The goods which go into making this component.
    pub parts: HashMap<usize, f64>,
    /// How much worker time is needed to install the component.
    /// To uninstall takes an equal amount of time.
    pub installation_time: f64,

    // Structure cost section
    /// The size cost of the component.
    pub size_cost: f64,
    /// The component is only placeable on the surface of a construct.
    pub surface_only: bool,

    // Energy Part
    /// The constant energy drain needed for maintenance.
    /// All of this is converted into thermal energy at the end.
    pub energy_minimum: f64,
    /// The energy cost of the component. Needed equaly across it's work.
    pub energy_cost: f64,
    /// When paying the energy cost, this 
    /// thermal_waste * energy_cost = thermal energy gained.
    pub thermal_waste: f64,

    // Factory and assembly part
    /// The recipe the factory/assembly can use.
    pub recipe_types: Vec<usize>,
    /// The goods which the component consumes, all of these need to be met
    /// for it to work.
    pub consumes: HashMap<usize, f64>,
    /// The goods which the component produces each term.
    pub produces: HashMap<usize, f64>,

    // Mining Part
    /// What materials(s) it extracts
    pub extracts: Vec<usize>,
    /// How many kg it can extract per time unit (1,000s)
    pub rate: f64,

    // Worker Section
    /// Whether the component grants a worker to whatever is installed.
    pub gives_worker: bool,

    // Storage Capability
    /// Storage space is how much space the component can store.
    /// Note: This is not needed 
    pub storage_space: f64,

    // Self-propelling Engine Section
    /// Whether the engine uses a reaction mass or not.
    /// If not, then it's likely either a Photon Rocket or using supertech 
    /// reactionless engines.
    pub reactive_engine: bool,
    /// The mass expended per second kg/s for thrust of the engine.
    /// Any mass expended for fuel is either destroyed or added to the
    /// 'interplanetary medium', thus allowing it to be reharvested.
    pub mass_flow: f64,
    /// The velocity of the exhaust upon leaving the engine.
    /// 
    /// Mass_flow (kg/s) * Exhaust Velocity(m/s) gives the total force 
    /// in N (kg m / s2).
    pub exhaust_velocity: f64,

    // light engine section
    /// The maximum raw energy consumption of a light engine measured in
    /// Watts.
    pub max_consumption: f64,
    /// Of all the energy consumed here, how much is converted into thrust.
    /// All energy not made into thrust is converted into thermal energy.
    pub thruster_efficiency: f64,
}