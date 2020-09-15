use bevy::prelude::*;

pub fn from_iso(iso: Vec2) -> Vec2 {
    Vec2::new(iso.y() * 1.0 + iso.x() * 1.0, iso.y() * 2.0 - iso.x() * 2.0)
}

pub fn to_iso(screen: Vec2) -> Vec2 {
    Vec2::new(screen.x() - screen.y() * 2.0, screen.x() + screen.y() * 2.0)
}
