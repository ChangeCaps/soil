mod isometric;

use bevy::prelude::*;
use std::collections::HashMap;

pub const TEXTURE_SCALE: Scale = Scale(4.0);
pub const TILE_SIZE: f32 = 48.0;
pub const YSORT_Z_RATIO: f32 = 1.0 / 1024.0;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Iso {
    pub x: i32,
    pub y: i32,
}

impl Iso {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn pos(&self) -> Vec2 {
        isometric::from_iso(*self)
    }
}

impl Into<Vec2> for Iso {
    fn into(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

struct Floating;

struct MapManager {
    tiles: HashMap<Iso, Entity>,
}

impl MapManager {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_resource(MapManager::new())
        .add_startup_system(setup.system())
        .add_system(floating_system.system())
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_manager: ResMut<MapManager>,
) {
    commands.spawn(Camera2dComponents {
        translation: Translation::new(0.0, 0.0, 128.0),
        ..Default::default()
    });

    for x in -2..3 {
        for y in -2..3 {
            let iso = Iso::new(x, y);
            let pos = isometric::from_iso(iso);

            let entity = commands
                .spawn(SpriteComponents {
                    material: materials.add(
                        asset_server
                            .load("assets/sprites/tile_roots.png")
                            .unwrap()
                            .into(),
                    ),
                    translation: Translation::new(pos.x(), pos.y(), -pos.y() * YSORT_Z_RATIO),
                    scale: TEXTURE_SCALE,
                    ..Default::default()
                })
                .with(iso)
                .with(Floating)
                .current_entity()
                .unwrap();

            map_manager.tiles.insert(iso, entity);
        }
    }
}

fn floating_system(time: Res<Time>, mut query: Query<(&Floating, &Iso, &mut Translation)>) {
    for (_floating, iso_pos, mut translation) in &mut query.iter() {
        translation.set_y(iso_pos.pos().y() + time.seconds_since_startup.sin() as f32 * 4.0)
    }
}
