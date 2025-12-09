use bevy::{
    asset::AssetServer, ecs::{component, system::Commands}, picking::hover::Hovered, prelude::*, state::state_scoped::DespawnOnExit, ui::Node
};

use crate::{game, game_state::{self, GameState}, screens::{menu_plugin::{MENU_COLOR, NORMAL_BUTTON}, screen_state::Screen}};

pub const PAUSE_BACKGROUND: Color = Color::srgba(0.0, 0.0, 0.0, 1.0);

pub fn pause_menu_plugin(app: &mut App) {
    info!("Pause Menu Plugin Loaded.");
    app
    .add_systems(OnEnter(GameState::Pause), pause_menu_setup)
    .add_systems(Update, pause_menu_key_actions.run_if(in_state(GameState::Pause)))
    .add_systems(Update, (pause_button_actions).run_if(in_state(GameState::Pause)));
}

pub fn pause_menu_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        DespawnOnExit(GameState::Pause),
        Node {
            width: percent(30),
            height: percent(100),
            justify_self: JustifySelf::Start,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(PAUSE_BACKGROUND),
    )).with_children(|parent| {
        // pause Menu Box
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
            // Pause Menu Label
            parent.spawn((
                Node {
                    border: UiRect::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(px(10)),
                    ..default()
                },
                Text::new(": Paused :"),
                TextFont {
                    font_size: 23.0,
                    ..default()
                },
            ));
            // Resume Button
            parent.spawn(
                button("Resume", PauseMenuButtonAction::Resume)
            );
            // Save Button (forces save)
            parent.spawn(
                button("Save Game", PauseMenuButtonAction::Save)
            );
            // Return to Menu
            parent.spawn(
                button("Return to Main Menu", PauseMenuButtonAction::ReturnToMenu)
            );
            // Quit game Button
            parent.spawn(
                button("Quit", PauseMenuButtonAction::QuitGame)
            );
        });
    });
}

pub fn pause_button_actions(
    interaction_query: Query<
        (&Interaction, &PauseMenuButtonAction),
        Changed<Interaction>
    >,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut screen: ResMut<NextState<Screen>>
) {
    for (interaction, pause_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match pause_action {
                PauseMenuButtonAction::Resume => {
                    info!("Resume Button Pressed!");
                    game_state.set(GameState::Game);
                    screen.set(Screen::Disabled);
                },
                PauseMenuButtonAction::Save => {
                    info!("Save Button Pressed!");
                    // TODO: Call Save game here.
                },
                PauseMenuButtonAction::ReturnToMenu => {
                    // TODO: Call Save Game here before leaving.
                    info!("Return to Main Menu Button Pressed!");
                    game_state.set(GameState::Menu);
                    screen.set(Screen::Main);
                },
                PauseMenuButtonAction::QuitGame => {
                    // TODO: call save game here before exiting.
                    info!("Quit Button Pressed!");
                    app_exit_writer.write(AppExit::Success);
                },
            }
        }
    }
} 

fn button(name: &str, action: PauseMenuButtonAction) -> impl Bundle {
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
        PauseMenuButton,
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

pub fn pause_menu_key_actions(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    if keys.just_pressed(KeyCode::Escape) {
        info!("Escape Pressed, returning to game!");
        game_state.set(GameState::Game);
    }
}

#[derive(Component, Debug)]
pub enum PauseMenuButtonAction {
    Resume,
    Save,
    ReturnToMenu,
    QuitGame
}

#[derive(Component, Debug)]
pub struct PauseMenuButton;