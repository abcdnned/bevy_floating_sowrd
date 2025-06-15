mod sword;
mod cursor;
mod player;
mod enemy;
mod sword_collider;
mod arc_motion;

use crate::player::PlayerPlugin;
use crate::sword::SwordPlugin;
use crate::sword_collider::SwordColliderPlugin;
use crate::cursor::CursorPlugin;
use crate::enemy::{EnemyPlugin, EnemySpawner};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            CursorPlugin,  // Handles cursor hiding/showing
            SwordPlugin,   // Handles sword following mouse
            SwordColliderPlugin,
            PlayerPlugin,
            EnemyPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);

    // Spawn an enemy spawner
    commands.spawn(EnemySpawner {
        cooldown: 2.0,  // Spawn enemy every 2 seconds
        timer: 0.0,
    });
}