use crate::{
    components::chunk::{pos::PosComponent, ChunkComponent},
    resources::generator::GeneratorResource,
};
use bevy::prelude::*;
pub struct WorldPlugin;

fn world_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    generator: Res<GeneratorResource>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            illuminance: 50000.,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    let range = -2..0;
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

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(world_startup_system);
    }
}
