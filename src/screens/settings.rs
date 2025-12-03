use bevy::{
    asset::AssetServer, 
    picking::hover::Hovered, 
    prelude::*, 
    window::WindowMode
};

use crate::{
    game_state::GameState, 
    screens::{
        menu_plugin::{
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
pub struct FullscreenToggle;

#[derive(Resource, Default, Debug)]
pub struct FullscreenState(bool);

#[derive(Component, Default, Debug)]
pub struct SettingButton;

#[derive(Component, Debug)]
pub enum SettingButtonAction {
    ToggleFullscreen,
    IncrementMaster(f32),
    DecrementMaster(f32),
    IncrementSounds(f32),
    DecrementSounds(f32),
    IncrementMusic(f32),
    DecrementMusic(f32),
}

/// # Settings Setup
/// 
/// This is the main menu setup screen. 
/// 
/// While lazy, duplicating this for the pause screen is entirely valid. Consolidate 
/// them later.
pub fn settings_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    let button_node = Node {
        width: px(200),
        height: px(60),
        border: UiRect::all(px(5)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(px(10)),
        ..default()
    };
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
                    width: px(500),
                    height: px(60),
                    border: UiRect::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(px(10)),
                    ..default()
                },
                Text::new("Display Settings: "),
                TextFont {
                    font_size: 23.0,
                    ..default()
                },
            ));
            // settings buttons
            // Windows vs full screen
            // TODO: Use Button instead of Checkbox, cheat like a little bitch.
            parent.spawn((
                Node {
                    width: px(500),
                    height: px(60),
                    border: UiRect::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(px(10)),
                    ..default()
                },
                BackgroundColor(MENU_COLOR.into()),
                children![(
                            Node {
                                width: px(500),
                                height: px(60),
                                border: UiRect::all(px(5)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(px(10)),
                                ..default()
                            },
                            SettingButton,
                            SettingButtonAction::ToggleFullscreen,
                            Button,
                            Hovered::default(),
                            BorderColor::all(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(NORMAL_BUTTON),
                            children![
                                Text::new("Toggle Fullscreen"),
                                TextFont {
                                    font_size: 23.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.3, 0.3, 0.9))
                            ],
                )]
            ));
            // Section Title: Sounds
            parent.spawn((
                Node {
                    width: px(500),
                    height: px(60),
                    border: UiRect::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(px(10)),
                    ..default()
                },
                Text::new("Audio Settings: "),
                TextFont {
                    font_size: 23.0,
                    ..default()
                },
            ));
            // TODO: Sound settings go here. Not going to bother because we don't have sounds to play anyway.
            // Master Sound
            // Music
            // Sound Effects
            // Back
            parent.spawn((
                button_node.clone(),
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

pub fn setting_buttons_action(
    interaction_query: Query<
        (&Interaction, &SettingButtonAction),
        Changed<Interaction>
    >,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut().unwrap();
    for (interaction, setting_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match setting_action {
                SettingButtonAction::ToggleFullscreen => {
                    if let WindowMode::Windowed = window.mode {
                        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
                    } else {
                        window.mode = WindowMode::Windowed;
                    }
                },
                SettingButtonAction::IncrementMaster(up) => todo!(),
                SettingButtonAction::DecrementMaster(down) => todo!(),
                SettingButtonAction::IncrementSounds(up) => todo!(),
                SettingButtonAction::DecrementSounds(down) => todo!(),
                SettingButtonAction::IncrementMusic(up) => todo!(),
                SettingButtonAction::DecrementMusic(down) => todo!(),
            }
        }
    }
}


fn setting_button(name: &str, action: SettingButtonAction) -> impl Bundle {
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
        SettingButton,
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