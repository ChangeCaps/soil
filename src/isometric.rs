use bevy::prelude::*;

pub fn from_iso(iso: impl Into<Vec2>) -> Vec2 {
    let iso = iso.into();

    Vec2::new(
        iso.x() * 2.0 * crate::TILE_SIZE - iso.y() * 2.0 * crate::TILE_SIZE,
        iso.x() * crate::TILE_SIZE + iso.y() * crate::TILE_SIZE,
    )
}

pub fn to_iso(screen: Vec2) -> Vec2 {
    Vec2::new(
        screen.y() * 1.0 + screen.x() * 1.0,
        screen.y() * 2.0 - screen.x() * 2.0,
    )
}
