use bevy::{ecs::component::Component, state::state::States};

/// # Screen States
/// 
/// All the available menu screens in the Game.
/// 
/// Menu Screen is disabled during Game State
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum Screen {
    /// Main menu with options of New Game, Load Game, Settings, and Quit
    Main,
    /// Pause state. Available only when in the Game State.
    Pause,
    /// Settings Menu, allows player to set options for the game. Should not
    /// be so many more than one is needed.
    Settings,
    /// Used when on splash screen, or in game and not paused.
    #[default]
    Disabled,
}

/// # Menu Button Action
/// 
/// The available navigation actions in our buttons.
#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    BackToMainMenu,
    BackToGame,
    Quit,
}