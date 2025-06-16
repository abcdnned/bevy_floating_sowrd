use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::player::PlayerMovement;

#[derive(Component)]
pub struct EnemySpawner {
    pub cooldown: f32,
    pub timer: f32,
}

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_spawning, update_enemies));
    }
}

pub fn update_spawning(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut spawner_query: Query<&mut EnemySpawner>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for mut spawner in spawner_query.iter_mut() {
        spawner.timer -= time.delta_secs();
        if spawner.timer <= 0. {
            let Ok(primary) = primary_query.single() else {
                return;
            };

            spawner.timer = spawner.cooldown;

            let mut spawn_transform = Transform::default().with_scale(Vec3::splat(30.));

            let mut rng = rand::thread_rng();

            if rng.gen_range(0..2) == 1 {
                if rng.gen_range(0..2) == 1 {
                    spawn_transform.translation = Vec3::new(
                        primary.width() / 2.,
                        rng.gen_range(-primary.height() / 2.0..primary.height() / 2.0),
                        0.,
                    );
                } else {
                    spawn_transform.translation = Vec3::new(
                        -primary.width() / 2.,
                        rng.gen_range(-primary.height() / 2.0..primary.height() / 2.0),
                        0.,
                    );
                }
            } else if rng.gen_range(0..2) == 1 {
                spawn_transform.translation = Vec3::new(
                    rng.gen_range(-primary.width() / 2.0..primary.width() / 2.0),
                    primary.height() / 2.,
                    0.,
                );
            } else {
                spawn_transform.translation = Vec3::new(
                    rng.gen_range(-primary.width() / 2.0..primary.width() / 2.0),
                    -primary.height() / 2.,
                    0.,
                );
            }

            commands.spawn((
                Enemy {
                    health: 100.0,
                    speed: 50.0,
                },
                Mesh2d(meshes.add(Circle::default())),
                MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))), // Red color
                RigidBody::Dynamic,
                Collider::ball(0.5),
                GravityScale(0.0),
                spawn_transform,
            ));
        }
    }
}

pub fn update_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&Enemy, &mut Transform, Entity), Without<PlayerMovement>>,
    player_query: Query<(&PlayerMovement, &Transform), Without<Enemy>>,
    mut commands: Commands,
) {
    if let Ok((_player_movement, player_transform)) = player_query.single() {
        for (enemy, mut transform, entity) in enemy_query.iter_mut() {
            let moving = Vec3::normalize(player_transform.translation - transform.translation)
                * enemy.speed
                * time.delta_secs();
            transform.translation += moving;
            if enemy.health <= 0. {
                commands.entity(entity).despawn();
            }
        }
    }
}
