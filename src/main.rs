use bevy::prelude::*;
use bevy_ecs_tilemap::*;
use bevy_ecs_tilemap::map::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod helpers;

// MESSING AROUND FROM https://github.com/StarArawn/bevy_ecs_tilemap/blob/b30b1fd215447d3dfa243e06a1ff76a8be8bce4d/examples/colors.rs

// Side length of a colored quadrant (in "number of tiles").
const QUADRANT_SIDE_LENGTH: u32 = 64;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    // In total, there will be `(QUADRANT_SIDE_LENGTH * 2) * (QUADRANT_SIDE_LENGTH * 2)` tiles.
    let map_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH * 2,
        y: QUADRANT_SIDE_LENGTH * 2,
    };
    let quadrant_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH,
        y: QUADRANT_SIDE_LENGTH,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    fill_tilemap_rect_color(
        TileTextureIndex(5),
        TilePos { x: 0, y: 0 },
        quadrant_size,
        Color::rgba(1.0, 0.0, 0.0, 1.0),
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect_color(
        TileTextureIndex(5),
        TilePos {
            x: QUADRANT_SIDE_LENGTH,
            y: 0,
        },
        quadrant_size,
        Color::rgba(0.0, 1.0, 0.0, 1.0),
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect_color(
        TileTextureIndex(5),
        TilePos {
            x: 0,
            y: QUADRANT_SIDE_LENGTH,
        },
        quadrant_size,
        Color::rgba(0.0, 0.0, 1.0, 1.0),
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect_color(
        TileTextureIndex(5),
        TilePos {
            x: QUADRANT_SIDE_LENGTH,
            y: QUADRANT_SIDE_LENGTH,
        },
        quadrant_size,
        Color::rgba(1.0, 1.0, 0.0, 1.0),
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        map_type: TilemapType::Square,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Topgallant"),
                ..Default::default()
            }),
            ..default()
        }))
        .add_plugin(bevy_ecs_tilemap::TilemapPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(startup)
        .add_system(helpers::camera::movement)
        .run();
}
