use bevy::prelude::*;
use bevy_tween::prelude::*;
use image::Rgb;

// Component to hold the parsed arc path
#[derive(Component)]
struct ArcPath {
    points: Vec<Vec2>,
}

// Custom interpolator for arc motion
#[derive(Debug, Clone)]
struct ArcMotionLens {
    start_pos: Vec3,
    arc_points: Vec<Vec2>,
}

impl Interpolator for ArcMotionLens {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        if self.arc_points.is_empty() {
            return;
        }
        
        // Calculate which point we should be at based on value (0.0 to 1.0)
        let total_points = self.arc_points.len();
        let float_index = value * (total_points - 1) as f32;
        let index = float_index.floor() as usize;
        let next_index = (index + 1).min(total_points - 1);
        let local_ratio = float_index - index as f32;
        
        // Interpolate between current and next point
        let current_point = self.arc_points[index];
        let next_point = self.arc_points[next_index];
        let interpolated = current_point.lerp(next_point, local_ratio);
        
        // Update transform position
        item.translation = Vec3::new(interpolated.x, interpolated.y, self.start_pos.z);
        
        // Optional: Calculate rotation based on movement direction
        if index < total_points - 1 && next_point != current_point {
            let direction = (next_point - current_point).normalize();
            let angle = direction.y.atan2(direction.x);
            item.rotation = Quat::from_rotation_z(angle);
        }
    }
}

// System to parse arc line from image
fn parse_arc_from_image(image_path: &str) -> Result<Vec<Vec2>, Box<dyn std::error::Error>> {
    let img = image::open(image_path)?;
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    
    let mut arc_points = Vec::new();
    
    // Define what color represents the arc line (e.g., black pixels)
    let target_color = Rgb([0, 0, 0]); // Black line
    let tolerance = 50; // Color tolerance
    
    // Scan the image to find arc pixels
    for y in 0..height {
        for x in 0..width {
            let pixel = rgb_img.get_pixel(x, y);
            
            // Check if pixel matches our target color (with tolerance)
            let color_diff = ((pixel[0] as i32 - target_color[0] as i32).abs() +
                             (pixel[1] as i32 - target_color[1] as i32).abs() +
                             (pixel[2] as i32 - target_color[2] as i32).abs()) as u32;
            
            if color_diff < tolerance * 3 {
                // Convert image coordinates to world coordinates
                // Assuming image center is (0,0) in world space
                let world_x = (x as f32 - width as f32 / 2.0) * 0.01; // Scale factor
                let world_y = (height as f32 / 2.0 - y as f32) * 0.01; // Flip Y axis
                arc_points.push(Vec2::new(world_x, world_y));
            }
        }
    }
    
    // Sort points to create a proper path (you might need more sophisticated ordering)
    arc_points.sort_by(|a, b| {
        // Sort by angle from center for circular arcs
        let center = Vec2::ZERO;
        let angle_a = (a.y - center.y).atan2(a.x - center.x);
        let angle_b = (b.y - center.y).atan2(b.x - center.x);
        angle_a.partial_cmp(&angle_b).unwrap()
    });
    
    Ok(arc_points)
}

// Alternative: More sophisticated path ordering using nearest neighbor
fn order_arc_points(points: Vec<Vec2>) -> Vec<Vec2> {
    if points.len() <= 1 {
        return points;
    }
    
    let mut ordered = Vec::new();
    let mut remaining = points;
    
    // Start with leftmost point
    let start_idx = remaining.iter()
        .enumerate()
        .min_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap())
        .unwrap().0;
    
    ordered.push(remaining.remove(start_idx));
    
    // Connect nearest unvisited points
    while !remaining.is_empty() {
        let current = *ordered.last().unwrap();
        let (nearest_idx, _) = remaining.iter()
            .enumerate()
            .map(|(i, &p)| (i, current.distance(p)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();
        
        ordered.push(remaining.remove(nearest_idx));
    }
    
    ordered
}

// System to create sword swing animation
fn setup_sword_swing(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Setup camera
    commands.spawn(Camera2d);
    
    // Load and parse arc from image
    let arc_points = match parse_arc_from_image("assets/sword_arc.png") {
        Ok(points) => order_arc_points(points),
        Err(e) => {
            warn!("Failed to load arc image: {}", e);
            // Fallback: create a simple arc manually
            create_fallback_arc()
        }
    };
    
    // Create the arc motion interpolator
    let arc_interpolator = ArcMotionLens {
        start_pos: Vec3::ZERO,
        arc_points: arc_points.clone(),
    };
    
    // Create tween with bevy_tween
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs(2),
        arc_interpolator,
    );
    
    // Spawn sword entity
    commands.spawn((
        Sprite::from_image(asset_server.load("sword.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        ArcPath {
            points: arc_points,
        },
        Animator::new(tween),
    ));
}

// Fallback arc creation
fn create_fallback_arc() -> Vec<Vec2> {
    let mut points = Vec::new();
    let center = Vec2::new(0.0, -200.0);
    let radius = 300.0;
    let start_angle = -std::f32::consts::PI / 4.0; // -45 degrees
    let end_angle = std::f32::consts::PI / 4.0;    // 45 degrees
    let steps = 30;
    
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let angle = start_angle + (end_angle - start_angle) * t;
        let point = center + Vec2::new(angle.cos(), angle.sin()) * radius;
        points.push(point);
    }
    
    points
}

// System to trigger sword swing on input
fn trigger_sword_swing(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut animators: Query<&mut Animator<Transform>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut animator in animators.iter_mut() {
            // Reset and restart the animation
            animator.set_progress(0.0);
        }
    }
    
    // Pause/resume with P key
    if keyboard.just_pressed(KeyCode::KeyP) {
        for mut animator in animators.iter_mut() {
            if animator.is_paused() {
                animator.resume();
            } else {
                animator.pause();
            }
        }
    }
    
    // Stop animation with S key
    if keyboard.just_pressed(KeyCode::KeyS) {
        for mut animator in animators.iter_mut() {
            animator.pause();
            animator.set_progress(0.0);
        }
    }
}

// System to visualize the arc path (debug)
fn debug_draw_arc(
    mut gizmos: Gizmos,
    query: Query<&ArcPath>,
) {
    for arc_path in query.iter() {
        // Draw lines between consecutive points
        for window in arc_path.points.windows(2) {
            let start = Vec3::new(window[0].x, window[0].y, 0.0);
            let end = Vec3::new(window[1].x, window[1].y, 0.0);
            gizmos.line(start, end, Color::srgb(1.0, 0.0, 0.0));
        }
        
        // Draw points
        for point in &arc_path.points {
            gizmos.circle(Vec3::new(point.x, point.y, 0.1), Vec3::Z, 5.0, Color::srgb(0.0, 1.0, 0.0));
        }
    }
}

// Plugin setup
pub struct SwordSwingPlugin;

impl Plugin for SwordSwingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultTweenPlugins)
            .add_systems(Startup, setup_sword_swing)
            .add_systems(Update, (trigger_sword_swing, debug_draw_arc));
    }
}

// Example usage in main.rs:
/*
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SwordSwingPlugin)
        .run();
}
*/

// Controls:
// Space - Start/restart sword swing animation
// P - Pause/resume animation
// S - Stop and reset animation