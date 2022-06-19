use bevy::prelude::*;

pub struct WorldPlugin;

fn world_startup_system(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2.,
    });

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            illuminance: 25000.,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(std::f32::consts::PI * 1.25),
            ..default()
        },
        ..default()
    });
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(world_startup_system);
    }
}
