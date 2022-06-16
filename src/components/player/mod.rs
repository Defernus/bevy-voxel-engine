use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent;

impl PlayerComponent {
    pub fn new() -> Self {
        Self {}
    }

    pub fn spawn(commands: &mut Commands) {
        commands
            .spawn_bundle(Camera3dBundle {
                transform: Transform::from_xyz(0., 50., -50.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            })
            .insert(PlayerComponent);
    }
}
