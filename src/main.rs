pub mod screens;
pub mod game_state;
pub mod splash;

use bevy::{DefaultPlugins, app::{App, Startup, Update}, camera::Camera2d, ecs::{schedule::IntoScheduleConfigs, system::Commands}, state::{app::AppExtStates, condition::in_state, state::OnEnter}};
use bevy_ui_widgets::UiWidgetsPlugins;

use crate::{game_state::GameState, splash::{splash_countdown, splash_setup}};

fn main() {
    // Start up app
    App::new()
        // add plugins we need for operations
        .add_plugins(DefaultPlugins)
        .add_plugins(UiWidgetsPlugins)
        // set game state
        .init_state::<GameState>()
        // Game start Setup, basic stuff.
        .add_systems(Startup, setup)
        // These two may be removed into a separate splash screen plugin later.
        .add_systems(OnEnter(GameState::Splash), splash_setup)
        .add_plugins((screens::menu_plugin::menu_plugin,))
        .add_systems(Update, splash_countdown.run_if(in_state(GameState::Splash)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}