use crate::events::*;
use bevy::{app::AppExit, prelude::*};
use bevy::window::PrimaryWindow;

pub fn setup(mut commands: Commands, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();
    let w = window.width() / 2.0;
    let h = window.height() / 2.0;

    window.position = WindowPosition::Centered(MonitorSelection::Current);

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(w, h, 1.0),
        ..default()
    });
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score.to_string())
    }
}
