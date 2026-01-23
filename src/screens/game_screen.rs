use std::{cmp::{self, Ordering}, collections::HashMap, f32::consts::{PI, TAU}, time};

use bevy::{
    app::{App, Update}, asset::Assets, core_pipeline::Skybox, ecs::{
        schedule::IntoScheduleConfigs, 
        system::{Commands, Res, ResMut}
    }, input::{ButtonInput, keyboard::KeyCode, mouse::{MouseMotion, MouseWheel}}, log::info, math::VectorSpace, mesh::Mesh, prelude::*, sprite_render::Wireframe2dPlugin, state::{condition::in_state, state::NextState}, transform};

use crate::{game::{body::{Body, BodyType}, orbital::{DAY_TO_SEC, LUNAMASS, Orbital}}, game_state::GameState};

const TIME_STEP: f64 = DAY_TO_SEC;

#[derive(Default, Debug, Resource)]
pub struct GameData {
    /// Keeps track whether the game is already loaded or not.
    pub game_loaded: bool,
    /// The name of the save, used for saving and loading.
    pub save_name: String,
    /// The body data for 'static' entities.
    pub bodies: HashMap<usize, Body>,
    /// The position and orbtal data for every independent body in the system.
    /// 
    /// Details of the data beyond movement are stored separately.
    /// Orbitals are either Bodies or fleets.
    pub orbitals: HashMap<usize, Orbital>,

    /// A list of ojects which are organized by mass. If not included, then the object is,
    /// definitionally, not massive enough to matter for our gravity calculatons.
    pub massives: Vec<(usize, f64)>,
}

impl GameData {
    /// # Tick
    /// 
    /// Tech function that steps through all orbitals
    pub fn tick(&mut self) {
        // duplicate our oribtals to update.
        let mut next_orbitals = self.orbitals.clone();
        //let delta = 
    }

    /// # Update All Mass Effects
    /// 
    /// This is a heavy duty function which completely resets all siblings and
    /// gravitational calculations.
    /// 
    /// It goes over all orbitals, selects, and organizes them by mass for 
    /// self.massives. Objects that don't have a mass greater than 1.0e18,
    /// are not included as their gravity is too small to matter.
    /// 
    /// TODO: Test this out, and/or get rid of it and instead just simplify to some kind of alternative
    /// 
    /// Ideas for alternatives
    /// 
    /// - Only do X most massive objects. Should cover most needs, but those X objects would need to include
    /// all planets, stars, and moons.
    /// 
    /// - Turn the system into a tree structure and use that to prune what is important for our needs. Much more
    /// dynamic and cool, but may be impractical.
    pub fn update_all_mass_effects(&mut self) {
        self.massives.clear();
        let mass_cutoff = LUNAMASS / 100.0;
        for (_, orb) in self.orbitals.iter_mut() {
            // clear out siblings just in case.
            orb.__siblings.clear();
            // if mass below our cutoff, skip.
            if orb.m <= mass_cutoff {
                continue;
            }
            // if above our cutoff, insert into our massive list.
            let mut inserted = false;
            for idx in 0..self.massives.len() {
                // should be in order from largest to smallest.
                if self.massives[idx].1 <= orb.m { 
                    // if current idx is smaller than our mass, insert.
                    self.massives.insert(idx, (orb.id, orb.m));
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                // If got to the end and found nothing, push to the end.
                self.massives.push((orb.id, orb.m));
            }
        }
        // Swap massive to be from largest to smallest.
        // Sorted by mass, start going through sibling calculations.
        // Only use the self.massive objects.
        let up_orbitals = self.orbitals.clone();
        for (curr_id, orb) in up_orbitals.iter() {
            let mut grav_sum = 0.0;
            for (other_id, _) in self.massives.iter() {
                if curr_id == other_id {
                    // Don't do the calculation with ourselves.
                    continue;
                }
                let other = self.orbitals.get_mut(other_id).unwrap();
                let mut g_force = orb.gravity_vector(other).magnitude();
                // get the gravitational pull of the other.
                if grav_sum == 0.0 {
                    // if first Gforce, always add
                    grav_sum += g_force;
                    self.orbitals.get_mut(curr_id).unwrap().__siblings.push(*other_id);
                } else if g_force / grav_sum > 0.001 {
                    // if next g_force, greater than 0.1% of current total, add to siblings.
                    grav_sum += g_force;
                    self.orbitals.get_mut(curr_id).unwrap().__siblings.push(*other_id);
                } else {
                    // if next g_force is less than 0.1% of current total, skip it.
                    break;
                }
            }
        }
    }

    pub fn mass_less_than(&self, id1: usize, id2: usize) -> bool {
        self.orbitals[&id1].m < self.orbitals[&id2].m
    }
}

#[derive(Debug, Component, Default)]
pub struct OrbitalId(pub usize);

#[derive(Debug, Component, Default)]
pub struct RelativeCameraPosition(pub f32);

/// 
#[derive(Component)]
pub struct SidebarButton(Entity);

pub fn game_plugin(app: &mut App) {
    info!("Game Plugin Loaded.");
    app
        .add_plugins(Wireframe2dPlugin::default())
        .insert_resource(ClearColor(Color::NONE))
        .init_resource::<GameData>()
        .add_systems( OnEnter(GameState::Game), load_game)
        .add_systems(Update,
            move_camera_2d.run_if(in_state(GameState::Game)))
        .add_systems(Update, 
            animation_tick.run_if(in_state(GameState::Game)))
        //.add_systems(OnExit(GameState::Game), clear_game)
        .add_systems(Update, 
            (keypress_actions).run_if(in_state(GameState::Game))
        );
    // Init Game Speed
    // playing UI register
    // Pause menu Register
    // Register Update, don't forget to include game speed effects.
    // Do load of testing data.
}

// TODO: Add a clear fame function which clears out entities upon returning to main menu, but not on pause.
fn move_camera_2d(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut motion: MessageReader<MouseMotion>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut cam: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
) {
    if let Ok((mut tf, mut proj)) = cam.single_mut() {
        if let Projection::Orthographic(proj) = &mut *proj {
            // pan with WASD
            let mut move_dir = Vec2::ZERO;
            let pan_speed = 400.0;
            if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) { 
                move_dir.x -= 1.0; 
            }
            if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) { 
                move_dir.x += 1.0; 
            }
            if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) { 
                move_dir.y += 1.0; 
            }
            if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) { 
                move_dir.y -= 1.0; 
            }
            tf.translation += (move_dir.normalize_or_zero() * pan_speed * proj.scale * time.delta_secs()).extend(0.0);

            // Drag Pan via Middle Mouse Button
            if mouse_button.pressed(MouseButton::Middle) {
                for ev in motion.read() {
                    tf.translation -= Vec3::new(ev.delta.x, -ev.delta.y, 0.0) * proj.scale;
                }
            }

            // zoom function
            for ev in mouse_wheel.read() {
                let delta = if ev.y > 0.0 { 0.9 } else { 1.0 / 0.9 };
                proj.scale = (proj.scale * delta).clamp(0.1, 10.0);
            }

            // camera reset
            // R resets the camera to 0.0 and zoom to our default (currently 1.0)
            if keyboard.just_pressed(KeyCode::KeyR) {
                tf.translation = Vec3::ZERO;
                proj.scale = 1.0;
            }
        } else {
            info!("Projection was not Orthographic?!");
        }
    }
}

/// Load game function, should only work the one time on entering GameState::Game
fn load_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_data: ResMut<GameData>,
) {
    // if game has already been loaded, skip this function.
    if game_data.game_loaded {
        return;
    }
    // if not, load and mark the game as loaded.
    game_data.game_loaded = true;
    
    let testhandle1 = meshes.add(Circle::new(50.0));
    let testhandle2 = meshes.add(Circle::new(25.0));

    let color = Color::srgba(1.0, 1.0, 1.0, 1.0);

    commands.spawn((
        Mesh2d(testhandle1),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        OrbitalId(0),
    ));

    commands.spawn((
        Mesh2d(testhandle2),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 0.0, 0.0, 1.0))),
        Transform::from_xyz(0.0, 500.0, 0.0),
        OrbitalId(1),
    ));
}

fn load_sidebar_ui(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_data: ResMut<GameData>,
) {
    let body_entities = vec![];
    // Navigation Sidebar UI
    // TODO: Redo zoom buttons into more nice grid form somthing like Body Name | Go-To Button | Expand/Collapse (children and/or more info) | hide from outliner
    // TODO: If we include the last button (Hide from outliner) we'll also need a 'unhide from outliner' button or context menu.
    commands
        .spawn((
            Node{
                width: Val::Px(280.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9).into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(": Celestial Bodies :"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                }
            ));

            parent.spawn((
                Node::DEFAULT,
            )).with_children(|column| {
                // Free Camera Button
                column.spawn((
                    Button,
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.4).into()),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("⟐ Free Camera"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        }
                    ));
                })
                .insert(SidebarButton(Entity::PLACEHOLDER));

                // Body buttons
                for (orbital_id, entity, color) in body_entities.iter() {
                    column
                        .spawn((
                            Button,
                            Node {
                                width: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(12.0)),
                                margin: UiRect::vertical(Val::Px(4.0)),
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.2).into()),
                            SidebarButton(*entity),
                        ));

                    column.spawn((
                        Node {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            margin: UiRect::right(Val::Px(12.0)),
                            ..default()
                        },
                        BackgroundColor(*color)
                    ));
                    let body_name = game_data.bodies.get(orbital_id).unwrap().name.clone();
                    column.spawn((
                        Text::new(body_name),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE)
                    ));
                }
            });
        });
}

// TODO: Rework this
// pub fn update_sidebar_highlight(
//     locked_target: Res<LockedTarget>,
//     mut button_query: Query<(&SidebarButton, &mut BackgroundColor), With<Button>>,
// ) {

//     for (button, mut bg) in button_query.iter_mut() {
//         if locked_target.entity == Some(button.0) {
//             *bg = Color::srgb(0.4, 0.6, 0.8).into();
//         } else if button.0 == Entity::PLACEHOLDER && locked_target.entity.is_none() {
//             *bg = Color::srgb(0.4, 0.7, 0.4).into();
//         } else {
//             *bg = Color::srgb(0.15, 0.15, 0.2).into();
//         }
//     }
// }

/// # Animation Tick
/// 
/// Calls and updates the position of orbitals and visuals, as well as 
fn animation_tick(
    mut _orb_data: ResMut<GameData>,
    mut query: Query<(&mut Transform, &mut Mesh2d, &OrbitalId)>,
    time: Res<Time>,
) {
    // deal with game speed checking here.
    // if time since last tick is not enough, skip the tick.
    // With time having passed successfully,
    for (mut transform, mut _mesh, OrbitalId(id)) in query.iter_mut() {
        // for now, this is a super simple calculation. only move the second object.
        if *id == 1 {
            let x = f32::sin(time.elapsed_secs() * TAU / 20.0) * 500.0;
            let y = f32::cos(time.elapsed_secs() * TAU / 20.0) * 500.0;
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
}

pub fn keypress_actions(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    // escape key pause menu
    if keys.just_pressed(KeyCode::Escape) {
        info!("Escape Pressed, Pausing game!");
        game_state.set(GameState::Pause);
    }
}