use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use crate::plugins::chunk::components::chunk_object::ChunkObjectComponent;

use self::handlers::ObjectHandlers;

pub mod handlers;

pub struct Object {
    pub entity: Option<Entity>,
    pub transform: Transform,
    pub id: usize,
}

impl Object {
    pub fn new(id: usize) -> Self {
        Self {
            entity: None,
            id,
            transform: Transform::default(),
        }
    }

    pub fn clear(&mut self, commands: &mut Commands) -> bool {
        self.id = 0;
        match &self.entity {
            Some(e) => {
                commands.entity(*e).despawn_recursive();
                self.entity = None;
                true
            }
            _ => false,
        }
    }
}
