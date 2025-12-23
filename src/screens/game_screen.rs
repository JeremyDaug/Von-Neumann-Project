use std::{f32::consts::{PI, TAU}, time};

use bevy::{
    app::{App, Update}, asset::Assets, ecs::{
        schedule::IntoScheduleConfigs, 
        system::{Commands, Res, ResMut}
    }, input::{ButtonInput, keyboard::KeyCode, mouse::{MouseMotion, MouseWheel}}, log::info, math::VectorSpace, mesh::Mesh, pbr::StandardMaterialFlags, platform::collections::HashMap, prelude::*, sprite_render::{ColorMaterial, Wireframe2dPlugin}, state::{condition::in_state, state::NextState}};

use crate::{game::{body::Body, orbital::{DAY_TO_SEC, Orbital}}, game_state::{self, GameState}};

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
    .add_systems( OnEnter(GameState::Game), load_game)
    .add_systems(Update,
        move_camera_2d.run_if(in_state(GameState::Game)))
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

/// TODO: This needs to be reworked for the 3d Camera!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
fn move_camera_2d(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut motion: MessageReader<MouseMotion>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut cam: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
) {
    if let Ok((mut tf, mut proj)) = cam.single_mut() {
        if let Projection::Orthographic(proj) = &mut *proj {
            // pan with WASD
            let mut move_dir = Vec2::ZERO;
            let pan_speed = 400.0;
            if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) { 
                move_dir.x -= 1.0; 
            }
            if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) { 
                move_dir.x += 1.0; 
            }
            if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) { 
                move_dir.y += 1.0; 
            }
            if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) { 
                move_dir.y -= 1.0; 
            }
            tf.translation += (move_dir.normalize_or_zero() * pan_speed * proj.scale * time.delta_secs()).extend(0.0);

            // Drag Pan via Middle Mouse Button
            if mouse_button.pressed(MouseButton::Middle) {
                for ev in motion.read() {
                    tf.translation -= Vec3::new(ev.delta.x, -ev.delta.y, 0.0) * proj.scale;
                }
            }

            // zoom function
            for ev in mouse_wheel.read() {
                let delta = if ev.y > 0.0 { 0.9 } else { 1.0 / 0.9 };
                proj.scale = (proj.scale * delta).clamp(0.1, 10.0);
            }

            // camera reset
            // R resets the camera to 0.0 and zoom to our default (currently 1.0)
            if keyboard.just_pressed(KeyCode::KeyR) {
                tf.translation = Vec3::ZERO;
                proj.scale = 1.0;
            }
        } else {
            info!("Projection was not Orthographic?!");
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
    commands.spawn((
        PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0),
    ));

    // Body 1 (larger, blue)
    //let mass = 10.0;
    game_data.orbitals.insert(0, 
        Orbital::new(0)
            .with_coords(-20.0, 0.0, 0.0)
            .with_mass(10.0)
            .with_velocity(0.0, 0.0, 4.0));

    commands.spawn((
        Mesh3d(meshes.add(Sphere{
            radius: 3.0,
            ..Default::default()
        }.mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.8),
            ..Default::default()
        })),
        Transform::from_xyz(-20.0, 0.0, 0.0),
        OrbitalId(0)
    ));

    // Body 2 (smaller, red)
    game_data.orbitals.insert(1, 
        Orbital::new(1)
            .with_coords(20.0, 0.0, 0.0)
            .with_mass(5.0)
            .with_velocity(0.0, 0.0, -8.0));

    commands.spawn((
        Mesh3d(meshes.add(Sphere {
            radius: 2.0,
            ..Default::default()
        }.mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
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
    mut _orb_data: ResMut<GameData>,
    mut query: Query<(&mut Transform, &mut Mesh2d, &OrbitalId)>,
    time: Res<Time>,
) {
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