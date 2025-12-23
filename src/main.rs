pub mod screens;
pub mod game_state;
pub mod splash;
pub mod game;

use bevy::{
    DefaultPlugins, 
    app::{
        App, 
        Startup, 
        Update
    }, 
    prelude::*,
    asset::Assets, 
    camera::{
        Camera3d,
    }, 
    ecs::{
        schedule::IntoScheduleConfigs, 
        system::{
            Commands, 
            ResMut
        }
    }, 
    light::PointLight, 
    math::{
        Vec3, 
        vec3
    }, 
    mesh::{Mesh, Mesh3d}, 
    pbr::StandardMaterial, 
    state::{app::AppExtStates, condition::in_state, state::OnEnter}, 
    transform::components::Transform
};
use bevy_ui_widgets::UiWidgetsPlugins;

use crate::{game::{body::Body, orbital::Orbital}, game_state::GameState, screens::{game_screen::{GameData, OrbitalId, RelativeCameraPosition, game_plugin}, menu_plugin::menu_plugin, pause_menu::pause_menu_plugin}, splash::{splash_countdown, splash_setup}};

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
        .add_plugins((menu_plugin, pause_menu_plugin, game_plugin))
        .add_systems(Update, splash_countdown.run_if(in_state(GameState::Splash)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_data: ResMut<GameData>,
) {
    // Camera setup.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 50.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}