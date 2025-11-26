use bevy::{
    asset::AssetServer, picking::hover::Hovered, prelude::*
};
use bevy_ui_widgets::Checkbox;

use crate::{game_state::GameState, screens::{menu_plugin::{MENU_COLOR, NORMAL_BUTTON}, screen_state::{MenuButtonAction, Screen}}};

const CHECKBOX_COLOR: Color = Color::srgb(0.45, 0.45, 0.45);
const CHECKBOX_FILL: Color = Color::srgb(0.35, 0.75, 0.35);

/// # Settings Setup
/// 
/// This is the main menu setup screen. 
/// 
/// While lazy, duplicating this for the pause screen is entirely valid. Consolidate 
/// them later.
pub fn settings_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        DespawnOnExit(GameState::Game),
        DespawnOnExit(Screen::Settings),
        Node { // Screen Node
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK)
    ))
    .with_children(|parent| {
        // menu_box
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
            // settings buttons
            // Windows vs full screen
            parent.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    column_gap: px(4),
                    ..default()
                },
                Name::new("Fullscreen?"),
                Hovered::default(),
                Checkbox
            ))
            .with_children(|parent| {
                // Checkbox outer
                parent.spawn(
                (
                    Node {
                        display: Display::Flex,
                        width: px(16),
                        height: px(16),
                        border: UiRect::all(px(2)),
                        ..default()
                    },
                    BorderColor::all(CHECKBOX_COLOR),
                    BorderRadius::all(px(3)),
                ))
                .with_children(|parent| {
                    // checkbox inner.
                    parent.spawn((
                        Node {
                            display: Display::Flex,
                            width: px(8),
                            height: px(8),
                            position_type: PositionType::Absolute,
                            left: px(2),
                            top: px(2),
                            ..default()
                        },
                        BackgroundColor(CHECKBOX_FILL),
                    ));
                });

                parent.spawn((
                    Text::new("Fullscreen?"),
                ));
            });
            // Master Sound
            // Music
            // Sound Effects
            // Back
            parent.spawn((
                Node {
                    width: px(200),
                    height: px(60),
                    border: UiRect::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(px(10)),
                    ..default()
                },
                MenuButtonAction::BackToMainMenu,
                Button,
                Hovered::default(),
                BorderColor::all(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                children![
                    Text::new("Return"),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.3, 0.3, 0.9))
                ],
            ));
        });
        
    });
}