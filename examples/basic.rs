use bevy::prelude::*;
use bevy_fontmesh::{FontMeshPlugin, TextMeshBundle, TextMesh, TextMeshStyle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FontMeshPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_text)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Text
    commands.spawn(TextMeshBundle {
        text_mesh: TextMesh {
            text: "FontMesh".to_string(),
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            style: TextMeshStyle {
                color: Color::srgb(1.0, 0.5, 0.0), // Orange
                depth: 0.5,
                quality: 20,
            },
        },
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.5, 0.0),
            perceptual_roughness: 0.1,
            ..default()
        }),
        transform: Transform::from_xyz(-2.5, 0.0, 0.0),
        ..default()
    });
}

fn rotate_text(time: Res<Time>, mut query: Query<&mut Transform, With<TextMesh>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}
