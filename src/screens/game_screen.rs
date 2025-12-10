use std::f32::consts::PI;

use bevy::{prelude::*, app::{App, Update}, asset::Assets, ecs::{schedule::IntoScheduleConfigs, system::{Commands, Res, ResMut}}, input::{ButtonInput, keyboard::KeyCode}, log::info, mesh::Mesh, platform::collections::HashMap, sprite_render::{ColorMaterial, Wireframe2dPlugin}, state::{condition::in_state, state::NextState}};

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

fn load_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_data: ResMut<GameData>,
    time: Res<Time>,
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

    let x = f32::sin(time.elapsed_secs() * PI) * 100.0;
    let y = f32::cos(time.elapsed_secs() * PI) * 100.0;

    commands.spawn((
        Mesh2d(testhandle2),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 0.0, 0.0, 1.0))),
        Transform::from_xyz(x, y, 0.0),
        OrbitalId(1),
    ));
}

fn animation_tick() {

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