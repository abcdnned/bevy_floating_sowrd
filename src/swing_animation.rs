use crate::sword::Sword;
use bevy::prelude::*;
use nalgebra::{Point2, Vector2};
use std::f32::consts::PI;

pub struct SwingAnimationPlugin;

impl Plugin for SwingAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animate_sword_swing, handle_input));
    }
}

#[derive(Component)]
pub struct SwingAnimation {
    pub timer: Timer,
    pub start_pos: Vec2,
    pub start_rotation: f32,
    pub startup_timer: Timer,
    pub end_timer: Timer,
    pub is_swinging: bool,
    pub swing_type: SwingType,
    // Add phase tracking
    pub current_phase: SwingPhase,
    pub swing_timer: Timer, // For the main swing phase
    // Store the actual end state of swing phase for recovery
    pub swing_end_pos: Vec2,
    pub swing_end_rotation: f32,
}

#[derive(Clone)]
pub enum SwingType {
    Vertical,
}

#[derive(Clone)]
pub enum SwingPhase {
    Startup,  // Moving to start position
    Swing,    // Main swing animation
    Recovery, // Moving to end position
}

fn handle_input(
   mouse: Res<ButtonInput<MouseButton>>,
   mut sword_query: Query<&mut SwingAnimation, With<Sword>>,
) {
   if let Ok(mut swing) = sword_query.single_mut() {
       if !swing.is_swinging {
           if mouse.just_pressed(MouseButton::Left) {
               start_swing(&mut swing, SwingType::Vertical);
           }
       }
   }
}

fn start_swing(swing: &mut SwingAnimation, swing_type: SwingType) {
    swing.is_swinging = true;
    swing.current_phase = SwingPhase::Startup;
    swing.startup_timer.reset();
    swing.swing_timer.reset();
    swing.end_timer.reset();
    swing.swing_type = swing_type;
}

fn animate_sword_swing(
    time: Res<Time>,
    mut sword_query: Query<(&mut Transform, &mut SwingAnimation), With<Sword>>,
) {
    for (mut transform, mut swing) in sword_query.iter_mut() {
        if swing.is_swinging {
            match swing.current_phase {
                SwingPhase::Startup => {
                    // Phase 1: Move to start position using linear interpolation
                    swing.startup_timer.tick(time.delta());
                    let startup_progress = swing.startup_timer.elapsed_secs() / swing.startup_timer.duration().as_secs_f32();
                    
                    if startup_progress >= 1.0 {
                        // Move to swing phase
                        swing.current_phase = SwingPhase::Swing;
                        swing.swing_timer.reset();
                        
                        // Set to exact start position
                        transform.translation = Vec3::new(swing.start_pos.x, swing.start_pos.y, 0.0);
                        transform.rotation = Quat::from_rotation_z(swing.start_rotation);
                    } else {
                        // Linear interpolation to start position
                        let current_pos = Vec2::lerp(Vec2::ZERO, swing.start_pos, startup_progress);
                        let current_rotation = lerp(0.0, swing.start_rotation, startup_progress);
                        
                        transform.translation = Vec3::new(current_pos.x, current_pos.y, 0.0);
                        transform.rotation = Quat::from_rotation_z(current_rotation);
                    }
                }
                
                SwingPhase::Swing => {
                    // Phase 2: Main swing using cubic bezier
                    swing.swing_timer.tick(time.delta());
                    let swing_progress = swing.swing_timer.elapsed_secs() / swing.swing_timer.duration().as_secs_f32();
                    
                    if swing_progress >= 1.0 {
                        // Store the final swing position and rotation for recovery phase
                        let (swing_position, swing_rotation) = match swing.swing_type {
                            SwingType::Vertical => calculate_vertical_swing_cubic(1.0),
                        };
                        swing.swing_end_pos = swing.start_pos + swing_position;
                        swing.swing_end_rotation = swing.start_rotation + swing_rotation;
                        // Move to recovery phase
                        swing.current_phase = SwingPhase::Recovery;
                        swing.end_timer.reset();
                    } else {
                        // Use cubic bezier for swing animation
                        let (swing_position, swing_rotation) = match swing.swing_type {
                            SwingType::Vertical => calculate_vertical_swing_cubic(swing_progress),
                        };
                        
                        // Add swing motion to start position
                        let final_pos = swing.start_pos + swing_position;
                        let final_rotation = swing.start_rotation + swing_rotation;
                        
                        transform.translation = Vec3::new(final_pos.x, final_pos.y, 0.0);
                        transform.rotation = Quat::from_rotation_z(final_rotation);
                    }
                }
                
                SwingPhase::Recovery => {
                    // Phase 3: Move back to origin (0, 0) using linear interpolation
                    swing.end_timer.tick(time.delta());
                    let end_progress = swing.end_timer.elapsed_secs() / swing.end_timer.duration().as_secs_f32();
                    
                    if end_progress >= 1.0 {
                        // Animation complete - return to origin
                        swing.is_swinging = false;
                        transform.translation = Vec3::ZERO;
                        transform.rotation = Quat::IDENTITY;
                    } else {
                        // Linear interpolation from actual swing end position back to origin
                        let current_pos = Vec2::lerp(swing.swing_end_pos, Vec2::ZERO, end_progress);
                        let current_rotation = lerp(swing.swing_end_rotation, 0.0, end_progress);
                        
                        transform.translation = Vec3::new(current_pos.x, current_pos.y, 0.0);
                        transform.rotation = Quat::from_rotation_z(current_rotation);
                    }
                }
            }
        }
    }
}

fn get_swing_end_position(swing_type: SwingType) -> Vec2 {
    match swing_type {
        SwingType::Vertical => Vec2::new(0.0, -80.0),
    }
}

fn get_swing_end_rotation(swing_type: SwingType) -> f32 {
    match swing_type {
        SwingType::Vertical => PI * 0.9,
    }
}

// Keep the original calculate_vertical_swing for reference
fn calculate_vertical_swing_cubic(t: f32) -> (Vec2, f32) {
    // Vertical swing using cubic bezier curve
    let progress = smooth_step(t);
    
    // Define cubic bezier control points for a pronounced U-shaped arc
    let p0 = Point2::new(0.0, 0.0);      // Start relative to start_pos
    let p1 = Point2::new(00.0, -200.0);  // First control point (far left, slightly down)
    let p2 = Point2::new(200.0, -200.0);   // Second control point (far right, slightly down)
    let p3 = Point2::new(200.0, 000.0);    // End relative to start_pos
    
    // Calculate position using cubic bezier
    let pos = cubic_bezier(p0, p1, p2, p3, progress);
    let position = Vec2::new(pos.x, pos.y);
    
    // Rotation follows the swing direction
    let rotation = lerp(0.0, PI * 2.1, progress);
    
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
    // Standard cubic bezier formula: B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
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
    
    let result = v0 * uuu + v1 * (3.0 * uu * t) + v2 * (3.0 * u * tt) + v3 * ttt;
    Point2::from(result)
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