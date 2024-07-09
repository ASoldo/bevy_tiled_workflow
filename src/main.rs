use bevy::color::Srgba;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
include!(concat!(env!("OUT_DIR"), "/generated_code.rs"));

#[derive(Component, Reflect, Resource, Default)]
#[reflect(Resource)]
struct Tile {
    id: u32,
}

#[derive(Component)]
struct MapObject {
    id: u32,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::default(),
        ))
        .init_resource::<Tile>()
        .register_type::<Tile>()
        .add_systems(Startup, setup)
        .run();
}

fn draw_map(commands: &mut Commands, server: Res<AssetServer>, tileset_size: Vec2) {
    let tile_size = Vec2::new(MAP.tilewidth as f32, MAP.tileheight as f32);

    let tileset_path = &MAP.tilesets[0].image_source.clone();
    let tileset_handle: Handle<Image> = server.load(tileset_path);
    for layer in &MAP.layers {
        for (y, row) in layer.data.chunks(MAP.width as usize).enumerate() {
            for (x, &tile_id) in row.iter().enumerate() {
                if tile_id >= 96 {
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

    for object_group in &MAP.object_groups {
        for object in &object_group.objects {
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(1.0, 0.0, 0.0, 0.5),
                        custom_size: Some(Vec2::new(20.0, 20.0)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(object.x, -object.y, 1.0), // Adjust as needed
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(MapObject { id: object.id });
        }
    }
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    println!("{:?}", *MAP);

    println!("{:?}", &MAP.layers);

    for layer in &MAP.layers {
        println!("{:?}", layer.data);
    }

    draw_map(&mut commands, server, Vec2::new(200.0, 200.0));

    commands.spawn(Camera2dBundle::default());
}
