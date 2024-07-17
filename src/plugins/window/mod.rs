use bevy::{
    app::AppExit,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::{CursorGrabMode, PresentMode},
};

pub struct WindowPlugin;

fn window_startup_system(mut windows: Query<&mut Window>) {
    let mut window = windows.get_single_mut().unwrap();
    let width = window.width();
    let height = window.height();
    window.set_cursor_position(Some(Vec2::new(width / 2., height / 2.)));
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.present_mode = PresentMode::Mailbox;
}

fn window_close_system(mut exit: EventWriter<AppExit>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn window_fps_system(mut windows: Query<&mut Window>, diagnostics: Res<DiagnosticsStore>) {
    let mut window = windows.get_single_mut().unwrap();
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .expect("fps plugin not added");
    if let Some(average) = fps.value() {
        window.title = format!("fps: {}", average as i32);
    }
}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, window_startup_system)
            .add_systems(Update, window_close_system)
            // REVIEW: Should this be PostUpdate?
            .add_systems(Update, window_fps_system);
    }
}
