use bevy::{app::AppExit, prelude::*};

pub struct WindowPlugin;

fn window_startup_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_position(Vec2::new(window.width() / 2., window.height() / 2.));
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);
}

fn window_close_system(mut exit: EventWriter<AppExit>, keys: Res<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(window_startup_system)
            .add_system(window_close_system);
    }
}
