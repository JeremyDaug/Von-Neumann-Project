use std::f64::consts::TAU;

use bevy::math::Quat;

/// # Vector
/// 
/// Vector struct, 3 components of f64
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn mult(&self, s: f64) -> Vector {
        Vector { x: self.x * s, y: self.y * s }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// # Magnitude
    /// 
    /// Gets the magnitude of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()
    }

    /// # Magnitude Squared
    /// 
    /// as Magnitude, but does not take the square root of it.
    pub fn m_sqrd(&self) -> f64 {
        self.x*self.x + self.y*self.y
    }
    
    /// # Normalize
    /// 
    /// Normalizes the vector to be of magnitude 1.
    pub fn normalize(&self) -> Vector {
        // println!("Coords: {:?}", self);
        let mag = self.magnitude();
        // println!("Magnitude: {:?}", mag);
        if mag > 0.0 {
            Vector {
                x: self.x / mag,
                y: self.y / mag,
            }
        } else {
            Vector::default()
        }
    }
    

    /// # Dot Product
    /// 
    /// Gets the dot product of the vector.
    pub fn dot(&self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// # Outer Product
    /// 
    /// Used to get angular differences between two vectors.
    /// 
    /// Useful for things like angular momentum and velocity.
    pub fn outer(&self, other: Vector) -> f64 {
        // (ax + by + cz)^(dx + ey + fz) =
        //   ae xy + af xz 
        // + bd yx + bf yz 
        // + cd zx + ce zy =
        // xy (ae - cd) +
        // yz (bf - ce) +
        // zx (cd - af) = (times pseudoscalar)
        // z (ae - cd) + x (bf-ce) + y (cd - af)
        self.x*other.y
    }
    
    pub(crate) fn to_vec2(&self) -> bevy::math::Vec2 {
        bevy::math::Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}