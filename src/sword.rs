use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct Sword {
    pub offset: Vec2,
}

impl Default for Sword {
    fn default() -> Self {
        Self {
            offset: Vec2::new(20.0, -10.0), // Default offset from mouse cursor
        }
    }
}

pub struct SwordPlugin;

impl Plugin for SwordPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sword)
            .add_systems(Update, update_sword_position);
    }
}

fn spawn_sword(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Sword sprite with texture atlas
    let texture = asset_server.load("sword.png");
    // The sprite sheet has 2 sprites arranged in a row, and they are all 64px x 64px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 1, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Sword::default(),
    ));
}

fn update_sword_position(
    mut sword_query: Query<(&mut Transform, &Sword)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = window_query.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else { return };
    let Ok((camera, camera_transform)) = camera_query.single() else { return };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return };
  
    for (mut transform, sword) in sword_query.iter_mut() {
        transform.translation.x = world_pos.x + sword.offset.x;
        transform.translation.y = world_pos.y + sword.offset.y;
    }
}