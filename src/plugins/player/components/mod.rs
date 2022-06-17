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
                transform: Transform::from_xyz(0., 0., 0.).looking_at(-Vec3::Z, Vec3::Y),
                ..default()
            })
            .insert(PlayerComponent);
    }
}
