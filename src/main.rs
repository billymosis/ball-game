use bevy::prelude::*;

pub mod events;
mod systems;

pub mod enemy;
mod player;
pub mod score;
pub mod star;

use enemy::EnemyPlugin;
use events::*;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_systems(Startup, setup)
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(StarPlugin)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}
