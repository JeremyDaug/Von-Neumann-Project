pub mod screens;
pub mod game_state;
pub mod splash;
pub mod game;

use core::f32;
use std::f32::consts::PI;

use bevy::{
    DefaultPlugins, app::{
        App, 
        Startup, 
        Update
    }, asset::Assets, camera::Camera3d, ecs::{
        schedule::IntoScheduleConfigs, 
        system::{
            Commands, 
            ResMut
        }
    }, input::mouse::{MouseMotion, MouseWheel}, light::PointLight, math::{
        NormedVectorSpace, Vec3, VectorSpace, vec3
    }, mesh::{Mesh, Mesh3d}, pbr::StandardMaterial, prelude::*, state::{app::AppExtStates, condition::in_state, state::OnEnter}, transform::components::Transform
};
use bevy_ui_widgets::UiWidgetsPlugins;

use crate::{game_state::GameState, screens::{game_screen::{OrbitalId, RelativeCameraPosition, game_plugin}, menu_plugin::menu_plugin, pause_menu::pause_menu_plugin}, splash::{splash_countdown, splash_setup}};

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
    // mut _meshes: ResMut<Assets<Mesh>>,
    // mut _materials: ResMut<Assets<StandardMaterial>>,
    // mut _game_data: ResMut<GameData>,
) {
    // Camera setup.
    commands.spawn((
        Camera2d,
        OrbitalId::default(),
        RelativeCameraPosition::default()
    ));
}