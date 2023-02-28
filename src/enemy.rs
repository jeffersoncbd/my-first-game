use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    components::{Enemy, SpriteSize},
    GameTextures, WinSize, ENEMY_SIZE, SPRITES_SCALE,
};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_system);
    }
}

fn spawn_system(mut commands: Commands, game_texture: Res<GameTextures>, win_size: Res<WinSize>) {
    // compute x/y
    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - 100.;
    let h_span = win_size.h / 2. - 100.;
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);

    commands
        .spawn(SpriteBundle {
            texture: game_texture.enemy.clone(),
            transform: Transform {
                translation: Vec3 { x, y, z: 0. },
                scale: Vec3::new(SPRITES_SCALE, SPRITES_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(SpriteSize::from(ENEMY_SIZE));
}
