use bevy::{app::{App, Update}, ecs::{schedule::IntoScheduleConfigs, system::{Res, ResMut}}, input::{ButtonInput, keyboard::KeyCode}, log::info, platform::collections::HashMap, state::{condition::in_state, state::NextState}};

use crate::{game::body::Body, game_state::{self, GameState}};

pub struct GameData {
    /// The name of the save, used for saving and loading.
    pub save_name: String,
    /// The body data for 'static' entities.
    pub bodies: HashMap<usize, Body>,
    /// The position and orbtal data for every independent body in the system.
    /// 
    /// Details of the data beyond movement are stored separately.
    /// Orbitals are either Bodies or fleets.
    pub orbitals: HashMap<usize, Body>,
}

pub fn game_plugin(app: &mut App) {
    info!("Game Plugin Loaded.");
    app
    .add_systems(Update, 
        (keypress_actions).run_if(in_state(GameState::Game))
    );
    // Init Game Speed
    // playing UI register
    // Pause menu Register
    // Register Update, don't forget to include game speed effects.
    // Do load of testing data.
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