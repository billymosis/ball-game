use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use super::{ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES};

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    for _ in 0..NUMBER_OF_ENEMIES {
        // Random between 0.0 to 1.0
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();
        let direction = Vec2::new(random::<f32>(), random::<f32>()).normalize();
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/ball_red_large.png"),
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                ..default()
            },
            Enemy { direction },
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        let pos1 = Vec2::new(transform.translation.x, transform.translation.y);
        let pos2 = pos1 + (direction.normalize_or_zero().truncate() * 50.0); // Calculate pos2 based on the normalized direction vector
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();

        gizmos.line_2d(pos1, pos2, Color::RED);
        // Calculate arrowhead points
        let arrowhead_length = 10.0; // Length of the arrowhead lines
        let arrowhead_width = 5.0; // Width of the arrowhead lines
        let perp_direction = Vec2::new(-direction.y, direction.x); // Calculate perpendicular direction

        let arrowhead_point1 =
            pos2 - (direction.truncate() * arrowhead_length) + (perp_direction * arrowhead_width);
        let arrowhead_point2 =
            pos2 - (direction.truncate() * arrowhead_length) - (perp_direction * arrowhead_width);

        // Draw arrowhead lines
        gizmos.line_2d(pos2, arrowhead_point1, Color::RED);
        gizmos.line_2d(pos2, arrowhead_point2, Color::RED);
    }
}

pub fn spawn_enemy_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.single();
        // Random between 0.0 to 1.0
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/ball_red_large.png"),
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn tick_enemy_spawn_timer(mut star_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn update_enemy_direction(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_max = window.height() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            commands.spawn(AudioBundle {
                source: sound_effect,
                ..default()
            });
        };
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;
    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}
