use std::{cmp::{self, Ordering}, collections::HashMap, f32::consts::{PI, TAU}, time};

use bevy::{
    app::{App, Update}, asset::Assets, core_pipeline::Skybox, ecs::{
        schedule::IntoScheduleConfigs, 
        system::{Commands, Res, ResMut}
    }, input::{ButtonInput, keyboard::KeyCode, mouse::{MouseMotion, MouseWheel}}, log::info, math::VectorSpace, mesh::Mesh, prelude::*, sprite_render::Wireframe2dPlugin, state::{condition::in_state, state::NextState}};

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

#[derive(Resource, Default)]
pub struct CameraControl {
    /// Whether the camera is orbiting right now or not.
    pub orbiting: bool,
    /// The previous position of the mouse on the screen.
    pub last_mouse_pos: Vec2,
    /// Distance from the Center
    pub radius: f32,
    /// Horizontal Angle (Radians)
    pub azimuth : f32,
    /// Vertical Angel (Radians)
    pub elevation: f32,
    /// Which orbital the camera is locked on.
    pub lock: Option<usize>,
}

#[derive(Resource, Default)]
pub struct LockedTarget {
    /// The entity we are locked on to.
    pub entity: Option<Entity>,
}

/// 
#[derive(Component)]
pub struct SidebarButton(Entity);

pub fn game_plugin(app: &mut App) {
    info!("Game Plugin Loaded.");
    app
        .add_plugins(Wireframe2dPlugin::default())
        .insert_resource(ClearColor(Color::NONE))
        .init_resource::<GameData>()
        .insert_resource(AmbientLight {
            // TODO: modify this to look right later.
            color: Color::WHITE,
            brightness: 1.0,
            ..default()
        })
        .init_resource::<LockedTarget>()
        .add_systems( OnEnter(GameState::Game), load_game)
            .add_systems(Update, (
                //camera_orbit_system,
                //camera_keyboard_pan_system,
                //camera_zoom_system,
                camera_control_system,
                sidebar_button_system,
                update_sidebar_highlight,
            ).run_if(in_state(GameState::Game)))
        .add_systems(OnEnter(GameState::Game), setup_skybox)
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

pub fn spherical_to_cartesian(radius: f32, azimuth: f32, elevation: f32) -> Vec3 {
    let x = radius * elevation.sin() * azimuth.cos();
    let y = radius * elevation.cos();
    let z = radius * elevation.sin() * azimuth.sin();
    Vec3::new(x, y, z)
}

pub fn setup_skybox(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // simple option, using hdr
    // TODO: Current Skybox is a 'free' skybox found online. Should make our own later.
    commands.spawn(
        Skybox {
            image: asset_server.load("skybox.hdr"), 
            ..Default::default()
        }
    );
}

pub fn camera_orbit_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion_events: MessageReader<MouseMotion>,
    mut camera_control: ResMut<CameraControl>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if buttons.pressed(MouseButton::Left) {
        camera_control.orbiting = true;
    } else {
        camera_control.orbiting = false;
        return;
    }

    let mut total_delta = Vec2::ZERO;
    for event in motion_events.read() {
        total_delta = event.delta;
    }

    if total_delta.length_squared() > 0.0 {
        // sensitivity
        let sensitivity = 0.004;
        camera_control.azimuth -= total_delta.x * sensitivity;
        camera_control.elevation -= total_delta.y * sensitivity;

        // clamp elevation to avoid flipping.
        camera_control.elevation = camera_control
            .elevation
            .clamp(0.01, PI - 0.01);

        // update camera position
        if let Ok(mut transform) = camera_query.single_mut() {
            transform.translation = spherical_to_cartesian(
                camera_control.radius, camera_control.azimuth, camera_control.elevation
            );
            transform.look_at(Vec3::ZERO, Vec3::Y);
        }
    }
}

pub fn camera_zoom_system(
    mut scroll_events: MessageReader<MouseWheel>,
    mut camera_control: ResMut<CameraControl>,
    mut camera_query: Query<&mut Transform, With<Camera>>
) {
    let mut scroll = 0.0;
    for event in scroll_events.read() {
        scroll += event.y;
    }

    if scroll.abs() > 0.0 {
        let zoom_speed = 15.0;
        camera_control.radius -= scroll * zoom_speed;
        camera_control.radius = camera_control.radius.clamp(30.0, 500.0);

        if let Ok(mut transform) = camera_query.single_mut() {
            transform.translation = spherical_to_cartesian(
                camera_control.radius, 
                camera_control.azimuth, 
                camera_control.elevation);
        }
    }
}

pub fn camera_keyboard_pan_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    camera_control: Res<CameraControl>,
) {
    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) { direction += Vec3::Z; }
    if keys.pressed(KeyCode::KeyS) { direction -= Vec3::Z; }
    if keys.pressed(KeyCode::KeyA) { direction -= Vec3::X; }
    if keys.pressed(KeyCode::KeyD) { direction += Vec3::X; }
    if keys.pressed(KeyCode::KeyQ) { direction -= Vec3::Y; }
    if keys.pressed(KeyCode::KeyE) { direction += Vec3::Y; }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        let speed = 80.0 * time.delta_secs();

        if let Ok(mut transform) = camera_query.single_mut() {
            let forward = (Vec3::ZERO - transform.translation).normalize();
            let right = forward.cross(Vec3::Y).normalize();
            let up = Vec3::Y;

            let movement = right * direction.x + up * direction.y + forward * direction.z;
            transform.translation += movement * speed;

            // Update radius and angles based on new position (optional, for consistency)
            // or just keep free movement without constraining to sphere.
        }
    }
}

fn camera_control_system(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    body_query: Query<(&Transform, &OrbitalId), Without<Camera>>,
    locked_target: Res<LockedTarget>,
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion_events: MessageReader<MouseMotion>,
    mut scroll_events: MessageReader<MouseWheel>,
    mut camera_control: ResMut<CameraControl>,
) {
    let mut camera_transform = camera_query.single_mut().expect("Camera not found!");

    if let Some(target_entity) = locked_target.entity {
        if let Ok((target_transform, _)) = body_query.get(target_entity) {
            // Smoothly follow the target with offset
            let target_pos = target_transform.translation;
            let desired_pos = target_pos + Vec3::new(0.0, 50.0, 120.0);
            camera_transform.translation = camera_transform.translation.lerp(desired_pos, 8.0 * time.delta_secs());
            camera_transform.look_at(target_pos, Vec3::Y);
            return; // Locked mode overrides manual control
        }
    }

    // Free Camera Mode
    // Orbit with mouse Drag
    if buttons.pressed(MouseButton::Left) {
        let mut delta = Vec2::ZERO;
        for ev in motion_events.read() {
            delta += ev.delta;
        }
        if delta.length_squared() > 0.0 {
            // TODO: Break sensitivity out from here to be configurable.
            let sensitivity = 0.004;
            camera_control.azimuth -= delta.x * sensitivity;
            camera_control.elevation -= delta.y * sensitivity;
            camera_control.elevation = camera_control.elevation.clamp(0.1, PI - 0.1);
        }
    }

    // Zoom 
    let mut scroll = 0.0;
    for ev in scroll_events.read() {
        scroll += ev.y;
    }
    if scroll != 0.0 {
        // TODO: Break scroll_sensitivity out from here to be configurable.
        let scroll_sensitivity = 20.0;
        camera_control.radius -= scroll * scroll_sensitivity;
        camera_control.radius = camera_control.radius.clamp(50.0, 800.0);
    }

    // update position
    camera_transform.translation = spherical_to_cartesian(camera_control.radius, 
        camera_control.azimuth, camera_control.elevation);
    
    camera_transform.look_at(Vec3::ZERO, Vec3::Y);
}

/// Load game function, should only work the one time on entering GameState::Game
fn load_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_data: ResMut<GameData>,
    mut camera_control: Res<CameraControl>,
) {
    // if game has already been loaded, skip this function.
    if game_data.game_loaded {
        return;
    }
    // if not, load and mark the game as loaded.
    game_data.game_loaded = true;

    // Light Source
    // commands.spawn((
    //     PointLight {
    //         intensity: 2000.0,
    //         shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     Transform::from_xyz(10.0, 20.0, 10.0),
    // ));

    let mut body_entities = vec![];

    // Body 1 (larger, blue)
    //let mass = 10.0;
    game_data.bodies.insert(0, Body { id: 0, name: "Smallers".into(), body_type: BodyType::Jovian, total_mass: 5000.0, resources: HashMap::new(), storage: HashMap::new(), radius: 1.0, tempurature: 1.0 });
    game_data.orbitals.insert(0, 
        Orbital::new(0)
            .with_coords(-20.0, 0.0, 0.0)
            .with_mass(10000.0)
            .with_velocity(0.0, 0.0, 4.0));
    let star_entity = commands.spawn((
        Mesh3d(meshes.add(Sphere{
            radius: 3.0,
            ..Default::default()
        }.mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.9, 0.7),
            emissive: Color::srgb(8.0, 5.0, 2.0).into(),
            perceptual_roughness: 0.8,
            reflectance: 0.1,
            ..Default::default()
        })),
        Transform::from_xyz(-20.0, 0.0, 0.0),
        OrbitalId(0),
        GlobalTransform::default(),
        PointLight {
            intensity: 8000000.0,
            radius: 3.0,
            range: 10000.0,
            color: Color::srgb(1.0, 0.95, 0.8),
            shadows_enabled: true,
            ..default()
        }
    ));

    body_entities.push((0, star_entity.id(), Color::srgb(1.0, 0.9, 0.6)));

    // Body 2 (smaller, red)
    game_data.bodies.insert(1, Body { id: 1, name: "Smallers".into(), body_type: BodyType::Jovian, total_mass: 5000.0, resources: HashMap::new(), storage: HashMap::new(), radius: 1.0, tempurature: 1.0 });
    game_data.orbitals.insert(1, 
        Orbital::new(1)
            .with_coords(20.0, 0.0, 0.0)
            .with_mass(5000.0)
            .with_velocity(0.0, 0.0, -8.0));

    let planet_entity = commands.spawn((
        Mesh3d(meshes.add(Sphere {
            radius: 2.0,
            ..Default::default()
        }.mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            perceptual_roughness: 0.8,
            reflectance: 0.1,
            ..Default::default()
        })),
        Transform::from_xyz(-20.0, 0.0, 0.0),
        OrbitalId(1)
    ));

    body_entities.push((1, planet_entity.id(), Color::srgb(1.0, 0.9, 0.6)));
    
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

/// A helper to generate outliner rows.
// fn outliner_row() -> impl Bundle {
//     todo!()
// }

fn _load_sidebar_ui(mut _commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
    _camera_control: Res<CameraControl>,
) {
    todo!()
}

pub fn sidebar_button_system(
    mut interaction_query: Query<(&Interaction, &SidebarButton), (Changed<Interaction>, With<Button>)>,
    mut locked_target: ResMut<LockedTarget>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if button.0 == Entity::PLACEHOLDER {
                locked_target.entity = None; // Unlock
            } else {
                locked_target.entity = Some(button.0);
            }
        }
    }
}

pub fn update_sidebar_highlight(
    locked_target: Res<LockedTarget>,
    mut button_query: Query<(&SidebarButton, &mut BackgroundColor), With<Button>>,
) {

    for (button, mut bg) in button_query.iter_mut() {
        if locked_target.entity == Some(button.0) {
            *bg = Color::srgb(0.4, 0.6, 0.8).into();
        } else if button.0 == Entity::PLACEHOLDER && locked_target.entity.is_none() {
            *bg = Color::srgb(0.4, 0.7, 0.4).into();
        } else {
            *bg = Color::srgb(0.15, 0.15, 0.2).into();
        }
    }
}

/// # Animation Tick
/// 
/// Calls and updates the position of orbitals and visuals, as well as 
fn animation_tick(
    mut orb_data: ResMut<GameData>,
    mut query: Query<(&mut Transform, &OrbitalId)>,
    time: Res<Time>,
) {
    let mut updates = HashMap::new();

    for (mut transform, OrbitalId(id)) in query {
        if let Some(orbital) = orb_data.orbitals.get(id) {
            let new_orb = orbital.take_step(time.delta_secs_f64(), &orb_data.orbitals);
            // update the transform of our object in the game
            transform.translation = new_orb.t.to_vec3();
            //transform.rotation = new_orb.w.to_quat()
            // Store the update for later.
            updates.insert(*id, new_orb);
        } else {
            info!("Could not find orbital {}!", id);
        }
    }
    // with all updates done, update our orbits.
    orb_data.orbitals = updates;

    // deal with game speed checking here.
    // if time since last tick is not enough, skip the tick.
    // With time having passed successfully,
    // for (mut transform, mut _mesh, OrbitalId(id)) in query.iter_mut() {
    //     // for now, this is a super simple calculation. only move the second object.
    //     if *id == 1 {
    //         let x = f32::sin(time.elapsed_secs() * TAU / 20.0) * 500.0;
    //         let y = f32::cos(time.elapsed_secs() * TAU / 20.0) * 500.0;
    //         transform.translation.x = x;
    //         transform.translation.y = y;
    //     }
    // }
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