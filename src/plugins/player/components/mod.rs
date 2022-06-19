use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent {
    pub noclip: bool,
    pub speed: Vec3,
}

impl Default for PlayerComponent {
    fn default() -> Self {
        Self {
            noclip: false,
            speed: -Vec3::Z,
        }
    }
}
