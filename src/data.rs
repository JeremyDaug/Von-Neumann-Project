use std::{arch::x86_64, collections::HashMap};

use itertools::Itertools;

use crate::{body::Body, construct::Construct, orbital::Orbital};


pub struct Data {
    pub bodies: HashMap<usize, Body>,
    pub orbitals: HashMap<usize, Orbital>,
    pub fleets: HashMap<usize, Construct>,
    pub blueprints: HashMap<usize, Construct>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            bodies: HashMap::new(),
            orbitals: HashMap::new(),
            fleets: HashMap::new(),
            blueprints: HashMap::new(),
        }
    }

    /// # Reform Parent Tree
    /// 
    /// When called, it goes through all orbitals and figures out who is a parent or
    /// child of whom. 
    /// 
    /// It starts via using the largest body first. It is considered the most likely first
    /// parent.
    /// 
    /// It iterates over them once to try and set each of their spheres of influence.
    /// 
    /// Meant to be done only once at the start after loading data.
    pub fn reform_parent_tree(&mut self) {
        // start by sorting our orbitals by mass.
        let sorted_ids: Vec<usize> = self.orbitals.iter()
            .map(|x| (*x.0, x.1.mass)) // get just mass and ID
            .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap()) // sort by mass in descending order.
            .map(|x| x.0) // get just the ids
            .collect_vec(); // put into vec
        
        // iterate down the ids by mass.
        // the first has infinite SoI.
        let first = self.orbitals.get_mut(sorted_ids.get(0).unwrap()).unwrap();
        first.sphere_of_influence = f64::INFINITY;
        let first: Option<()> = None;
        // all the rest are calculated base on their next most massive nearest neighbor.
        // CHeck that the body is captured by the larger body or not. If so, set it as parent also.
        let mut touched_ids = vec![];
        for id in sorted_ids.iter() {
            // find closest body you are in the SoI of.
            let current = (*self.orbitals.get(id).unwrap()).clone(); // current body
            let mut closest = 0; // current closest we're in the SoI of
            let mut closest_dis = f64::INFINITY; // the distance of the closest body we're in SoI
            for other in touched_ids.iter()
            .map(|x| self.orbitals.get(x).unwrap()) { // go through previous (larger) bodies.
                // check if current is within SoI. If so, Then we're likely their child.
                let distance = other.distance_to(&current);
                if other.sphere_of_influence > distance && // if in SoI
                distance < closest_dis { // And Closest
                    closest = other.id; // update closest
                    closest_dis = distance;
                }
            }
            // once closest is found, calculate SoI
            let other = self.orbitals.get(&closest).unwrap().clone();
            self.orbitals.get_mut(id).unwrap().sphere_of_influence = current.calculate_sphere_of_influence(&other);
            // Next, mark the other body as parent and this body as child of the other.
            self.orbitals.get_mut(id).unwrap().orbital_parent = Some(other.id);
            self.orbitals.get_mut(&other.id).unwrap().orbital_children.push(current.id);
        }
        // with all sorted IDs gone through, we should have a proper tree now and SoIs should be set. Update these as 
    }
}