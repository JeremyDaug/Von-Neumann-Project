use bevy::state::state::States;

/// # Game State
/// 
/// The available game states.
/// 
/// Includes our splash screen for loading, menu state for when the game is not running,
/// and game state, for when it is.
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    /// Default, used for loading the game at start.
    #[default]
    Splash,
    /// The Menu screens part of the game.
    Menu,
    /// The Game state.
    Game,
    /// Called to pause the game. May be removed later as it just overrides the 
    /// game state with the pause menu.
    Pause,
}