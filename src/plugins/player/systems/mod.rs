use crate::plugins::player::components::PlayerComponent;
use bevy::prelude::*;

pub mod control_system;
pub mod move_system;
pub mod rotate_system;

pub fn player_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn()
        .insert(PlayerComponent::default())
        .insert_bundle(PbrBundle {
            transform: Transform::from_xyz(0., 0., 0.).looking_at(-Vec3::Z, Vec3::Y),
            mesh: meshes.add(Mesh::from(shape::Box::new(1., 1., 1.))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        });
}
