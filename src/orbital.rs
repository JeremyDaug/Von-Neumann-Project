//! Orbital Body handles geometry, transformation, movement, and similar activities.

use std::{collections::HashMap, f64::consts::{PI, TAU}};

const G: f64 = 6.674e-11;

/// # Orbital data
/// 
/// Broken up into it's components, it uses geometric algebra methods to
/// assist where possible. Some additional factors are included however.
/// 
/// ## Notes
/// 
/// The Moment of Inertia for our Objects is assumed to be equal to that of a
/// disk with radius (rad). Giving it the angular energy 1/2 mass * rad^2.
/// 
/// Mass is in Kg
/// Energy is in J.
/// Distances used are measured in Meters.
/// Velocity measured in Meters per second.
/// Rotations use Radians.
/// Rotation speeds Rad / Sec.
pub struct Orbital {
    /// The id of the orbital object, should be shared with equivalent bodies like
    /// bodies, satellites, fleets, and bases.
    pub id: usize,
    
    // Isolated Data
    /// The rest mass of the body (scalar component). measured in Kg.
    /// Must be Positive Value.
    pub mass: f64,
    /// The thermal energy of the body. Measured in J.
    /// Must be Positive Value.
    pub thermal_energy: f64,
    /// The Body's Radius, used in multiple factors.
    /// 
    /// Represents the average distance from center of mass to surface.
    /// 
    /// Must be Positive Value
    pub rad: f64,

    // Positions
    /// X positional Component m (e_1)
    pub x: f64,
    /// y positional Component m (e_2)
    pub y: f64,
    /// The spacial rotational component (e_12) (it's orientation right now.)
    pub rot: f64,

    // Velocities
    /// The x velocity component m / sec (e_01)
    pub vx: f64,
    /// The y velocity component m / sec (e_02)
    pub vy: f64,
    /// The rotation speed component rad / sec (e_012)
    pub vrot: f64,
    /// The net change in energy, both ambient and internal.
    /// This should be 0 for any body in thermal equilibrium.
    /// 
    /// J / day
    /// 
    /// (collisions are not included here)
    pub thermal_balance: f64,

    // Relational Info.
    /// The orbital body which is considered to be this ones big sibling.
    /// If it has none, then there is no body which has an outsized 
    /// influence on the body.
    /// 
    /// This is not necissarily who the body is orbiting, merely the biggest 
    /// influence.
    /// 
    /// Orbital Parent threshold should be 50% >= of net gravitational forces.
    pub orbital_parent: Option<usize>,
    /// The ids of the other bodies which have a noteworthy level of influence 
    /// on it's motion. (should be at least 1% of the influence of the orbital 
    /// parent, or at least 20% of total force if body has no parent.)
    pub primary_influences: Vec<usize>,

    /// The detailed mass breakdown of a body, should sum up to self.mass.
    /// 
    /// The Key is the ID of the material in question.
    pub mass_breakdown: HashMap<usize, f64>
}

impl Orbital {
    // Derived Statistics
    /// # Rotational Eneregy
    /// 
    /// Measured in J.
    /// 
    /// Calculated from the mass, radius, and rotational speed.
    pub fn rotational_energy(&self) -> f64 {
        1.0 / 4.0 * (self.mass * self.rad.powi(2)) * self.vrot.powi(2)
    }

    /// # Rotational Momentum
    /// 
    /// The Rotational Momentum of the body.
    /// 
    /// Based on Radius and Rotational Velocity, mass, and radius.
    pub fn rotational_momentum(&self) -> f64 {
        PI * self.mass * self.vrot / TAU * self.rad.powi(2)
    }

    /// # Momentum
    /// 
    /// Gets the momentum of the body. Assumes that
    /// the velocity is relativet to the origin.
    pub fn momentum(&self) -> [f64; 2] {
        [
            self.mass * self.vx,
            self.mass * self.vy
        ]
    }

    /// # Mass Density
    /// 
    /// A measure of the density in kg / m^2
    /// 
    /// Based on mass and radius.
    pub fn mass_density(&self) -> f64 {
        self.mass / PI * self.rad.powi(2)
    }
    
    /// # Energy Density
    /// 
    /// A measure of the density of thermal energy in J / m^2.
    /// 
    /// Based on thermal_energy and Radius
    pub fn energy_density(&self) -> f64 {
        self.thermal_energy / PI * self.rad.powi(2)
    }

    /// # Tempurature
    /// 
    /// The Tempurature of the body, measured in K.
    /// 
    /// Calculated from Energy and Mass, with the Specific capacity being
    /// used = 1 J / g / K or 1000 J / kg / K
    pub fn tempurature(&self) -> f64 {
        self.thermal_energy / self.mass / 1000.0
    }

    // changes in size fns
    /// # Collapse Energy
    /// 
    /// The energy gained by an object collapsing into itself,
    /// converting potential energy into kinetic/thermal energy.
    /// 
    /// Measured in J.
    /// 
    /// Based on the starting radius and mass and 
    /// 
    /// Rotational Energy does not change from such a collapse, though 
    /// rotation speed should change.
    pub fn collapse_energy(&self, rad_new: f64) -> f64 {
        let u_i = -4.0 * G * self.mass.powf(2.0) / 25.0 / self.rad;
        let u_f = -4.0 * G * self.mass.powf(2.0) / 25.0 / rad_new;
        u_i - u_f
    }

    /// # Rotational Collapse
    /// 
    /// Gives the new rotation based on the new radius. 
    /// Rotational Momentum is conserved, so it should be a simple
    /// proportional change to the speed.
    /// 
    /// Measured in Rad/sec
    /// 
    /// Based on Rotational Momentum and Radius
    pub fn rotational_collapse(&self, rad_new: f64) -> f64 {
        self.vrot * self.rad.powi(2) / rad_new.powi(2)
    }

    /// # Change Radius
    /// 
    /// Alters the radius of the body, altering the rotation and thermal 
    /// energy appropriately.
    pub fn change_radius(&self, rad_new: f64) -> Orbital {
        let thermal_addition = self.collapse_energy(rad_new);
        let new_rot = self.rotational_collapse(rad_new);
        Orbital {
            id: self.id,
            mass: self.mass,
            thermal_energy: self.thermal_energy + thermal_addition,
            rad: rad_new,
            x: self.x,
            y: self.y,
            rot: self.rot,
            vx: self.vx,
            vy: self.vy,
            vrot: new_rot,
            thermal_balance: self.thermal_balance + thermal_addition,
            orbital_parent: self.orbital_parent,
            primary_influences: self.primary_influences.clone(),
            mass_breakdown: self.mass_breakdown.clone()
        }
    }

    /// # Minimum Influence
    /// 
    /// Based on the mass of the object, it gets the radius at which
    /// gravitational accelereation is at minimum 1 m / s^2.
    /// 
    /// Any other body within this limit is considered to be influenced as it's strong enough to be noticeable.
    pub fn min_influence(&self) -> f64 {
        let min_pull = 1.0;
        (min_pull / G / self.mass).sqrt()
    }
}