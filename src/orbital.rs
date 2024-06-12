//! Orbital Body handles geometry, transformation, movement, and similar activities.

use core::fmt;
use std::{collections::HashMap, f64::consts::{PI, TAU}, process::Child};

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
#[derive(Clone, Debug)]
pub struct Orbital {
    /// The id of the orbital object, should be shared with equivalent bodies like
    /// bodies, satellites, fleets, and bases.
    pub id: usize,

    /// Whether the body is fixed or not. If fixed, then it will not change 
    /// it's position due to outer forces. It will still be effected by 
    pub is_fixed: bool,
    
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
    /// Orbital bodies which are in an orbit of this body.
    pub orbital_children: Vec<usize>,
    /// The ids of the other bodies which have a noteworthy level of influence 
    /// on it's motion. (should be at least 1% of the influence of the orbital 
    /// parent, or at least 20% of total force if body has no parent.)
    pub primary_influences: Vec<usize>,

    /// Sphere of Influence, a helper datum which helps the system know when an
    /// object should change it's orbital_parent and/or primary influences.
    pub sphere_of_influence: f64,

    /// WHether the orbital body has collision or not.
    /// 
    /// Note that only if both objects don't have collision will they not 
    /// collide. If one object has collision and the other doesn't, then 
    /// a collision will still occur. This typically occurs for something
    /// like a fleet coming into contact with a planet.
    pub has_collision: bool,
}

impl Orbital {
    // Builders and modifier chains.
    /// # New
    /// 
    /// Makes a new default Orbital
    /// 
    /// Mass and thermal energy is set to 1.
    /// is_fixed is true.
    /// id is 0
    /// all other values are either 0.0 or empty.
    pub fn new() -> Orbital {
        Orbital {
            id: 0,
            is_fixed: true,
            mass: 1.0,
            thermal_energy: 1.0,
            rad: 0.0,
            x: 0.0,
            y: 0.0,
            rot: 0.0,
            vx: 0.0,
            vy: 0.0,
            vrot: 0.0,
            thermal_balance: 0.0,
            orbital_parent: None,
            primary_influences: vec![],
            sphere_of_influence: 0.0,
            has_collision: true,
            orbital_children: vec![],
        }
    }

    /// # Include Matter
    /// 
    /// Adds the matter given, consumes original orbital.
    pub fn with_matter(mut self, matter: HashMap<usize, f64>) -> Orbital {
        for (id, quant) in matter.into_iter() {
            self.mass += quant;
        }
        self
    }

    pub fn with_mass(mut self, mass: f64) -> Orbital {
        self.mass = mass;
        self
    }

    pub fn with_angular_velocity(mut self, vrot: f64) -> Orbital {
        self.vrot = vrot;
        self
    }

    pub fn with_velocity(mut self, vx: f64, vy: f64) -> Orbital {
        self.vx = vx;
        self.vy = vy;
        self
    }

    pub fn set_thermal_energy(mut self , thermal_energy: f64) -> Orbital {
        debug_assert!(thermal_energy > 0.0, "Thermal energy must be positive value.");
        self.thermal_energy = thermal_energy;
        self
    }

    /// # Identifier
    /// 
    /// Consuming Id setter.
    pub fn identifier(mut self, id: usize) -> Orbital {
        self.id = id;
        self
    }

    /// # Rotation
    /// 
    /// Consuming rotation setter.
    pub fn rotation(mut self, rot: f64) -> Orbital {
        self.rot = rot;
        self
    }

    /// # Position
    /// 
    /// Consumes Orbital and sets positoin.
    pub fn position(mut self, x: f64, y: f64) -> Orbital {
        self.x = x;
        self.y = y;
        self
    }
    
    /// # With Parent
    /// 
    /// Consumes orbital and sets parent.
    /// 
    /// Cannot set parent to None.
    fn with_parent(mut self, id: usize) -> Orbital {
        self.orbital_parent = Some(id);
        self
    }

    /// # Add Child
    /// 
    /// Adds child to this orbital body, creating it in a circular orbit with parameters given.
    /// start_angle is in radians and defines it's rotational location relative to the X axis.
    /// 
    /// Will return Err if the child created is too big, outside of Sphere of Influence,
    /// or so close that they are touching.
    /// 
    /// Child bodies are built assuming that 1 m^2 = 5000 kg.
    /// 
    /// TODO improve vector math being done by floating it off to better data storage.
    pub fn add_child(&mut self, id: usize, mass: f64, distance: f64, start_angle: f64, reverse_orbit: bool) -> Result<Orbital, ChildCreateErr> {
        if distance > self.sphere_of_influence {
            return Err(ChildCreateErr::OutOfSoI);
        } else if mass > self.mass {
            // TODO modify this to be a bit lower so that binary systems are harder to create.
            return Err(ChildCreateErr::TooMassive);
        }
        // calculate radius bassed off the standard 5000 kg / m^2
        let area = mass / 5000.0;
        let child_rad = (area / PI).sqrt();
        if (child_rad + self.rad) <= distance {
            return Err(ChildCreateErr::TooClose);
        }

        // create the rotated position based off of distance and start_angle
        let posx = distance * f64::cos(start_angle);
        let posy = distance * f64::sin(start_angle);
        let (finx, finy) = (posx + self.x, posy + self.y);
        // calculate the needed velocity to give a (near) circular orbit.
        // TODO this assumes the child is of negligible mass relative to the parent. Consider changing to be more flexible.
        let v = (G * self.mass / distance).sqrt();
        // the direction is a right angle from posx, posy, assume counterclockwise default.
        let norm = (posx/distance, posy/distance);
        let (vfinx, vfiny) = if reverse_orbit {
            (
                norm.1 * v,
                -norm.0 * v
            )
        } else {
            (
                -norm.1 * v,
                norm.0 * v
            )
        };


        // with all errors passed, build the child.
        let child = Orbital::new()
        .identifier(id)
        .with_mass(mass)
        .position(finx, finy)
        .with_velocity(vfinx, vfiny)
        .with_parent(self.id);

        self.orbital_children.push(id);
        Ok(child)
    }

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
            is_fixed: false,
            sphere_of_influence: self.sphere_of_influence,
            has_collision: true,
            orbital_children: vec![],
        }
    }

    /// # Minimum Influence
    /// 
    /// Based on the mass of the object, it gets the radius at which
    /// gravitational accelereation is at minimum 0.01 m / s^2.
    /// 
    /// Any other body within this limit is considered to be influenced as it's strong enough to be noticeable.
    pub fn min_influence(&self) -> f64 {
        let min_pull = 0.01;
        (min_pull / G / self.mass).sqrt()
    }

    /// # Distance Squared
    pub fn dist_sqrd(&self, other: &Orbital) -> f64 {
        let x = self.x - &other.x;
        let y = self.y - &other.y;
        x * x + y * y
    }
    
    /// # Distance
    /// 
    /// Calculates the distance bectween this orbital's position and anothers.
    pub fn distance_to(&self, other: &Orbital) -> f64 {
        self.dist_sqrd(other).sqrt()
    }

    /// # Relative Velocity
    /// 
    /// Get's the current body's velocity relative to another body.
    /// 
    /// Return is vx and vy respectively.
    pub fn relative_velocity(&self, other: &Orbital) -> (f64, f64) {
        (
            self.vx - other.vx,
            self.vy - other.vy
        )
    }

    /// # Speed
    /// 
    /// Given vX and vY, get the body's current speed overall.
    /// 
    /// IE the magnitude of vx and vy.
    pub fn speed(vx: f64, vy: f64) -> f64 {
        (vx * vx + vy * vy).sqrt()
    }
    
    /// # Calculate Sphere of Influence
    /// 
    /// Calculates the Sphere of influence of this body relative to another body.
    pub fn calculate_sphere_of_influence(&self, other: &Orbital) -> f64 {
        let distance = other.distance_to(&self);
        distance - (distance / ((self.mass / other.mass).sqrt() + 1.0))
    }

    /// # Escape Velocity
    /// 
    /// Calculates the speed needed to escape this body's pull, given a startng
    /// radius.
    pub fn escape_velocity(&self, radius: f64) -> f64 {
        (2.0 * G * self.mass / radius).sqrt()
    }

}

#[derive(Debug)]
pub enum ChildCreateErr {
    OutOfSoI,
    TooMassive,
    TooClose,
}

impl Error for ChildCreateErr {}

impl fmt::Display for ChildCreateErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChildCreateErr::OutOfSoI => write!(f, "Outside of body's Sphere of Influence."),
            ChildCreateErr::TooMassive => write!(f, "Child Body is too massive, must be smaller than the parent."),
            ChildCreateErr::TooClose => write!(f, "Child body is too close, they cannot start touching."),
        }
    }
}