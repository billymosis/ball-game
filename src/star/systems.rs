use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::NUMBER_OF_STARS;
use super::components::Star;
use super::resources::StarSpawnTimer;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    for _ in 0..NUMBER_OF_STARS {
        // Random between 0.0 to 1.0
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/star.png"),
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.single();
        // Random between 0.0 to 1.0
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/star.png"),
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                ..default()
            },
            Star {},
        ));
    }
}
