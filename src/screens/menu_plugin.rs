use bevy::{
    prelude::*,
    app::{App, AppExit, Update}, color::Color, ecs::{message::MessageWriter, query::{Changed, Has, Or}, schedule::IntoScheduleConfigs, system::{Query, ResMut}}, log::info, picking::hover::Hovered, state::{
        app::AppExtStates, condition::in_state, state::{NextState, OnEnter}
    }, 
    ui::{BackgroundColor, BorderColor, Interaction, Pressed}
};

use crate::{game_state::GameState, screens::{
    main_menu::main_menu_setup, 
    pause_menu::pause_menu_setup, 
    screen_state::{MenuButtonAction, Screen}, 
    settings::{FullscreenState, setting_buttons_action, settings_setup}
}};

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const NORMAL_BUTTON_BORDER: Color = Color::srgb(0.35, 0.35, 0.35);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const HOVERED_BUTTON_BORDER: Color = Color::srgb(0.65, 0.65, 0.65);
//const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
pub const PRESSED_BUTTON_BORDER: Color = Color::srgb(0.35, 0.35, 0.85);
// checkbox colors
pub const CHECKBOX_COLOR: Color = Color::srgb(0.45, 0.45, 0.45);
pub const CHECKBOX_FILL: Color = Color::srgb(0.35, 0.75, 0.35);
pub const CHECKBOX_OUTLINE: Color = Color::srgb(0.45, 0.45, 0.45);

// Slightly translucent Grey.
pub const MENU_COLOR: Color = Color::linear_rgba(0.5, 0.5, 0.5, 0.9);

#[derive(Resource, Debug, Default)]
pub struct MasterVolume(f32);

#[derive(Resource, Debug, Default)]
pub struct SoundVolume(f32);

#[derive(Resource, Debug, Default)]
pub struct MusicVolume(f32);

pub fn menu_plugin(app: &mut App) {
    app
    // When loading this plugin, what state we start on.
    // default screen is main menu.
    .init_state::<Screen>()
    // TODO: Update this to load fullscreen state setting from a settings config file.
    .init_resource::<FullscreenState>()
    .init_resource::<MasterVolume>()
    .init_resource::<SoundVolume>()
    .init_resource::<MusicVolume>()
    // What to call on entering GameState::Menu
    .add_systems(OnEnter(GameState::Menu), menu_setup)
    .add_systems(OnEnter(Screen::Main), main_menu_setup)
    .add_systems(OnEnter(Screen::Pause), pause_menu_setup)
    .add_systems(OnEnter(Screen::Settings), settings_setup)
    .add_systems(Update, 
        (menu_button_action, update_buttons, 
        ).run_if(in_state(GameState::Menu)))
    .add_systems(Update, (setting_buttons_action,)
        .run_if(in_state(Screen::Settings)));
}

fn update_buttons(
    mut buttons: Query<
        (
            Has<Pressed>,
            &Hovered,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (
            Or<(
                Changed<Pressed>,
                Changed<Hovered>,
            )>,
        )
    >,
) {
    for (pressed, hovered, mut color, mut border_color) in &mut buttons {
        set_button_style(hovered.get(), pressed, &mut color, &mut border_color);
    }
}

pub fn set_button_style(
    hovered: bool,
    pressed: bool,
    color: &mut BackgroundColor,
    border_color: &mut BorderColor,
) {
    match (hovered, pressed) {
        (true, true) => { // Pressed and Hovered
            *color = PRESSED_BUTTON.into();
            border_color.set_all(PRESSED_BUTTON_BORDER);
        },
        (true, false) => {
            *color = HOVERED_BUTTON.into();
            border_color.set_all(HOVERED_BUTTON_BORDER);
        },
        (false, _) => {
            *color = NORMAL_BUTTON.into();
            border_color.set_all(NORMAL_BUTTON_BORDER);
        },
    }
}

/// # Menu Setup
/// 
/// Setup for the menu screens. Called on entering GameState::Menu
/// 
/// Just sets the screen state to Screen::Main
fn menu_setup(mut menu_state: ResMut<NextState<Screen>>) {
    menu_state.set(Screen::Main);
}

/// # Menu Action
/// 
/// Occurs when a button is selected.
/// 
/// Changes the current menu screen state.
fn menu_button_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        Changed<Interaction>
    >,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut screen: ResMut<NextState<Screen>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::NewGame => {
                    info!("New Game button Pressed!");
                    game_state.set(GameState::Game);
                    screen.set(Screen::Disabled);
                },
                MenuButtonAction::Load => {
                    info!("Load Button Pressed!");
                }
                MenuButtonAction::Settings => {
                    info!("Settings Button Pressed!");
                    screen.set(Screen::Settings)
                },
                MenuButtonAction::BackToMainMenu => {
                    info!("Back To Main Menu Pressed!");
                    screen.set(Screen::Main);
                },
                MenuButtonAction::BackToGame => {
                    info!("Return to Game Pressed!");
                    screen.set(Screen::Disabled);
                    //game_state.set();
                },
                MenuButtonAction::Quit => {
                    info!("Quit Button Pressed!");
                    app_exit_writer.write(AppExit::Success);
                },
            }
        }
    }
}
