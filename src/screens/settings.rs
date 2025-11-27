use bevy::{
    asset::AssetServer, 
    picking::hover::Hovered, 
    prelude::*, 
    ui::interaction_states, 
    window::{
        PrimaryWindow, 
        WindowMode
    }
};
use bevy_ui_widgets::{Checkbox, checkbox_self_update};

use crate::{
    game_state::GameState, 
    screens::{
        menu_plugin::{
            CHECKBOX_COLOR, 
            CHECKBOX_FILL, 
            CHECKBOX_OUTLINE, 
            MENU_COLOR, 
            NORMAL_BUTTON
        }, 
        screen_state::{
            MenuButtonAction, 
            Screen
        }
    }
};

#[derive(Component, Default, Debug)]
pub struct SettingCheckbox;

#[derive(Component, Default, Debug)]
pub struct FullscreenCheckbox;

#[derive(Component, Default, Debug)]
pub struct Checkmark;

#[derive(Resource, Default)]
pub struct FullscreenState(bool);

/// # Settings Setup
/// 
/// This is the main menu setup screen. 
/// 
/// While lazy, duplicating this for the pause screen is entirely valid. Consolidate 
/// them later.
pub fn settings_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            // Section Title: Display
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
                children![(
                    Text::new("Display Settings: "),
                    TextFont {
                        font_size: 23.0,
                        ..default()
                    },
                )]
            ));
            // settings buttons
            // Windows vs full screen
            parent.spawn((
                Node{
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(px(5)),
                    ..default()
                },
                Checkbox,
                Hovered::default(),
            )).with_children(|row| {
                // checkbox square
                row.spawn((
                    Node {
                        width: px(24),
                        height: px(24),
                        border: UiRect::all(px(3)),
                        margin: UiRect::all(px(3)),
                        ..default()
                    },
                    BorderColor::all(CHECKBOX_OUTLINE),
                    BackgroundColor(CHECKBOX_FILL),
                    FullscreenCheckbox,
                ));

                // checkmark (hidden by default)
                row.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: px(24),
                        height: px(24),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    Checkmark,
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("X"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                    ));
                });

                // Label
                row.spawn((
                    Text::new(":Fullscreen"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                ));
            });
            // Section Title: Sounds
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
                children![(
                    Text::new("Audio Settings: "),
                    TextFont {
                        font_size: 23.0,
                        ..default()
                    },
                )]
            ));
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

pub fn checkbox_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        With<FullscreenCheckbox>
    >,
    mut checkmark_query: Query<&mut Visibility, With<Checkmark>>,
    mut fullscreen_state: ResMut<FullscreenState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = if let Ok(win) = windows.single_mut() {
        win
    } else {
        info!("Could not find window!?");
        return;
    };
    let mut checkmark_visibility = if let Ok(check) = checkmark_query.single_mut() {
        check
    } else {
        info!("Could not find the Checkmark!?");
        return;
    };

    info!("Checkbox_System: Queries {}", interaction_query.count());

    for (interaction, mut bg_color, mut border_color) in &mut interaction_query {
        info!("Interaction: {:?}", interaction);
        match *interaction {
            Interaction::Pressed => {
                // toggle the state
                fullscreen_state.0 = !fullscreen_state.0;

                // apply to window
                window.mode = if fullscreen_state.0 {
                    WindowMode::BorderlessFullscreen(MonitorSelection::Current)
                } else {
                    WindowMode::Windowed
                };

                // Update visual Feedback
                if fullscreen_state.0 {
                    *bg_color = BackgroundColor(Color::srgb(0.0, 0.8, 0.0));
                    *border_color = Color::srgb(0.0, 1.0, 0.0).into();
                    *checkmark_visibility = Visibility::Visible;
                } else {
                    *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 1.0));
                    *border_color = Color::WHITE.into();
                    *checkmark_visibility = Visibility::Hidden;
                }
            },
            Interaction::Hovered => {
                *border_color = Color::srgb(0.8, 0.8, 1.0).into();
            },
            Interaction::None => {
                *border_color = if fullscreen_state.0 {
                    Color::srgb(0.0, 1.0, 0.0).into()
                } else {
                    Color::WHITE.into()
                };
            },
        }
    }

    // Keep UI in sync if user toggles with Alt+Enter
    let current_is_fullscreen = matches!(
        window.mode,
        WindowMode::Fullscreen(..) | WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    );
    if current_is_fullscreen != fullscreen_state.0 {
        fullscreen_state.0 = current_is_fullscreen;
        // Update visuals
        let (bg, border, visibility) = if fullscreen_state.0 {
            (
                Color::srgb(0.0, 0.8, 0.0),
                Color::srgb(0.0, 1.0, 0.0),
                Visibility::Visible,
            )
        } else {
            (
                Color::srgba(0.15, 0.15, 0.15, 1.0),
                Color::WHITE,
                Visibility::Hidden,
            )
        };
        *checkmark_visibility = visibility;
    }
}