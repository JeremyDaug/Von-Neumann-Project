use crate::orbital::Orbital;

/// # Body
pub struct Body {
    /// The unique id of the body. Shared by it's base and orbital 
    /// information.
    pub id: usize,
    /// The name of the body.
    pub name: String,
    /// What kind of body it is. Star, Jovian, Terrestrial, Asteroid.
    pub kind: BodyType,
    /// All of the orbital factors of the body.
    pub orbital_data: Orbital,
    /// The number of surface slots on the body.
    /// Set by the initial self.orbital_data.rad. Only changes when 
    /// the radius changes dramatically (IE, gravitational collapse).
    /// 
    /// Should the number of buildings on it be greater than the surface area, 
    /// then they are destroyed by 
    pub surface_area: f64,

}

pub enum BodyType {
    /// Very large in size, has a large positive energy output from fusion.
    /// 
    /// Cannot be landed on.
    Star,
    /// Large in size, small positive energy.
    /// 
    /// Requires tech to land on (Aerial Bases).
    Jovian,
    /// Moderate in size, small positive energy. Can be landed on immediately.
    /// 
    /// Should be large enough to clear orbit and requires sizeable energy 
    /// output to leave, requiring either trans-orbital engines, or mass
    /// drivers.
    Terrestrial,
    /// Small in size, and little to no energy generation. Can be landed on
    /// immediately.
    /// 
    /// So small that its gravity is often barely enough to keep it together.
    Asteroid
}