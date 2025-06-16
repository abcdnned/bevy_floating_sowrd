use bevy::prelude::*;

pub const MOVEMENT_SPEED: f32 = 50.0;

#[derive(Component)]
pub struct PlayerMovement {
    pub speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(Color::srgb(0.0, 0.0, 1.0)))), // BLUE
        Transform::default().with_scale(Vec3::splat(30.)),
        PlayerMovement {
            speed: MOVEMENT_SPEED,
        },
    ));
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&PlayerMovement, &mut Transform)>,
) {
    for (player_movement, mut transform) in query.iter_mut() {
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            transform.translation.y += player_movement.speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= player_movement.speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= player_movement.speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            transform.translation.x += player_movement.speed * time.delta_secs();
        }
    }
}
