use bevy::{
    app::App, 
    asset::AssetServer, 
    ecs::{
        system::{
            Commands, 
            Res
        }
    }, 
    prelude::*,
    state::state_scoped::DespawnOnExit, 
    ui::{
        AlignItems, 
        JustifyContent, 
        Node, 
        percent, 
        widget::ImageNode
    }, 
    utils::default
};

use crate::game_state::GameState;

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(Timer);

#[derive(Component)]
struct SplashScreen;

/// # Splash Plugin
/// 
/// Sets up the splash screen.
/// 
/// TODO: Should be expanded to show both Bevy's screen, and the company logo(which it's already doing).
pub fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("splash_logo.png");
    // display the logo
    // Command creates (an) entity(ies).
    commands.spawn((
        // Set entity to despawn on exiting the splash screen.
        DespawnOnExit(GameState::Splash),
        // Nodes set up UI information.
        // First node can set up entire screen.
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default() // default to fill out rest.
        },
        SplashScreen,
        // creates child entities.
        children![(
            ImageNode::new(icon),
            Node {
                width: percent(50),
                ..default()
            },
        )],
    ));
    commands.insert_resource(
        SplashTimer(
            Timer::from_seconds(
                1.0, 
                TimerMode::Once)
            )
    );
}

/// # Splash Countdown
/// 
/// The countdown handler for the splash screen.
/// 
/// TODO: Expand to animate the splash screen.
pub fn splash_countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).is_finished() {
        game_state.set(GameState::Menu);
    }
}