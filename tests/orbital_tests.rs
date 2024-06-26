mod orbital_tests {
    use std::collections::HashMap;

    use von_neumann::orbital::Orbital;

    #[test]
    pub fn collapse_energy_gain_test() {
        let t = Orbital{
            id: 1,
            mass: 1_000_000_000_000_000.0,
            thermal_energy: 1.0,
            rad: 100_000_000.0,
            x: 0.0,
            y: 0.0,
            rot: 0.0,
            vx: 0.0,
            vy: 0.0,
            vrot: 0.0,
            thermal_balance: 0.0,
            orbital_parent: None,
            primary_influences: vec![],
            is_fixed: false,
            sphere_of_influence: 0.0,
            has_collision: true,
            orbital_children: vec![],
        };

        // collapse by half radius
        let res = t.collapse_energy(5_00_000.0);
        println!("{}", res)
    }
}