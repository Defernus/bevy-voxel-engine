use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent {
    pub noclip: bool,
    pub speed: Vec3,
}

impl Default for PlayerComponent {
    fn default() -> Self {
        Self {
            noclip: true,
            speed: -Vec3::Z,
        }
    }
}
