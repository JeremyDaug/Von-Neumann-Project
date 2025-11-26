use bevy::{
    asset::AssetServer, color::Color, ecs::system::{
        Commands, 
        Res, entity_command::observe
    }, 
    picking::hover::Hovered, 
    prelude::*, 
    state::state_scoped::DespawnOnExit, 
    ui::{
        AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, percent
    }, 
    utils::default
};
use bevy_ui_widgets::Activate;

use crate::{game_state::GameState, screens::{menu_plugin::{MENU_COLOR, NORMAL_BUTTON}, screen_state::{MenuButtonAction, Screen}}};


#[derive(Component, Debug)]
struct MainMenuButton;

/// # Main Menu Setup
/// 
/// The setup for the main menu screen.
/// 
/// Shows the UI for mthe main menu.
pub fn main_menu_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        DespawnOnExit(GameState::Game),
        DespawnOnExit(Screen::Main),
        Node { // screen node.
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
    ))
    .with_children(|parent| {
        // Menu Box
        parent.spawn((
            Node { 
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(10),
                margin: UiRect::all(px(50)),
                ..default()
            },
            BackgroundColor(MENU_COLOR.into())
        ))
        .with_children(|parent| {
            parent.spawn((
                button("New Game", MenuButtonAction::NewGame),
            ));

            parent.spawn((
                button("Load Game", MenuButtonAction::Load),
            ));

            parent.spawn((
                button("Settings", MenuButtonAction::Settings),
            ));

            parent.spawn((
                button("Quit", MenuButtonAction::Quit),
            ));
        });
    });
}

fn button(name: &str, action: MenuButtonAction) -> impl Bundle {
    (
        Node {
            width: px(200),
            height: px(60),
            border: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(px(10)),
            ..default()
        },
        MainMenuButton,
        action,
        Button,
        Hovered::default(),
        BorderColor::all(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(NORMAL_BUTTON),
        children![
            Text::new(name),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.3, 0.3, 0.9))
        ],
    )
}