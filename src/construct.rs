use std::{collections::HashMap, f64::consts::PI};

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
    /// The id of the object this construct is attached to.
    pub parent: Option<usize>,
    /// The Constructs which are docked here.
    pub children: Vec<usize>,
    /// How many docking ports this construct has.
    pub docking_space: usize,

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

    // Control data
    /// Whether the construct has a control node or not.
    /// 
    /// Without a personality node it must be controlled on site by 
    /// another construct with a Personality Node or via a connection.
    pub has_personality_node: bool,
    /// Whether this construct is the center of authority or not.
    /// 
    /// If it is, then it recieves global orders no matter what.
    /// A nexus must have a personality node.
    pub is_nexus: bool,
    /// Whether the Construct has a construct to the rest of the network.
    /// 
    /// If it does not have a connection it cannot connect to the global 
    /// queue, and it will simply repeat it's last orders if possible.
    pub has_connection: bool,

    // queues and actions
    // Action Queue
    // Build Queue
    // Refit Queue
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