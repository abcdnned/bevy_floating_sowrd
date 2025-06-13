use bevy::prelude::*;
use bevy::window::{PrimaryWindow};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cursor)
            .add_systems(Update, handle_cursor_toggle);
    }
}

fn setup_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.single_mut() {
        window.cursor_options.visible = false;
    }
}

fn handle_cursor_toggle(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        if let Ok(mut window) = window_query.single_mut() {
            window.cursor_options.visible = !window.cursor_options.visible;
        }
    }
}
