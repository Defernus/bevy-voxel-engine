use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent;

impl PlayerComponent {
    pub fn new() -> Self {
        Self {}
    }
}
