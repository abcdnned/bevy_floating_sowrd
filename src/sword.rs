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
pub struct SwordNode {
    pub locked_position: Option<Vec2>, // Store locked position during swing
    pub offset: Vec2, // Offset to maintain after swing completion
}

impl Default for SwordNode {
    fn default() -> Self {
        Self {
            locked_position: None,
            offset: Vec2::ZERO, // Initially no offset
        }
    }
}

pub struct SwordPlugin;

impl Plugin for SwordPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sword_with_node).add_systems(
            Update,
            (update_node_position, check_swing_status).chain(),
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
            SwordNode::default(),
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

// Check swing status and manage position locking
fn check_swing_status(
    mut node_query: Query<(&mut SwordNode, &Children, &Transform)>,
    swing_query: Query<&SwingAnimation>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    for (mut sword_node, children, node_transform) in node_query.iter_mut() {
        // Find the sword child and check its swing status
        let mut is_currently_swinging = false;
        let mut was_swinging = sword_node.locked_position.is_some();
        
        for child in children.iter() {
            if let Ok(swing_animation) = swing_query.get(child) {
                is_currently_swinging = swing_animation.is_swinging;
                break;
            }
        }

        // If starting to swing and not already locked, lock the current mouse position
        if is_currently_swinging && sword_node.locked_position.is_none() {
            if let (Ok(window), Ok((camera, camera_transform))) = 
                (window_query.single(), camera_query.single()) {
                if let Some(cursor_pos) = window.cursor_position() {
                    if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                        sword_node.locked_position = Some(world_pos);
                    }
                }
            }
        }
        
        // If swing just finished, calculate and set the offset
        if was_swinging && !is_currently_swinging && sword_node.locked_position.is_some() {
            if let (Ok(window), Ok((camera, camera_transform))) = 
                (window_query.single(), camera_query.single()) {
                if let Some(cursor_pos) = window.cursor_position() {
                    if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                        // Calculate offset: mouse position - node position
                        let node_pos = Vec2::new(node_transform.translation.x, node_transform.translation.y);
                        sword_node.offset = world_pos - node_pos;
                    }
                }
            }
            
            // Clear the lock
            sword_node.locked_position = None;
        }
    }
}

// Update the node position to follow the mouse (only when not locked)
fn update_node_position(
    mut node_query: Query<(&mut Transform, &mut SwordNode), Without<Sword>>,
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

    for (mut transform, mut sword_node) in node_query.iter_mut() {
        // If we have a locked position, use that instead of mouse position
        if let Some(locked_pos) = sword_node.locked_position {
            let target_pos = locked_pos - sword_node.offset;
            transform.translation.x = target_pos.x;
            transform.translation.y = target_pos.y;
            
        } else {
            // Normal mouse following when not locked, but apply the offset
            let target_pos = world_pos - sword_node.offset;
            transform.translation.x = target_pos.x;
            transform.translation.y = target_pos.y;
        }
    }
}