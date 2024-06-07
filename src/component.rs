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
}