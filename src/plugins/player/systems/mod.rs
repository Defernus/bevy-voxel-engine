use crate::{
    common::components::static_mesh::{vertex::Vertex, StaticMeshComponent},
    plugins::player::components::PlayerComponent,
};
use bevy::prelude::*;

pub mod control_system;
pub mod move_system;
pub mod rotate_system;

pub fn player_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let vertices = vec![
        Vertex {
            pos: Vec3::new(-0.5, 0., 0.5),
            color: Color::rgb(1., 0.3, 0.3),
            normal: Vec3::Y,
        },
        Vertex {
            pos: Vec3::new(0.5, 0., 0.5),
            color: Color::rgb(1., 0.3, 0.3),
            normal: Vec3::Y,
        },
        Vertex {
            pos: Vec3::new(0., 0., -0.5),
            color: Color::rgb(1., 0.3, 0.3),
            normal: Vec3::Y,
        },
    ];
    let mesh = StaticMeshComponent::generate_mesh(vertices);
    commands
        .spawn()
        .insert(PlayerComponent::default())
        .insert_bundle(PbrBundle {
            transform: Transform::from_xyz(0., 0., 0.).looking_at(-Vec3::Z, Vec3::Y),
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            ..default()
        });
}
