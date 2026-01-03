use std::{cmp::{self, Ordering}, collections::HashMap, f32::consts::{PI, TAU}, time};

use bevy::{
    app::{App, Update}, asset::Assets, ecs::{
        schedule::IntoScheduleConfigs, 
        system::{Commands, Res, ResMut}
    }, 
    input::{ButtonInput, keyboard::KeyCode, mouse::{MouseMotion, MouseWheel}}, 
    log::info, math::VectorSpace, mesh::Mesh, pbr::StandardMaterialFlags, prelude::*, sprite_render::{ColorMaterial, Wireframe2dPlugin}, state::{condition::in_state, state::NextState}};

use crate::{CameraControl, game::{body::Body, orbital::{DAY_TO_SEC, LUNAMASS, Orbital}}, game_state::{self, GameState}};

const TIME_STEP: f64 = DAY_TO_SEC;

#[derive(Default, Debug, Resource)]
pub struct GameData {
    /// Keeps track whether the game is already loaded or not.
    pub game_loaded: bool,
    /// The name of the save, used for saving and loading.
    pub save_name: String,
    /// The body data for 'static' entities.
    pub bodies: HashMap<usize, Body>,
    /// The position and orbtal data for every independent body in the system.
    /// 
    /// Details of the data beyond movement are stored separately.
    /// Orbitals are either Bodies or fleets.
    pub orbitals: HashMap<usize, Orbital>,

    /// A list of ojects which are organized by mass. If not included, then the object is,
    /// definitionally, not massive enough to matter for our gravity calculatons.
    pub massives: Vec<(usize, f64)>,
}

impl GameData {
    /// # Tick
    /// 
    /// Tech function that steps through all orbitals
    pub fn tick(&mut self) {
        // duplicate our oribtals to update.
        let mut next_orbitals = self.orbitals.clone();
        //let delta = 
    }

    /// # Update All Mass Effects
    /// 
    /// This is a heavy duty function which completely resets all siblings and
    /// gravitational calculations.
    /// 
    /// It goes over all orbitals, selects, and organizes them by mass for 
    /// self.massives. Objects that don't have a mass greater than 1.0e18,
    /// are not included as their gravity is too small to matter.
    /// 
    /// TODO: Test this out, and/or get rid of it and instead just simplify to some kind of alternative
    /// 
    /// Ideas for alternatives
    /// 
    /// - Only do X most massive objects. Should cover most needs, but those X objects would need to include
    /// all planets, stars, and moons.
    /// 
    /// - Turn the system into a tree structure and use that to prune what is important for our needs. Much more
    /// dynamic and cool, but may be impractical.
    pub fn update_all_mass_effects(&mut self) {
        self.massives.clear();
        let mass_cutoff = LUNAMASS / 100.0;
        for (_, orb) in self.orbitals.iter_mut() {
            // clear out siblings just in case.
            orb.__siblings.clear();
            // if mass below our cutoff, skip.
            if orb.m <= mass_cutoff {
                continue;
            }
            // if above our cutoff, insert into our massive list.
            let mut inserted = false;
            for idx in 0..self.massives.len() {
                // should be in order from largest to smallest.
                if self.massives[idx].1 <= orb.m { 
                    // if current idx is smaller than our mass, insert.
                    self.massives.insert(idx, (orb.id, orb.m));
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                // If got to the end and found nothing, push to the end.
                self.massives.push((orb.id, orb.m));
            }
        }
        // Swap massive to be from largest to smallest.
        // Sorted by mass, start going through sibling calculations.
        // Only use the self.massive objects.
        let up_orbitals = self.orbitals.clone();
        for (curr_id, orb) in up_orbitals.iter() {
            let mut grav_sum = 0.0;
            for (other_id, _) in self.massives.iter() {
                if curr_id == other_id {
                    // Don't do the calculation with ourselves.
                    continue;
                }
                let other = self.orbitals.get_mut(other_id).unwrap();
                let mut g_force = orb.gravity_vector(other).magnitude();
                // get the gravitational pull of the other.
                if grav_sum == 0.0 {
                    // if first Gforce, always add
                    grav_sum += g_force;
                    self.orbitals.get_mut(curr_id).unwrap().__siblings.push(*other_id);
                } else if g_force / grav_sum > 0.001 {
                    // if next g_force, greater than 0.1% of current total, add to siblings.
                    grav_sum += g_force;
                    self.orbitals.get_mut(curr_id).unwrap().__siblings.push(*other_id);
                } else {
                    // if next g_force is less than 0.1% of current total, skip it.
                    break;
                }
            }
        }
    }

    pub fn mass_less_than(&self, id1: usize, id2: usize) -> bool {
        self.orbitals[&id1].m < self.orbitals[&id2].m
    }
}

#[derive(Debug, Component, Default)]
pub struct OrbitalId(pub usize);

#[derive(Debug, Component, Default)]
pub struct RelativeCameraPosition(pub f32);

pub fn game_plugin(app: &mut App) {
    info!("Game Plugin Loaded.");
    app
    .add_plugins(Wireframe2dPlugin::default())
    .insert_resource(ClearColor(Color::NONE))
    .init_resource::<GameData>()
    .insert_resource(AmbientLight {
        // TODO: modify this to look right later.
        color: Color::WHITE,
        brightness: 1.0,
        ..default()
    })
    .add_systems( OnEnter(GameState::Game), load_game)
        .add_systems(Update, (
            camera_orbit_system,
            camera_keyboard_pan_system,
            camera_zoom_system,
        ).run_if(in_state(GameState::Game)))
    .add_systems(Update, 
        animation_tick.run_if(in_state(GameState::Game)))
    //.add_systems(OnExit(GameState::Game), clear_game)
    .add_systems(Update, 
        (keypress_actions).run_if(in_state(GameState::Game))
    );
    // Init Game Speed
    // playing UI register
    // Pause menu Register
    // Register Update, don't forget to include game speed effects.
    // Do load of testing data.
}

// TODO: Add a clear fame function which clears out entities upon returning to main menu, but not on pause.

pub fn spherical_to_cartesian(radius: f32, azimuth: f32, elevation: f32) -> Vec3 {
    let x = radius * elevation.sin() * azimuth.cos();
    let y = radius * elevation.cos();
    let z = radius * elevation.sin() * azimuth.sin();
    Vec3::new(x, y, z)
}

pub fn camera_orbit_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion_events: MessageReader<MouseMotion>,
    mut camera_control: ResMut<CameraControl>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if buttons.pressed(MouseButton::Left) {
        camera_control.orbiting = true;
    } else {
        camera_control.orbiting = false;
        return;
    }

    let mut total_delta = Vec2::ZERO;
    for event in motion_events.read() {
        total_delta = event.delta;
    }

    if total_delta.length_squared() > 0.0 {
        // sensitivity
        let sensitivity = 0.004;
        camera_control.azimuth -= total_delta.x * sensitivity;
        camera_control.elevation -= total_delta.y * sensitivity;

        // clamp elevation to avoid flipping.
        camera_control.elevation = camera_control
            .elevation
            .clamp(0.01, PI - 0.01);

        // update camera position
        if let Ok(mut transform) = camera_query.single_mut() {
            transform.translation = spherical_to_cartesian(
                camera_control.radius, camera_control.azimuth, camera_control.elevation
            );
            transform.look_at(Vec3::ZERO, Vec3::Y);
        }
    }
}

pub fn camera_zoom_system(
    mut scroll_events: MessageReader<MouseWheel>,
    mut camera_control: ResMut<CameraControl>,
    mut camera_query: Query<&mut Transform, With<Camera>>
) {
    let mut scroll = 0.0;
    for event in scroll_events.read() {
        scroll += event.y;
    }

    if scroll.abs() > 0.0 {
        let zoom_speed = 15.0;
        camera_control.radius -= scroll * zoom_speed;
        camera_control.radius = camera_control.radius.clamp(30.0, 500.0);

        if let Ok(mut transform) = camera_query.single_mut() {
            transform.translation = spherical_to_cartesian(
                camera_control.radius, 
                camera_control.azimuth, 
                camera_control.elevation);
        }
    }
}

pub fn camera_keyboard_pan_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    camera_control: Res<CameraControl>,
) {
    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) { direction += Vec3::Z; }
    if keys.pressed(KeyCode::KeyS) { direction -= Vec3::Z; }
    if keys.pressed(KeyCode::KeyA) { direction -= Vec3::X; }
    if keys.pressed(KeyCode::KeyD) { direction += Vec3::X; }
    if keys.pressed(KeyCode::KeyQ) { direction -= Vec3::Y; }
    if keys.pressed(KeyCode::KeyE) { direction += Vec3::Y; }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        let speed = 80.0 * time.delta_secs();

        if let Ok(mut transform) = camera_query.single_mut() {
            let forward = (Vec3::ZERO - transform.translation).normalize();
            let right = forward.cross(Vec3::Y).normalize();
            let up = Vec3::Y;

            let movement = right * direction.x + up * direction.y + forward * direction.z;
            transform.translation += movement * speed;

            // Update radius and angles based on new position (optional, for consistency)
            // or just keep free movement without constraining to sphere.
        }
    }
}

/// Load game function, should only work the one time on entering GameState::Game
fn load_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_data: ResMut<GameData>,
) {
    // if game has already been loaded, skip this function.
    if game_data.game_loaded {
        return;
    }
    // if not, load and mark the game as loaded.
    game_data.game_loaded = true;

    // Light Source
    // commands.spawn((
    //     PointLight {
    //         intensity: 2000.0,
    //         shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     Transform::from_xyz(10.0, 20.0, 10.0),
    // ));

    // Body 1 (larger, blue)
    //let mass = 10.0;
    game_data.orbitals.insert(0, 
        Orbital::new(0)
            .with_coords(-20.0, 0.0, 0.0)
            .with_mass(10000.0)
            .with_velocity(0.0, 0.0, 4.0));
    commands.spawn((
        Mesh3d(meshes.add(Sphere{
            radius: 3.0,
            ..Default::default()
        }.mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.9, 0.7),
            emissive: Color::srgb(8.0, 5.0, 2.0).into(),
            perceptual_roughness: 0.8,
            reflectance: 0.1,
            ..Default::default()
        })),
        Transform::from_xyz(-20.0, 0.0, 0.0),
        OrbitalId(0),
        GlobalTransform::default(),
        PointLight {
            intensity: 8000000.0,
            radius: 3.0,
            range: 10000.0,
            color: Color::srgb(1.0, 0.95, 0.8),
            shadows_enabled: true,
            ..default()
        }
    ));

    // Body 2 (smaller, red)
    game_data.orbitals.insert(1, 
        Orbital::new(1)
            .with_coords(20.0, 0.0, 0.0)
            .with_mass(5000.0)
            .with_velocity(0.0, 0.0, -8.0));

    commands.spawn((
        Mesh3d(meshes.add(Sphere {
            radius: 2.0,
            ..Default::default()
        }.mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            perceptual_roughness: 0.8,
            reflectance: 0.1,
            ..Default::default()
        })),
        Transform::from_xyz(-20.0, 0.0, 0.0),
        OrbitalId(1)
    ));
    
    // let testhandle1 = meshes.add(Circle::new(50.0));
    // let testhandle2 = meshes.add(Circle::new(25.0));

    // let color = Color::srgba(1.0, 1.0, 1.0, 1.0);

    // commands.spawn((
    //     Mesh2d(testhandle1),
    //     MeshMaterial2d(materials.add(color)),
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    //     OrbitalId(0),
    // ));

    // commands.spawn((
    //     Mesh2d(testhandle2),
    //     MeshMaterial2d(materials.add(Color::srgba(1.0, 0.0, 0.0, 1.0))),
    //     Transform::from_xyz(0.0, 500.0, 0.0),
    //     OrbitalId(1),
    // ));
}

/// # Animation Tick
/// 
/// Calls and updates the position of orbitals and visuals, as well as 
fn animation_tick(
    mut orb_data: ResMut<GameData>,
    mut query: Query<(&mut Transform, &OrbitalId)>,
    time: Res<Time>,
) {
    let mut updates = HashMap::new();

    for (mut transform, OrbitalId(id)) in query {
        if let Some(orbital) = orb_data.orbitals.get(id) {
            let new_orb = orbital.take_step(time.delta_secs_f64(), &orb_data.orbitals);
            // update the transform of our object in the game
            transform.translation = new_orb.t.to_vec3();
            //transform.rotation = new_orb.w.to_quat()
            // Store the update for later.
            updates.insert(*id, new_orb);
        } else {
            info!("Could not find orbital {}!", id);
        }
    }
    // with all updates done, update our orbits.
    orb_data.orbitals = updates;

    // deal with game speed checking here.
    // if time since last tick is not enough, skip the tick.
    // With time having passed successfully,
    // for (mut transform, mut _mesh, OrbitalId(id)) in query.iter_mut() {
    //     // for now, this is a super simple calculation. only move the second object.
    //     if *id == 1 {
    //         let x = f32::sin(time.elapsed_secs() * TAU / 20.0) * 500.0;
    //         let y = f32::cos(time.elapsed_secs() * TAU / 20.0) * 500.0;
    //         transform.translation.x = x;
    //         transform.translation.y = y;
    //     }
    // }
}

pub fn keypress_actions(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    // escape key pause menu
    if keys.just_pressed(KeyCode::Escape) {
        info!("Escape Pressed, Pausing game!");
        game_state.set(GameState::Pause);
    }
}