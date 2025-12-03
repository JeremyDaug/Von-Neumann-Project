/// # Vector
/// 
/// Vector struct, to keep paired things easy.
/// 
/// Only used for things which have 2 components.
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
            y: self.y + other.y
        }
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y
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
    
    /// # Norm
    /// 
    /// Normalizes the vector to be of magnitude 1.
    pub fn norm(&self) -> Vector {
        // println!("Coords: {:?}", self);
        let mag = self.magnitude();
        // println!("Magnitude: {:?}", mag);
        if mag > 0.0 {
            Vector {
                x: self.x / mag,
                y: self.y / mag
            }
        } else {
            Vector::default()
        }
    }
    

    /// # Dot Product
    /// 
    /// Gets the dot product of the vector.
    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }
}