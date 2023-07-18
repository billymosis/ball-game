use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use rand::prelude::*;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0;
const NUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SPEED: f32 = 200.0;
const ENEMY_SIZE: f32 = 64.0;
const NUMBER_OF_STARS: usize = 10;
const STAR_SIZE: f32 = 30.0;
const STAR_SPAWN_TIME: f32 = 1.0;
const ENEMY_SPAWN_TIME: f32 = 5.0;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .init_resource::<HighScores>()
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, enemy_hit_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, tick_enemy_spawn_timer)
        .add_systems(Update, spawn_enemy_over_time)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, update_high_scores)
        .add_systems(Update, high_scores_updated)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}
impl Default for HighScores {
    fn default() -> Self {
        HighScores { scores: Vec::new() }
    }
}

#[derive(Resource)]
struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window_query.single_mut();
    let w = window.width() / 2.0;
    let h = window.height() / 2.0;

    window.position = WindowPosition::Centered(MonitorSelection::Current);

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(w, h, 1.0),
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ball_blue_large.png"),
            transform: Transform::from_xyz(w, h, 0.0),
            ..default()
        },
        Player {},
    ));
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    for _ in 0..NUMBER_OF_ENEMIES {
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
                direction: Vec2::new(random::<f32>(), random::<f32>()),
            },
        ));
    }
}

fn spawn_stars(
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

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;
        let mut translation = player_transform.translation;

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
        player_transform.translation = translation;
    }
}

fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

fn update_enemy_direction(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

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

fn confine_enemy_movement(
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

fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("GAME OVER");
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    ..default()
                });
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

fn enemy_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            if distance < player_radius + star_radius {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/laserLarge_000.ogg"),
                    ..default()
                });
                score.value += 1;
                commands.entity(star_entity).despawn();
            }
        }
    }
}

fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

fn tick_enemy_spawn_timer(mut star_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

fn spawn_stars_over_time(
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

fn spawn_enemy_over_time(
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

fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score.to_string())
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}