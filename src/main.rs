mod cursor;
mod enemy;
mod player;
mod swing_animation;
mod sword;
mod sword_collider;

use crate::cursor::CursorPlugin;
use crate::enemy::{EnemyPlugin, EnemySpawner};
use crate::player::PlayerPlugin;
use crate::swing_animation::SwingAnimationPlugin;
use crate::sword::SwordPlugin;
use crate::sword_collider::SwordColliderPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            // CursorPlugin, // Handles cursor hiding/showing
            SwordPlugin,  // Handles sword following mouse
            SwordColliderPlugin,
            PlayerPlugin,
            SwingAnimationPlugin,
            // EnemyPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Spawn an enemy spawner
    commands.spawn(EnemySpawner {
        cooldown: 2.0, // Spawn enemy every 2 seconds
        timer: 0.0,
    });
}
