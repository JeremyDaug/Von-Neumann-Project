use std::{collections::HashMap, f64::consts::PI};

use crate::{component::Component, recipe::RecipeType};

const STRUCTURE_BUILD_TIME: f64 = 1000.0;

/// # Construct
/// 
/// A construct is anything the player can make. These may or may not be
/// mobile.
/// 
/// A fleet is made up of constructs. 
/// 
/// A body may only have 1 construct attached to it, but constructs can land on a planet.
pub struct Construct {
    /// The Id of the construct, if it matches a blueprint, then it should 
    /// 
    pub id: usize,
    /// The id for the unique construct, so it can be stored and acted on directly.
    pub unique: usize,
    /// The id of the object this construct is attached to.
    pub parent: Option<usize>,
    /// The Constructs which are docked here.
    pub children: Vec<usize>,
    /// How many docking ports this construct has.
    pub docking_space: f64,

    /// The components in the construct which effects the body.
    pub components: HashMap<usize, f64>,
    /// The form factor for the construct, defines the gain rate of surface 
    /// space as the total structure grows.
    pub form_factor: FormFactor,
    /// The amount of structure available in the construct.
    /// 
    /// Each point costs 1 Building Space.
    pub total_structure: f64,
    /// The number of structure points used. Strictly internal pointss
    pub structure_used: f64,
    /// How much of the structure's surface area is used.
    pub surface_used: f64,
    /// The breakdown of components on the surafce of the construct.
    /// 
    /// Any Component not here is internal to the construct.
    pub surface_breakdown: HashMap<usize, f64>,

    // consolidated construction info
    /// All parts needed to create the construct.
    /// 
    /// This is the sum of all component parts as well as structural parts 
    /// needed to make the ship.
    /// 
    /// Measured in Kgs.
    pub all_parts: HashMap<usize, f64>,
    /// The amount of time needed to construct this, assuming only
    /// one worker is doing it.
    /// 
    /// Measured in Seconds.
    pub construction_time_needed: f64,

    // energy
    /// The minimum energy needed by the construct at all times.
    pub energy_minimum: f64,
    /// The total energy cost of all components if all are active at the same 
    /// time.
    /// 
    /// This will need to be broken up into parts.
    pub energy_cost: f64,
    /// How much total energy wast is produced when all components are active.
    /// 
    /// Will likely need to be broken up into parts.
    pub thermal_waste: f64,

    // storages
    /// The amonut of eneryg stored in MJs.
    pub energy_stored: f64,
    /// How much energy, in MJ the construct can store.
    pub energy_storage: f64,
    /// The amount of heat stored in MJs currently.
    pub heat_stored: f64,
    /// The amount of extra thermal energy the construct can store.
    pub thermal_capacitance: f64,
    /// The amount of matter that can be stored in this construct by Kg.
    pub storage_space: f64,
    /// The matter currently stored in the construct.
    pub stored_matter: HashMap<usize, f64>,

    // Factory parts
    /// The recipies set in the factory and the number of factories working on it.
    pub recipies: HashMap<usize, f64>,
    /// What each recipe is consuming.
    pub consuming: HashMap<usize, HashMap<usize, f64>>,
    /// What each recipe is producing.
    pub produces: HashMap<usize, HashMap<usize, f64>>,
    /// Specific recipe capacity.
    pub recipe_capacity: HashMap<RecipeType, f64>,

    // mining section
    /// What it's allowed to extract and at what rate.
    pub extracts: HashMap<usize, f64>,
    /// How much energy is needed to run all extractors at the same time.
    /// (for less than full, just divide normally.)
    pub extraction_energy_cost: f64,

    /// How many workers the construct hase.
    pub workers: f64,
    /// How much energy it costs to keep all workers active.
    pub worker_energy_cost: f64,

    // engines
    /// Whether the engine has a reaction mass or not.
    pub reactive_engine: bool,
    /// How much mass all engines on this vessel expend simultaniously.
    pub mass_flow: f64,
    /// The average Exhaust Velocity of all direct thrust engines.
    /// TODO: Need to touble check if thruster power is added, averaged, or whathaveyou.
    pub exhaust_velocity: f64,

    // Light Engine section
    /// How much energy all light thrusters can take.
    pub max_photon_energy_consumption: f64,
    /// The efficiency of the thrusters, whatever is lost to thrust
    /// becomes heat energy.
    pub thruster_efficiency: f64,

    // Light Sail Section
    /// The total size of all light sails.
    pub sail_size: f64,
    /// The amount of energy which strikes it and goes into moving the 
    /// construct. Any lost becomes thermal energy.
    pub sail_efficiency: f64, 
    /// How much of the sail is a mirror as opposed to absorber.
    pub mirror: f64,

    // queues and actions
    // Action Queue
    // Build Queue
    // Refit Queue

    // Possible additions
    // Control data
    // Whether the construct has a control node or not.
    // 
    // Without a personality node it must be controlled on site by 
    // another construct with a Personality Node or via a connection.
    //pub has_personality_node: bool,
    // Whether this construct is the center of authority or not.
    // 
    // If it is, then it recieves global orders no matter what.
    // A nexus must have a personality node.
    //pub is_nexus: bool,
    // Whether the Construct has a construct to the rest of the network.
    // 
    // If it does not have a connection it cannot connect to the global 
    // queue, and it will simply repeat it's last orders if possible.
    //pub has_connection: bool,
}

impl Construct {
    /// # Expand
    /// 
    /// Call this when adding space to a construct.
    pub fn expand(&mut self, space: f64) -> Result<(), ()> {

        Ok(())
    }

    /// # Surface Area
    /// 
    /// Calculates the surface area of the construct.
    /// This surface area is only the outside
    pub fn surface_area(&self) -> f64 {
        match &self.form_factor {
            FormFactor::Spherical => {
                (self.total_structure / PI).sqrt()
            },
            FormFactor::Rectangular(ratio) => {
                let x = (self.total_structure * ratio).sqrt();
                2.0 * x + 2.0 * (ratio * x)
            },
            FormFactor::Triangle(ratio) => {
                let h = (2.0 * self.total_structure / ratio).sqrt();
                let b = h * ratio;
                let side = (h*h + (b/2.0).powi(2)).sqrt();
                b + 2.0 * side
            },
            FormFactor::Ring => {
                0.0
            },
            FormFactor::Special(spec) => {
                match spec {
                    SpecialForm::OrbitalRing(_parent, _radius) => {
                        0.0
                    },
                }
            },
        }
    }

    /// # Collect components
    /// 
    /// Collects the components ,refreshing the capabilities of the 
    /// components into consolidated data.
    pub fn collect_components(&mut self, comps:&HashMap<usize, Component>) {
        // clear existing effects for good measure.
        self.clear_data();
        // base construction time
        self.construction_time_needed = self.total_structure * STRUCTURE_BUILD_TIME;
        // go through components, counting up their effects.
        for (component, quant) in self.components.iter() {
            let component = comps.get(component).unwrap();
            // space used
            self.structure_used += component.size_cost * quant;
            if component.surface_only {
                self.surface_used += component.size_cost * quant;
                self.surface_breakdown.insert(component.id, *quant);
            }
            // docking space
            if component.is_dock {
                self.docking_space += quant;
            }
            // construction time
            self.construction_time_needed += component.installation_time * quant;
            // overall energy costs
            if component.energy_minimum > 0.0 {
                self.energy_minimum += component.energy_minimum * quant;
            }
            self.energy_cost += component.energy_cost * quant;
            // overall thermal energy
            self.thermal_waste += component.thermal_waste * quant;
            self.thermal_capacitance += component.thermal_capacitance * quant;
            // factory work
            if component.recipe_types.is_some() {
                for rec_type in component.recipe_types.iter() {
                    match rec_type {
                        RecipeType::Refining => { 
                            self.recipe_capacity.entry(RecipeType::Refining)
                            .and_modify(|x| *x += 1.0)
                            .or_insert(1.0);
                        },
                        RecipeType::OilRefining => { 
                            self.recipe_capacity.entry(RecipeType::OilRefining)
                            .and_modify(|x| *x += 1.0)
                            .or_insert(1.0);
                        },
                        RecipeType::Fabricating => { 
                            self.recipe_capacity.entry(RecipeType::Fabricating)
                            .and_modify(|x| *x += 1.0)
                            .or_insert(1.0);
                        },
                        RecipeType::Assembly => { 
                            self.recipe_capacity.entry(RecipeType::Assembly)
                            .and_modify(|x| *x += 1.0)
                            .or_insert(1.0);
                        },
                        RecipeType::Printing => { 
                            self.recipe_capacity.entry(RecipeType::Printing)
                            .and_modify(|x| *x += 1.0)
                            .or_insert(1.0);
                        },
                    }
                }
            }
            // storage space
            self.storage_space += component.storage_space * quant;
            // extraction
            if component.extracts.len() > 0 {
                for item in component.extracts.iter() {
                    self.extracts.entry(*item)
                        .and_modify(|x| *x += component.extraction_rate * quant)
                        .or_insert(component.extraction_rate * quant);
                }
                self.extraction_energy_cost += component.energy_cost * quant;
            }
            // workers
            if component.gives_worker {
                self.workers += quant;
                self.worker_energy_cost += component.energy_cost;
            }
            // reactive engines
            if component.reactive_engine {
                let original_mass_flow = self.mass_flow;
                let new_mass_flow = component.mass_flow * quant;
                self.mass_flow += component.mass_flow * quant;
                self.exhaust_velocity = (self.exhaust_velocity * original_mass_flow + component.exhaust_velocity * new_mass_flow) / self.mass_flow;
            }
            // light engines
            if component.max_consumption > 0.0 {
                let old_con = self.max_photon_energy_consumption;
                let new_con = component.max_consumption * quant;
                self.max_photon_energy_consumption += component.max_consumption * quant;
                self.thruster_efficiency = (old_con * self.thruster_efficiency + new_con * component.thruster_efficiency) / self.max_photon_energy_consumption;
            }
            // sails
            if component.sail_size > 0.0 {
                let old_size = self.sail_size;
                let new_size = component.sail_size * quant;
                self.sail_size += new_size;
                self.sail_efficiency = (old_size * self.sail_efficiency + new_size * component.sail_efficiency) / self.sail_size;
            }
        }
    }

    /// # Clear Data
    /// 
    /// Clears out construct data so it can be reconstructed from component data.
    /// Also does not clear structural data as thoel points are added to directly.
    pub fn clear_data(&mut self) {
        self.docking_space = 1.0;
        // don't clear out components.
        // or total_structure.
        self.structure_used = 0.0;
        self.surface_used = 0.0;
        self.surface_breakdown.clear();
        self.all_parts.clear();
        self.construction_time_needed = 0.0;
        self.energy_minimum = 0.0;
        self.energy_cost = 0.0;
        self.thermal_waste = 0.0;
        self.energy_storage = 0.0;
        self.energy_storage = 0.0;
        self.thermal_capacitance = 0.0;
        self.storage_space = 0.0;
        // don't clear out stored matter for safety reasons.
        self.extracts.clear();
        self.extraction_energy_cost = 0.0;
        self.worker_energy_cost = 0.0;
        self.workers = 0.0;
        self.reactive_engine = false;
        self.mass_flow = 0.0;
        self.exhaust_velocity = 0.0;
        self.max_photon_energy_consumption = 0.0;
        self.thruster_efficiency = 0.0;
        self.sail_size = 0.0;
        self.sail_efficiency = 0.0;
        self.mirror = 0.0;
    }
}

/// # Form Factor 
/// 
/// The shape the construct takes.
pub enum FormFactor {
    /// The construct is structured as a sphere, 
    /// 
    /// The surface area is proportional to it's total structure
    /// IE.
    /// (total_structure / PI).sqrt()
    Spherical,
    /// The construct is rectangular in shape, with the ratio given by the
    /// contained value.
    /// 
    /// Depth / width = value.
    Rectangular (f64),
    /// The construct is triangular in shape, giving less space in return for
    /// more engine capacity.
    /// 
    /// Base / Height = value
    Triangle (f64),
    /// Ring structure, maximizes surface area via a ring structure,
    /// while separating an internal volume from an external volume.
    /// 
    /// This shape will be on hold.
    Ring,
    /// The construct is special, typically a megastructure or similar
    /// building.
    /// 
    /// Such constructs have their structure defined by their location
    /// and relations.
    Special (SpecialForm),
}

/// # Special Form
/// 
/// A unique subset of forms, typically for megastructures.
pub enum SpecialForm {
    /// The form is an ring around a planetary body.
    /// 
    /// Contains the ID of the body it's attached to and the radius
    /// of the ring.
    OrbitalRing(usize, f64)
}