use std::f64::consts::TAU;

use bevy::math::Quat;

/// # Vector
/// 
/// Vector struct, 3 components of f64
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn mult(&self, s: f64) -> Vector {
        Vector { x: self.x * s, y: self.y * s, z: self.z * s }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }

    /// # Magnitude
    /// 
    /// Gets the magnitude of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    /// # Magnitude Squared
    /// 
    /// as Magnitude, but does not take the square root of it.
    pub fn m_sqrd(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
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
                z: self.z / mag
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
    pub fn outer(&self, other: Vector) -> Vector {
        // (ax + by + cz)^(dx + ey + fz) =
        //   ae xy + af xz 
        // + bd yx + bf yz 
        // + cd zx + ce zy =
        // xy (ae - cd) +
        // yz (bf - ce) +
        // zx (cd - af) = (times pseudoscalar)
        // z (ae - cd) + x (bf-ce) + y (cd - af)
        Vector {
            x: self.y*other.z-self.z*other.y,
            y: self.z*other.x-self.x*other.z,
            z: self.x*other.y-self.y*other.x
        }
    }

    /// # Geometric Product
    /// 
    /// Multiplies two vectors to produce the inner and outer product.
    /// 
    /// Useful if you need both.
    pub fn product(&self, other: Vector) -> (f64, Vector) {
        (self.dot(other), self.outer(other))
    }
    
    /// # Angular Clamp
    /// 
    /// Clamp function that binds the values to between + and - TAU
    pub(crate) fn angular_clamp(&self) -> Vector {
        Vector { 
            x: self.x % TAU, 
            y: self.y % TAU, 
            z: self.z % TAU 
        }
    }
    
    pub(crate) fn to_vec3(&self) -> bevy::math::Vec3 {
        bevy::math::Vec3 {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
        }
    }
}