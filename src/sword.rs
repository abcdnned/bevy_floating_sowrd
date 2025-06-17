use crate::swing_animation::SwingAnimation;
use crate::swing_animation::SwingType;
use crate::swing_animation::SwingPhase;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Sword {
    pub offset: Vec2,
}

impl Default for Sword {
    fn default() -> Self {
        Self {
            offset: Vec2::new(20.0, -10.0), // Default offset from node position
        }
    }
}

// New component to mark the intermediate node
#[derive(Component)]
pub struct SwordNode;

pub struct SwordPlugin;

impl Plugin for SwordPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sword_with_node).add_systems(
            Update,
            (update_node_position, update_sword_position).chain(),
        );
    }
}

fn spawn_sword_with_node(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Sword sprite with texture atlas
    let texture = asset_server.load("sword.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 1, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Spawn the intermediate node (invisible parent)
    let node_entity = commands
        .spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::default(),
            SwordNode,
        ))
        .id();

    // Spawn the sword as a child of the node
    let sword_entity = commands
        .spawn((
            Sprite {
                image: texture,
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                }),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1.0), // Initial position relative to node
        SwingAnimation {
            timer: Timer::from_seconds(0.8, TimerMode::Once), // Total animation time (legacy)
            
            // Phase 1: Startup - Move to attack position
            startup_timer: Timer::from_seconds(0.1, TimerMode::Once), // 200ms to reach attack position
            start_pos: Vec2::new(-20.0, 60.0), // Back and up for windup
            start_rotation: -PI * 0.2, // Rotated back (~-54 degrees)
            
            // Phase 2: Main swing
            swing_timer: Timer::from_seconds(0.4, TimerMode::Once), // 400ms for main swing
            
            // Phase 3: Recovery - Move to rest position  
            end_timer: Timer::from_seconds(0.2, TimerMode::Once), // 200ms to reach rest position
            // State tracking
            is_swinging: false,
            swing_type: SwingType::Vertical,
            current_phase: SwingPhase::Startup, // Will be set properly in start_swing()
            swing_end_pos: Vec2::ZERO,
            swing_end_rotation: 0.0,
        },
            Sword::default(),
        ))
        .id();

    // Make sword a child of the node
    commands.entity(node_entity).add_child(sword_entity);
}

// Update the node position to follow the mouse
fn update_node_position(
    mut node_query: Query<&mut Transform, (With<SwordNode>, Without<Sword>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    for mut transform in node_query.iter_mut() {
        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
    }
}

// Update sword position relative to its parent node
fn update_sword_position(mut sword_query: Query<(&mut Transform, &Sword), Without<SwordNode>>) {
    // for (mut transform, sword) in sword_query.iter_mut() {
    //     transform.translation.x = sword.offset.x;
    //     transform.translation.y = sword.offset.y;
    // }
}
