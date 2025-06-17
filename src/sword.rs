use crate::swing_animation::SwingAnimation;
use crate::swing_animation::SwingType;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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
                timer: Timer::from_seconds(0.8, TimerMode::Once),
                start_pos: Vec2::ZERO,
                start_rotation: 0.0,
                is_swinging: false,
                swing_type: SwingType::Horizontal,
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
