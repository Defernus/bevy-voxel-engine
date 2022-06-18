use bevy::{prelude::*, render::camera::Projection};

#[derive(Component)]
pub struct PlayerComponent;

impl PlayerComponent {
    pub fn new() -> Self {
        Self {}
    }

    pub fn spawn(commands: &mut Commands) {
        commands
            .spawn_bundle(Camera3dBundle {
                transform: Transform::from_xyz(0., 0., 0.).looking_at(-Vec3::Z, Vec3::Y),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: std::f32::consts::PI / 2.0,
                    ..default()
                }),
                ..default()
            })
            .insert(PlayerComponent);
    }
}
