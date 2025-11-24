use bevy::{
    asset::AssetServer, color::Color, ecs::system::{
        Commands, 
        Res
    }, 
    picking::hover::Hovered, 
    prelude::*, 
    state::state_scoped::DespawnOnExit, 
    ui::{
        AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, percent
    }, 
    utils::default
};

use crate::game_state::GameState;

// Slightly translucent Grey.
const MENU_COLOR: Color = Color::linear_rgba(0.5, 0.5, 0.5, 0.9);

// text color

// button color dark grey
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
// button border color

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
                row_gap: px(10),
                margin: UiRect::all(px(50)),
                // width: px(150),
                // height: px(100),
                ..default()
            },
            BackgroundColor(MENU_COLOR.into()),
            children![(
                // buttons go here.
                (// New Game Button
                    Node {
                        width: px(200),
                        height: px(60),
                        border: UiRect::all(px(5)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(px(10)),
                        ..default()
                    },
                    Button,
                    Hovered::default(),
                    BorderColor::all(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    children![(
                        Text::new("New Game"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.3, 0.3, 0.9)),
                        //TextShadow::default()
                    )]
                ),
            )]
        )],
    ));
}