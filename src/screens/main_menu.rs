use bevy::{
    asset::AssetServer, ecs::{
        system::{
            Commands, 
            Res
        }
    }, 
    prelude::*,
    state::state_scoped::DespawnOnExit, ui::{
        AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, percent
    }, 
    utils::default,
    color::Color
};

use crate::game_state::GameState;

// Slightly translucent Grey.
const MENU_COLOR: Color = Color::linear_rgba(0.5, 0.5, 0.5, 0.9);

/// # Main Menu Setup
/// 
/// The setup for the main menu screen.
/// 
/// Shows the UI for mthe main menu.
pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 
    commands.spawn((
        // remove on leaving the menu state.
        DespawnOnExit(GameState::Game),
        Node { // screen node.
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        children![(
            // Menu Box Node
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                // width: px(150),
                // height: px(100),
                ..default()
            },
            BackgroundColor(MENU_COLOR.into()),
            children![(
                // buttons go here.
                (// New Game Button
                    
                )
            )]
        )],
    ));
}