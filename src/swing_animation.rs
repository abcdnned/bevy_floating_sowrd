use crate::sword::Sword;
use bevy::prelude::*;
use nalgebra::{Point2, Vector2};
use std::f32::consts::PI;

pub struct SwingAnimationPlugin;

impl Plugin for SwingAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (animate_sword_swing, handle_input));
    }
}

#[derive(Component)]
pub struct SwingAnimation {
    pub timer: Timer,
    pub start_pos: Vec2,
    pub start_rotation: f32,
    pub is_swinging: bool,
    pub swing_type: SwingType,
}

#[derive(Clone)]
pub enum SwingType {
    Horizontal,
    Vertical,
    Diagonal,
    Circular,
}

fn setup() {
    // Instructions: Press H (Horizontal), V (Vertical), D (Diagonal), C (Circular)
    println!("Controls: Press H (Horizontal), V (Vertical), D (Diagonal), C (Circular)");
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut sword_query: Query<&mut SwingAnimation, With<Sword>>,
) {
    if let Ok(mut swing) = sword_query.single_mut() {
        if !swing.is_swinging {
            if keyboard.just_pressed(KeyCode::KeyH) {
                start_swing(&mut swing, SwingType::Horizontal);
            } else if keyboard.just_pressed(KeyCode::KeyV) {
                start_swing(&mut swing, SwingType::Vertical);
            } else if keyboard.just_pressed(KeyCode::KeyD) {
                start_swing(&mut swing, SwingType::Diagonal);
            } else if keyboard.just_pressed(KeyCode::KeyC) {
                start_swing(&mut swing, SwingType::Circular);
            }
        }
    }
}

fn start_swing(swing: &mut SwingAnimation, swing_type: SwingType) {
    swing.is_swinging = true;
    swing.timer.reset();
    swing.swing_type = swing_type;
}

fn animate_sword_swing(
    time: Res<Time>,
    mut sword_query: Query<(&mut Transform, &mut SwingAnimation), With<Sword>>,
) {
    for (mut transform, mut swing) in sword_query.iter_mut() {
        if swing.is_swinging {
            swing.timer.tick(time.delta());
            let progress = swing.timer.elapsed_secs() / swing.timer.duration().as_secs_f32();

            if progress >= 1.0 {
                swing.is_swinging = false;
                // Reset to neutral position
                transform.translation = Vec3::new(0.0, 0.0, 0.0);
                transform.rotation = Quat::from_rotation_z(0.0);
            } else {
                // Use different curves based on swing type
                let (position, rotation) = match swing.swing_type {
                    SwingType::Horizontal => calculate_horizontal_swing(progress),
                    SwingType::Vertical => calculate_vertical_swing(progress),
                    SwingType::Diagonal => calculate_diagonal_swing(progress),
                    SwingType::Circular => calculate_circular_swing(progress),
                };

                transform.translation = Vec3::new(position.x, position.y, 0.0);
                transform.rotation = Quat::from_rotation_z(rotation);
            }
        }
    }
}

fn calculate_horizontal_swing(t: f32) -> (Vec2, f32) {
    // Horizontal arc swing from left to right
    let progress = smooth_step(t);
    let angle = lerp(-PI * 0.4, PI * 0.4, progress);

    // Use nalgebra for 2D calculations
    let radius = 60.0;
    let center = Point2::new(0.0, -20.0);

    // Calculate position on arc
    let x = angle.sin() * radius;
    let y = center.y + (1.0 - angle.cos()) * 20.0; // Arc motion

    let position = Vec2::new(x, y);
    let rotation = angle + PI * 0.5; // Sword follows the arc

    (position, rotation)
}

fn calculate_vertical_swing(t: f32) -> (Vec2, f32) {
    // Vertical swing from top to bottom
    let progress = smooth_step(t);

    // Use nalgebra for bezier curve
    let p0 = Point2::new(0.0, 80.0); // Start position (high)
    let p1 = Point2::new(15.0, 40.0); // Control point (slight curve)
    let p2 = Point2::new(0.0, -40.0); // End position (low)

    // Quadratic bezier curve
    let pos = quadratic_bezier(p0, p1, p2, progress);
    let position = Vec2::new(pos.x, pos.y);

    // Rotation follows the swing direction
    let rotation = lerp(-PI * 0.2, PI * 0.7, progress);

    (position, rotation)
}

fn calculate_diagonal_swing(t: f32) -> (Vec2, f32) {
    // Diagonal slash from upper-left to lower-right
    let progress = smooth_step(t);

    // Create a curved diagonal path using nalgebra
    let start = Point2::new(-60.0, 60.0);
    let control1 = Point2::new(-20.0, 20.0);
    let control2 = Point2::new(20.0, -20.0);
    let end = Point2::new(60.0, -60.0);

    // Cubic bezier curve
    let pos = cubic_bezier(start, control1, control2, end, progress);
    let position = Vec2::new(pos.x, pos.y);

    // Rotation follows the diagonal direction
    let rotation = lerp(PI * 0.75, -PI * 0.25, progress);

    (position, rotation)
}

fn calculate_circular_swing(t: f32) -> (Vec2, f32) {
    // Full circular swing
    let progress = ease_in_out_cubic(t);
    let angle = progress * PI * 2.0; // Full circle

    // Use nalgebra for circular motion
    let radius = 50.0;
    let center = Point2::new(0.0, -10.0);

    let x = center.x + angle.cos() * radius;
    let y = center.y + angle.sin() * radius;

    let position = Vec2::new(x, y);
    let rotation = angle + PI * 0.5; // Sword tangent to circle

    (position, rotation)
}

// Fixed utility functions using nalgebra
fn quadratic_bezier(p0: Point2<f32>, p1: Point2<f32>, p2: Point2<f32>, t: f32) -> Point2<f32> {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;

    // Convert to vectors for arithmetic, then back to point
    let v0 = p0.coords;
    let v1 = p1.coords;
    let v2 = p2.coords;

    Point2::from(v0 * uu + v1 * (2.0 * u * t) + v2 * tt)
}

fn cubic_bezier(
    p0: Point2<f32>,
    p1: Point2<f32>,
    p2: Point2<f32>,
    p3: Point2<f32>,
    t: f32,
) -> Point2<f32> {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;

    // Convert to vectors for arithmetic, then back to point
    let v0 = p0.coords;
    let v1 = p1.coords;
    let v2 = p2.coords;
    let v3 = p3.coords;

    Point2::from(v0 * uuu + v1 * (3.0 * uu * t) + v2 * (3.0 * u * tt) + v3 * ttt)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}
