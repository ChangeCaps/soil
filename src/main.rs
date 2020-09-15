mod isometric;

use bevy::prelude::*;

pub const TEXTURE_SCALE: Scale = Scale(4.0);
pub const TILE_SIZE: f32 = 32.0;

struct HexPosition {
    pos: Vec2,
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dComponents {
        ..Default::default()
    });

    for x in -1..2 {
        for y in -1..2 {
            let pos = isometric::to_iso(Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE));
            
            commands.spawn(SpriteComponents {
                material: materials
                    .add(asset_server.load("assets/sprites/tile.png").unwrap().into()),
                translation: Translation::new(pos.x(), pos.y(), pos.y()),
                scale: TEXTURE_SCALE,
                ..Default::default()
            });
        }
    }
}
