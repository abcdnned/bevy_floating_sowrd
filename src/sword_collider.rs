use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::sword::Sword;

pub struct SwordColliderPlugin;

impl Plugin for SwordColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin {
                enabled: true,
                ..default()
            })
            .add_systems(PostStartup, setup_physics)
            .add_systems(Update, handle_collisions);
    }
}

fn setup_physics(
    mut commands: Commands,
    sword_query: Query<(Entity, &Sword), Without<RigidBody>>,
) {
    for (entity, sword) in sword_query.iter() {
        commands.entity(entity)
            .insert(RigidBody::KinematicPositionBased)
            .insert(Collider::cuboid(10., 30.))
            .insert(Sensor) // Optional: makes it a sensor (no collision response, just detection)
            .insert(ActiveEvents::COLLISION_EVENTS); // Enable collision events for this entity
    }
}

fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
) {
    // Handle Rapier's built-in collision events
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                println!("Collision started between {:?} and {:?}", entity1, entity2);
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                println!("Collision stopped between {:?} and {:?}", entity1, entity2);
            }
        }
    }
}