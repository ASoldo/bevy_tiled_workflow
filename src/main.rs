use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::{BLUE_500, GREEN_500},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
include!(concat!(env!("OUT_DIR"), "/generated_code.rs"));
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    LoadingScreen,
    MainMenu,
    Game,
    Menu,
}

impl GameState {
    /// Returns `true` if the game state is [`MainMenu`].
    ///
    /// [`MainMenu`]: GameState::MainMenu
    #[must_use]
    fn is_game(&self) -> bool {
        matches!(self, Self::Game)
    }
}

fn hello_bre() {
    dbg!("Oyyy cunt");
}

fn get_position_compomponent_system(query: Query<&PositionComponent>) {
    for comp in &query {
        dbg!(comp);
    }
}

#[derive(Component, Debug)]
struct PositionComponent {
    x: f32,
    y: f32,
    z: f32,
}

fn update_position_component_system(mut query: Query<(&mut PositionComponent, &Transform)>) {
    for (mut pos_comp, transform) in &mut query {
        pos_comp.x = transform.translation.x;
        pos_comp.y = transform.translation.y;
        pos_comp.z = transform.translation.z;
    }
}

fn draw_axes(mut gizmos: Gizmos, query: Query<&Transform, With<PositionComponent>>) {
    for &transform in &query {
        gizmos.axes(transform, 1.0)
    }
}

fn draw_line(mut gizmos: Gizmos, query: Query<&Transform, With<PositionComponent>>) {
    for obj in &query {
        gizmos.line(
            Vec3::new(10.0, 10.0, 0.0),
            Vec3::new(obj.translation.x, obj.translation.y, 0.0),
            GREEN_500,
        );
        gizmos.sphere(obj.translation, Quat::IDENTITY, 3.0, BLUE_500);
    }
}

fn toggle_game_state(
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
    real: Res<State<GameState>>,
) {
    if input.just_released(KeyCode::Space) {
        state.set(GameState::Game);
    }
    // if real.is_game() {
    //     dbg!("It's a Game!");
    // }
}

fn main() {
    App::new()
        // Enable physics
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            MaterialPlugin::<CustomMaterial>::default(),
            WorldInspectorPlugin::default(),
        ))
        // .add_plugins(TiledMapPlugin)
        .init_state::<GameState>()
        // .insert_state(GameState::Game)
        .add_systems(Startup, setup)
        .add_systems(
            Startup,
            hello_bre.run_if(in_state(GameState::LoadingScreen)),
        )
        .add_systems(
            Update,
            (
                draw_axes,
                toggle_game_state,
                draw_line.run_if(in_state(GameState::Game)),
            )
                .chain(),
        )
        // .add_systems(
        //     Update,
        //     (get_pos_comp, update_position_component_system).chain(),
        // )
        .run();
}

#[derive(Component)]
struct Tile {
    id: u32,
}
fn draw_map(commands: &mut Commands, tileset_handle: Handle<Image>, tileset_size: Vec2) {
    let tile_size = Vec2::new(MAP.tilewidth as f32, MAP.tileheight as f32);

    for layer in &MAP.layers {
        for (y, row) in layer.data.chunks(MAP.width as usize).enumerate() {
            for (x, &tile_id) in row.iter().enumerate() {
                if tile_id == 0 {
                    continue; // Skip empty tiles
                }

                let tile_position =
                    Vec3::new(x as f32 * tile_size.x, -(y as f32 * tile_size.y), 0.0);

                let tiles_per_row = (tileset_size.x / tile_size.x) as u32;
                let tile_column = (tile_id - 1) % tiles_per_row;
                let tile_row = (tile_id - 1) / tiles_per_row;

                let rect_min = Vec2::new(
                    tile_column as f32 * tile_size.x,
                    tile_row as f32 * tile_size.y,
                );
                let rect_max = rect_min + tile_size;

                commands
                    .spawn(SpriteBundle {
                        texture: tileset_handle.clone(),
                        sprite: Sprite {
                            custom_size: Some(tile_size),
                            rect: Some(Rect {
                                min: rect_min,
                                max: rect_max,
                            }),
                            ..Default::default()
                        },
                        transform: Transform {
                            translation: tile_position,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Tile { id: tile_id });
            }
        }
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    server: Res<AssetServer>,
) {
    println!("{:?}", *MAP);

    println!("{:?}", &MAP.layers);

    for layer in &MAP.layers {
        println!("{:?}", layer.data);
    }
    let tileset_handle: Handle<Image> = server.load("../assets/images/tileset1.png");
    draw_map(&mut commands, tileset_handle, Vec2::new(200.0, 200.0));
    // generated_function();
    // Static physics object with a collision shape
    // commands.spawn((
    //     RigidBody::Static,
    //     Collider::cylinder(4.0, 0.1),
    //     MaterialMeshBundle {
    //         mesh: meshes.add(Cylinder::new(4.0, 0.1)),
    //         material: materials.add(CustomMaterial {}),
    //         ..default()
    //     },
    // ));
    //
    // // Dynamic physics object with a collision shape and initial angular velocity
    // commands.spawn((
    //     PositionComponent {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 0.0,
    //     },
    //     Name::new("Kocka"),
    //     RigidBody::Dynamic,
    //     Collider::cuboid(1.0, 1.0, 1.0),
    //     AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
    //     MaterialMeshBundle {
    //         mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    //         material: materials.add(CustomMaterial {}),
    //         transform: Transform::from_xyz(0.0, 4.0, 0.0),
    //         ..default()
    //     },
    // ));
    //
    // // Light
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });
    //
    // // Camera
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
    //     ..default()
    // });
    commands.spawn(Camera2dBundle::default());
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "../assets/shaders/shader.wgsl".into()
    }
}
