use std::collections::HashMap;

use orbital::Orbital;
use von_neumann::body::{Body, BodyType};

pub mod orbital;

const SOL_M: f64 = 2e30;
const EARTH_M: f64 = 5.972e24;


fn main() {
    let test_data = Body {
        id: 0,
        name: String::from("Tessol"),
        kind: BodyType::Star,
        surface_area: 0.0,
    };

    let orbital = Orbital {
        id: 0,
        is_fixed: true,
        mass: SOL_M,
        thermal_energy: 1e26,
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
        mass_breakdown: HashMap::new(),
        sphere_of_influence: 1.0,
    };
}
