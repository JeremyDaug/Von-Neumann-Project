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
        // all the rest are calculated base on their next most massive nearest neighbor.
        // CHeck that the body is captured by the larger body or not. If so, set it as parent also.
    }
}