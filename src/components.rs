use bevy::math::Vec2;
use bevy::prelude::*;

// region:      --- Common Components
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);
impl From<(f32, f32)> for SpriteSize {
    fn from(values: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(values.0, values.1))
    }
}
// endregion:   --- Common Components

// region:      --- Player Components
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct FromPlayer;
// endregion:   --- Player Components

// region:      --- Enemy Components
#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct FromEnemy;
// endregion:   --- Enemy Components

// region:      --- Explosion Components
#[derive(Component)]
pub struct Explosion;
#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);
// endregion:   --- Explosion Components