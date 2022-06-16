use bevy::prelude::*;

use crate::{
    components::chunk::{pos::PosComponent, ChunkComponent},
    resources::generator::GeneratorResource,
};
pub struct ChunkPlugin;

fn chunk_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    generator: Res<GeneratorResource>,
) {
    let range = -4..2;
    for x in range.clone() {
        for y in range.clone() {
            for z in range.clone() {
                ChunkComponent::spawn(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &generator,
                    PosComponent::new(x, y, z),
                );
            }
        }
    }
}

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(chunk_startup_system);
    }
}
