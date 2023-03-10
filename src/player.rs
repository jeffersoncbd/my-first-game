use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    components::{FromPlayer, Laser, Movable, Player, SpriteSize, Velocity},
    GameTextures, PlayerState, WinSize, PLAYER_LASER_SIZE, PLAYER_RESPAWN_DELAY, PLAYER_SIZE,
    SPRITES_SCALE,
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(spawn_system),
            )
            .add_system(keyboard_event_system)
            .add_system(fire_system);
    }
}

fn spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let now = time.elapsed_seconds_f64();
    let last_shot = player_state.last_shot;

    if !player_state.on && (last_shot == -1. || now > last_shot + PLAYER_RESPAWN_DELAY) {
        // add player
        let bottom = -win_size.h / 2.;
        commands
            .spawn(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        0.,
                        bottom + PLAYER_SIZE.1 / 2. * SPRITES_SCALE + 5.,
                        0.,
                    ),
                    scale: Vec3::new(SPRITES_SCALE, SPRITES_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player)
            .insert(SpriteSize::from(PLAYER_SIZE))
            .insert(Movable {
                auto_despawn: false,
            })
            .insert(Velocity { x: 0., y: 0. });

        player_state.spawned();
    }
}

fn fire_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if keyboard.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2. * SPRITES_SCALE - 5.;

            let mut spawn_laser = |x_offset: f32| {
                commands
                    .spawn(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y, 0.),
                            scale: Vec3::new(SPRITES_SCALE, SPRITES_SCALE, 0.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity { x: 0., y: 1. });
            };

            spawn_laser(x_offset);
            spawn_laser(-x_offset);
        }
    }
}

fn keyboard_event_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if keyboard.pressed(KeyCode::Left) {
            if velocity.x > -1. {
                velocity.x - 0.05
            } else {
                -1.
            }
        } else if keyboard.pressed(KeyCode::Right) {
            if velocity.x < 1. {
                velocity.x + 0.05
            } else {
                1.
            }
        } else {
            if velocity.x < -0.2 {
                velocity.x + 0.10
            } else if velocity.x > 0.2 {
                velocity.x - 0.10
            } else {
                0.
            }
        }
    }
}
