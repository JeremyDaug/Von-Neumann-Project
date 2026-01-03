pub mod body;
pub mod orbital;
pub mod vector;

#[cfg(test)]
mod game_tests {
    use std::collections::HashMap;

    use crate::game::orbital::Orbital;

    #[test]
    fn center_of_gravity_should() {
        let mut start = Orbital::new(0)
            .with_coords(10.0, 0.0, 0.0)
            .with_mass(100.0);

        start.__siblings.push(1);
        start.__siblings.push(2);
        let start_pos_vec = start.t;

        let o1 = Orbital::new(1)
            .with_coords(0.0, 10.0, 0.0)
            .with_mass(100000000.0);

        let o2 = Orbital::new(2)
            .with_coords(0.0, -10.0, 0.0)
            .with_mass(100000000.0);

        let mut others = HashMap::new();
        others.insert(0, start);
        others.insert(1, o1);
        others.insert(2, o2);

        // With orbital's mirrored around origin, gravity should pull direrctly in the
        // negative X direction.
        // Normalize for comparison.
        let accel = others.get(&0).unwrap()
            .under_accel(1.0, &others).normalize();
        // println!("accel Vec: {:?}", accel);

        let com = others.get(&0).unwrap()
            .center_of_attraction(&others).0;
        // println!("Com: {:?}", com);
        let rel_com = start_pos_vec.sub(&com).normalize();
        // println!("Rel Com: {:?}", rel_com);

        let dot = accel.dot(rel_com);
        println!("Acceleration: {:?}", accel);
        println!("Relative Center of Mass: {:?}", rel_com);
        println!("Dot Product: {:?}", dot);
        assert!(0.95 <= dot && dot <= 1.05, "Miscalculating center of attraction vector. Outside 5%. {:?}: {:?}", accel, rel_com);
        assert!(0.97 <= dot && dot <= 1.03, "Miscalculating center of attraction vector. Outside 3%. {:?}: {:?}", accel, rel_com);
        assert!(0.99 <= dot && dot <= 1.01, "Miscalculating center of attraction vector. Outside 1%. {:?}: {:?}", accel, rel_com);
    }
}