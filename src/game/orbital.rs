use std::{collections::HashMap, default};

use crate::game::vector::Vector;

/// # Gravitational Constant
/// 
/// 6.67408e-11 m^3 kg^-1 s^-2
pub const G: f64 = 6.67408e-11; 

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

    // Line Vector, x is vertical line, y is horizontal line, r is the 'line at infinity'.
    /// e_01 X-Translation (m)
    pub tx: f64,
    /// e_20 Y Translation (m)
    pub ty: f64,
    /// e_12 Rotation (m/m)
    pub xy: f64,

    /// e_1, X velocity vector. (m/s)
    pub vx: f64,
    /// e_2, Y velocity Vector. (m/s)
    pub vy: f64,
    /// e_0, Rotational Velocity (m / m / s)
    pub vxy: f64,

    /// e_012, Pseudoscalar factor. Not quite sure what to do with it, but it's here if needed.
    pub txy: f64
}

impl Orbital {
    /// # New
    /// 
    /// Simple new up function.
    pub fn new(id: usize) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn with_radius(mut self, radius: f64) -> Self {
        self.r = radius;
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

    pub fn with_coords(mut self, x: f64, y: f64) -> Self {
        self.tx = x;
        self.ty = y;
        self
    }

    pub fn with_velocity(mut self, x: f64, y: f64) -> Self {
        self.vx = x;
        self.vy = y;
        self
    }

    pub fn with_rotation(mut self, rot: f64) -> Self {
        self.xy = rot;
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
        let norm = r_vector.norm();
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
}