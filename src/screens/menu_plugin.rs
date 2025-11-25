use std::fmt::Formatter;

use bevy::{
    app::{App, AppExit, Update}, color::{Color, palettes::css::GRAY}, ecs::{entity_disabling::Disabled, hierarchy::Children, message::MessageWriter, query::{Added, Changed, Has, Or, With}, schedule::IntoScheduleConfigs, system::{Query, ResMut}}, log::info, picking::{events::Press, hover::Hovered}, reflect::PartialReflect, state::{
        app::AppExtStates, condition::in_state, state::{NextState, OnEnter}
    }, ui::{BackgroundColor, BorderColor, Interaction, InteractionDisabled, Pressed, widget::Text}
};
use bevy_ui_widgets::Button;

use crate::{game_state::GameState, screens::{
    main_menu::main_menu_setup, 
    pause_menu::pause_menu_setup, 
    screen_state::{MenuButtonAction, Screen}, 
    settings::settings_setup
}};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const NORMAL_BUTTON_BORDER: Color = Color::srgb(0.35, 0.35, 0.35);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_BUTTON_BORDER: Color = Color::srgb(0.65, 0.65, 0.65);
//const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const PRESSED_BUTTON_BORDER: Color = Color::srgb(0.35, 0.35, 0.85);

pub fn menu_plugin(app: &mut App) {
    app
    // When loading this plugin, what state we start on.
    // default screen is main menu.
    .init_state::<Screen>()
    // What to call on entering GameState::Menu
    .add_systems(OnEnter(GameState::Menu), menu_setup)
    .add_systems(OnEnter(Screen::Main), main_menu_setup)
    .add_systems(OnEnter(Screen::Pause), pause_menu_setup)
    .add_systems(OnEnter(Screen::Settings), settings_setup)
    .add_systems(Update, 
        (menu_action, update_buttons).run_if(in_state(GameState::Menu)));
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
fn menu_action(
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