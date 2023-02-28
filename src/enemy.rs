use bevy::{prelude::*, time::FixedTimestep};
use rand::{thread_rng, Rng};

use crate::{
    components::{Enemy, SpriteSize},
    EnemyCount, GameTextures, WinSize, ENEMY_MAX, ENEMY_SIZE, SPRITES_SCALE,
};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(spawn_system),
        );
    }
}

fn spawn_system(
    mut commands: Commands,
    game_texture: Res<GameTextures>,
    mut enemy_count: ResMut<EnemyCount>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
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

        enemy_count.0 += 1;
    }
}
