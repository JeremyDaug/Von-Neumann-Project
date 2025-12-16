use std::{collections::HashMap, default};

use bevy::math::{Vec3, Vec4, primitives::{Circle, Sphere}};

use crate::game::vector::Vector;

/// # Gravitational Constant
/// 
/// 6.67408e-11 m^3 kg^-1 s^-2
pub const G: f64 = 6.67408e-11; 

/// Seconds in a day.
pub const DAY_TO_SEC: f64 = 86400.0;
/// Astronomical Units (AU) to Meters (m)
pub const AU_TO_M: f64 = 149_597_870_700.0;

/// # Orbital
/// 
/// Orbital contains all of the motion data for bodies, fleets, platforms, and ships.
/// 
/// This presumes no forces beyond gravity are acting upon the body in question and that
/// they are point masses. Collision is a simple, intersecting radii deal.
/// 
/// ## Math Explanation
/// 
/// Most of the math is focused on 2 body physics, N-body physics is possible, but likely to be
/// highly simplified.
/// 
/// 
#[derive(Debug, Clone, Default)]
pub struct Orbital {
    /// The unique Id attached to the orbital. Shared with the body or construct who's
    /// orbit it defines.
    pub id: usize,

    /// Siblings, the other bodies which are large enough and close enough to have
    /// meaningful (0.1% influence or more) gravitational pull on the orbital.
    pub siblings: Vec<usize>,

    /// The radius of the body in meters, should always match body's radius.
    pub r: f64,

    // Vector information (geometric algebra style)
    /// The mass of the body in the orbital, should be a tight duplicate with 
    /// body.total_mass. Measured in Kg.
    /// 
    /// Kinda-sorta the Scalar value of our orbital vector data.
    pub m: f64,

    /// Inverted mass, to consolidate gravity calculations going forward.
    pub inv_m: f64,

    // position
    /// The position of the body in 3d
    pub t: Vector,

    // rotation
    /// The Rotation values, 3 Bivectors, or a Quaternion for losers.
    pub rot: Vector,
    /// The scalar part of our rotation vector.
    pub rot_scal: f64,

    /// The current translational velocity of the body.
    pub v: Vector,
    /// The current Rotational Velocity of the body.
    pub w: Vector,

    /// The Circle Mesh for the Orbital.
    /// Calculated as the log base 10 of the radius.
    pub sphere: Sphere
}

impl Orbital {
    /// # New
    /// 
    /// Simple new up function.
    pub fn new(id: usize) -> Self {
        Self {
            id,
            sphere: Sphere { radius: 1.0 },
            ..Default::default()
        }
    }

    pub fn with_radius(mut self, radius: f64) -> Self {
        self.r = radius;
        self.sphere = Sphere::default();
        self.sphere.radius = radius as f32;
        self
    }

    /// # With Mass
    /// 
    /// Sets the mass of the orbital object and it's inverse.
    /// 
    /// Mass is in Kg.
    pub fn with_mass(mut self, mass: f64) -> Self {
        self.m = mass;
        self.inv_m = 1.0 / mass;
        self
    }

    pub fn with_coords(mut self, x: f64, y: f64, z: f64) -> Self {
        self.t = Vector { x, y, z };
        self
    }

    pub fn with_velocity(mut self, x: f64, y: f64, z: f64) -> Self {
        self.v = Vector { x, y, z };
        self
    }

    pub fn with_rotation(mut self, scalar: f64, i: f64, j: f64, k: f64) -> Self {
        self.rot_scal = scalar;
        self.rot = Vector { x: i, y: j, z: k };
        self
    }

    pub fn with_rot_vel(mut self, w: f64) -> Self {
        self.vxy = w;
        self
    }

    /// # Angular Inertia
    /// 
    /// The current angular inertia of the object.
    /// 
    /// kg m^2
    pub fn angular_inertia(&self) -> f64 {
        2.0 / 5.0 * self.m * self.r.powi(2)
    }

    /// # Angular Momentum
    /// 
    /// The angular momentum of the body at this moment.
    /// 
    /// kg m^2 s^-1
    pub fn angular_momentum(&self) -> f64 {
        self.angular_inertia() * self.vxy
    }

    /// # Linear Momentum
    /// 
    /// The linear momentum of th ebody at this moment.
    /// 
    /// kg m s^-1
    pub fn linear_momentum(&self) -> Vector {
        Vector {
            x: self.vx * self.m,
            y: self.vy * self.m
        }
    }

    /// # Speed
    /// 
    /// Gets the magnitude of the velocity (speed)
    /// 
    /// m * s^-1
    pub fn speed_sqrd(&self) -> f64 {
        self.vx.powi(2) + self.vy.powi(2)
    }

    /// # Kinetic Energy
    /// 
    /// J or kg m^2 s^-2
    pub fn kinetic_energy(&self) -> f64 {
        0.5 * self.m * self.speed_sqrd()
    }

    /// # Rotational Energy
    /// 
    /// The rotational energy of the body at this moment.
    pub fn rotational_energy(&self) -> f64 {
        0.5 * self.angular_inertia() * self.vxy
    }

    /// # Gravitational Acceleration
    /// 
    /// The acceleration felt on another body d meters away.
    /// 
    /// m / s^2
    pub fn g_accel(&self, d: f64) -> f64 {
        G * self.m / d.powi(2)
    }

    /// # Position Vector
    /// 
    /// Gets the Position vector.
    pub fn position_vec(&self) -> Vector {
        Vector {
            x: self.tx,
            y: self.ty
        }
    }

    /// # Velocity Vector
    /// 
    /// Get the velocity vector.
    pub fn velocity_vec(&self) -> Vector {
        Vector {
            x: self.vx,
            y: self.vy
        }
    }

    /// # Relative Position
    /// 
    /// Gets the relative Position Vector of self -> other
    pub fn relative_position(&self, other: &Orbital) -> Vector {
        Vector {
            x: self.tx - other.tx,
            y: self.ty - other.ty
        }
    }

    /// # Relative Velocity
    /// 
    /// Gets the relative Velocity vector of self -> other
    pub fn relative_velocity(&self, other: &Orbital) -> Vector {
        Vector {
            x: self.vx - other.vx,
            y: self.vy - other.vy
        }
    }

    /// # Gravity Vector
    /// 
    /// Calculates the gravitational pull of the other object on
    /// this object, producing a vector of the acceleration.
    pub fn gravity_vector(&self, other: &Orbital) -> Vector {
        // println!("Gravity Vector");
        // get the self -> other vector
        let r_vector = self.relative_position(other);
        // get the norm of that vector.
        let norm = r_vector.normalize();
        // Get the acceleration of gravity at that point.
        let gravity = G * other.m / r_vector.m_sqrd();
        // multiply the norm by our gravitational force and return.
        let ret = norm.mult(gravity);
        // println!("Gravity Vec: {:?}", ret);
        ret
    }

    /// # Center of Attraction
    /// 
    /// Given all bodies which are pulling on this one, where is the relative 
    /// center of mass for everything pulling at this one.
    /// 
    /// This only covers this body's siblings.
    /// 
    /// Returns the absolute position of the mass, and it's calculated mass.
    pub fn center_of_attraction(&self, others: &HashMap<usize, Orbital>) -> (Vector, f64) {
        // println!("Center of Attraction");
        let mut center = Vector::default();
        let mut mass = 0.0;
        // iterate over siblings
        for other in self.siblings.iter().map(|x| others.get(x)
        .expect("Could not find Body in orbitals!")) {
            let other_pos = other.position_vec();
            // println!("Other Position: {:?}", other_pos);
            let other_mass = other.m;
            mass += other_mass;
            center = center.add(&other_pos.mult(other_mass));
        }
        // println!("Total Mass: {:?}", mass);

        if mass > 0.0 {
            center = center.mult(1.0 / mass);
        }
        // println!("Center - Mass: {:?} - {:?}", center, mass);

        (center, mass)
    }

    /// # Under Acceleration
    /// 
    /// Calculates the acceleration the body is under given
    /// the siblings it has. Siblings and their strength are not calculated here, merely
    /// collected.
    pub fn under_accel(&self, delta: &f64, others: &HashMap<usize, Orbital>) -> Vector {
        // println!("Under Acceleration");
        let mut change_sum = Vector::default();
        // Iterate over siblings
        for other in self.siblings.iter()
        .map(|x| others.get(x).expect("Orbital Id Not Found!")) {
            // with other gotten, calculate their gravity vector, multiply by our step 
            // delta and add to our sum.
            let g_vec = &self.gravity_vector(other).mult(*delta);
            // println!("Gravity Vector: {:?}", g_vec);
            change_sum = change_sum.add(g_vec);
        }
        // return our sum.
        // println!("Gravity Sum: {:?}", change_sum);
        // println!("Leaving Under Acceleration");
        change_sum
    }

    /// # Update Position
    /// 
    /// Moves the orbital position forward based on it's current velocity.
    pub fn update_position(&mut self, delta: &f64) {
        // positino vector starts at the position, adds the velocity, multiplied by delta.
        let pos = self.position_vec().add(&self.velocity_vec().mult(*delta));
        self.tx = pos.x;
        self.ty = pos.y;
    }

    /// # Update Velocity
    /// 
    /// Updates the velocity based on the gravitational pull of the body's siblings,
    /// moving forward by our delta in time.
    pub fn update_velocity(&mut self, delta: &f64, others: &HashMap<usize, Orbital>) {
        let g = self.under_accel(delta, others);
        let new_velocity = self.velocity_vec().add(&g.mult(*delta));
        self.vx = new_velocity.x;
        self.vy = new_velocity.y;
    }

    /// # Update Rotatino
    /// 
    /// Updates the rotation bivector by our delta and rotatino velocity.
    /// 
    /// Note, rotation is measured in right angle turns, so 360 degrees = 4 txy.
    /// 
    /// NOTE: If the modulo operation here is too imprecise, make an internal one for extra perecision.
    pub fn update_rotation(&mut self, delta: &f64) {
        let new_rot = self.txy + self.vxy * delta;
        let new_rot = new_rot % 4.0; // cut off at 4
        self.txy = new_rot;
    }

    /// # Take Step
    /// 
    /// Changes the orbital to take a step of the delta given.
    /// 
    /// Does not alter the orbital in place, instead, returning the alterations of the 
    /// orbital.
    /// 
    /// Delta is measured in seconds. Does not break down further, this is the smallest
    /// step of calculation currently.
    pub fn take_step(&self, delta: &f64, others: &HashMap<usize, Orbital>) -> Orbital {
        let mut ret = self.clone();
        // update velocity
        ret.update_velocity(delta, others);
        // Move forward by our step.
        ret.update_position(delta);
        // rotate
        ret.update_rotation(delta);
        ret
    }

    // TODO: Include functions for collisions, don't forget to include rotational effects of the collision.
}