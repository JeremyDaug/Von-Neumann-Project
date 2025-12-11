use std::f32::consts::PI;

use bevy::{app::{App, Update}, asset::Assets, camera::CameraProjection, ecs::{schedule::IntoScheduleConfigs, system::{Commands, Res, ResMut}}, input::{ButtonInput, keyboard::KeyCode, mouse::MouseWheel}, log::info, mesh::Mesh, platform::collections::HashMap, prelude::*, sprite_render::{ColorMaterial, Wireframe2dPlugin}, state::{condition::in_state, state::NextState}};

use crate::{game::{body::Body, orbital::Orbital}, game_state::{self, GameState}};

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

#[derive(Debug, Component)]
pub struct OrbitalId(usize);

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

fn move_camera_2d(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut cam: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
) {
    if let Ok((mut tf, mut proj)) = cam.single_mut() {
        if let Projection::Orthographic(proj) = &mut *proj {
            // pan with WASD
            let mut move_dir = Vec2::ZERO;
            let pan_speed = 400.0;
            if keyboard.pressed(KeyCode::KeyA) { move_dir.x -= 1.0; }
            if keyboard.pressed(KeyCode::KeyD) { move_dir.x += 1.0; }
            if keyboard.pressed(KeyCode::KeyW) { move_dir.y += 1.0; }
            if keyboard.pressed(KeyCode::KeyS) { move_dir.y -= 1.0; }
            tf.translation += (move_dir.normalize_or_zero() * pan_speed * proj.scale * time.delta_secs()).extend(0.0);

            // zoom function
            for ev in mouse_wheel.read() {
                let delta = if ev.y > 0.0 { 0.9 } else { 1.0 / 0.9 };
                proj.scale = (proj.scale * delta).clamp(0.1, 10.0);
            }
        } else {
            info!("Projection was not Orthographic?!");
        }
    }
}

fn load_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_data: ResMut<GameData>,
) {
    // if game has already been loaded, skip this function.
    if game_data.game_loaded {
        return;
    }
    // if not, load and mark the game as loaded.
    game_data.game_loaded = true;
    
    let testhandle1 = meshes.add(Circle::new(50.0));
    let testhandle2 = meshes.add(Circle::new(25.0));

    let color = Color::srgba(1.0, 1.0, 1.0, 1.0);

    commands.spawn((
        Mesh2d(testhandle1),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        OrbitalId(0),
    ));

    commands.spawn((
        Mesh2d(testhandle2),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 0.0, 0.0, 1.0))),
        Transform::from_xyz(0.0, 100.0, 0.0),
        OrbitalId(1),
    ));
}

fn animation_tick(
    mut _orb_data: ResMut<GameData>,
    mut query: Query<(&mut Transform, &OrbitalId)>,
    time: Res<Time>,
) {
    // deal with game speed checking here.
    // if time since last tick is not enough, skip the tick.
    // With time having passed successfully,
    for (mut transform, OrbitalId(id)) in query.iter_mut() {
        // for now, this is a super simple calculation. only move the second object.
        if *id == 1 {
            let x = f32::sin(time.elapsed_secs() * PI) * 100.0;
            let y = f32::cos(time.elapsed_secs() * PI) * 100.0;
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
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