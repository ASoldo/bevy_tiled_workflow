use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
include!(concat!(env!("OUT_DIR"), "/generated_code.rs"));

#[derive(Component)]
struct Tile {
    id: u32,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn draw_map(commands: &mut Commands, tileset_handle: Handle<Image>, tileset_size: Vec2) {
    let tile_size = Vec2::new(MAP.tilewidth as f32, MAP.tileheight as f32);

    for layer in &MAP.layers {
        for (y, row) in layer.data.chunks(MAP.width as usize).enumerate() {
            for (x, &tile_id) in row.iter().enumerate() {
                if tile_id == 96 {
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

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    println!("{:?}", *MAP);

    println!("{:?}", &MAP.layers);

    for layer in &MAP.layers {
        println!("{:?}", layer.data);
    }
    let tileset_handle: Handle<Image> = server.load("../assets/images/tileset1.png");
    draw_map(&mut commands, tileset_handle, Vec2::new(200.0, 200.0));

    commands.spawn(Camera2dBundle::default());
}
