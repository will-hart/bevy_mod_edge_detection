use bevy::{
    anti_aliasing::fxaa::{Fxaa, Sensitivity},
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    pbr::DirectionalLightShadowMap,
    prelude::*,
};
use bevy_mod_edge_detection::{EdgeDetectionCamera, EdgeDetectionConfig, EdgeDetectionPlugin};

fn main() {
    App::new()
        // MSAA currently doesn't work correctly with the plugin
        .add_plugins((DefaultPlugins, EdgeDetectionPlugin))
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .init_resource::<EdgeDetectionConfig>()
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_entities)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // set up the camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        // The edge detection effect requires the depth and normal prepass
        DepthPrepass,
        NormalPrepass,
        // Add some anti-aliasing because the lines can be really harsh otherwise
        // This isn't required, but some form of AA is recommended
        Msaa::Off,
        Fxaa {
            enabled: true,
            edge_threshold: Sensitivity::Extreme,
            edge_threshold_min: Sensitivity::Extreme,
        },
        EdgeDetectionCamera,
    ));

    // set up basic scene

    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("castle.glb"))),
        Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, 1.4)).with_scale(Vec3::splat(0.5)),
    ));
    commands.spawn((
        Transform::default(),
        Visibility::Visible,
        Rotate(1.5),
        children![(
            PointLight {
                shadows_enabled: true,
                intensity: 20_000_000.0,
                ..default()
            },
            Transform::from_xyz(4.0, 8.0, 4.0),
        )],
    ));
}

#[derive(Component, Default)]
struct Rotate(f32);

fn rotate_entities(time: Res<Time>, mut items: Query<(&mut Transform, &Rotate)>) {
    for (mut tx, rotate) in &mut items {
        let speed = rotate.0;
        // let delta = Quat::from_axis_angle(Vec3::Z, speed * time.delta_secs());
        tx.rotate_y(speed * time.delta_secs());
    }
}
