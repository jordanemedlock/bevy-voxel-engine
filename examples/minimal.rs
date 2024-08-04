use bevy::{core_pipeline::fxaa::Fxaa, prelude::*};
use bevy_voxel_engine::{
    BevyVoxelEnginePlugin, BoxCollider, CollisionEffect, Edges, Flags, LoadVoxelWorld, Portal, RenderGraphSettings, VoxelCameraBundle, VoxelPhysics
};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyVoxelEnginePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_plugins(bevy_mod_debugdump::CommandLineArgs)
        .run();
}

#[derive(Component)]
struct Cube;

fn setup(
    mut commands: Commands,
    mut load_voxel_world: ResMut<LoadVoxelWorld>,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut render_graph_settings: ResMut<RenderGraphSettings>
) {
    // Voxel world
    *load_voxel_world = LoadVoxelWorld::File("assets/monu9.vox".to_string());

    // character
    let character_transform = Transform::from_xyz(5.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y);

    let projection = Projection::Perspective(PerspectiveProjection {
        fov: PI / 2.0,
        ..default()
    });

    // camera
    commands.spawn((
        VoxelCameraBundle {
            transform: character_transform,
            projection: projection.clone(),
            ..default()
        },
        Fxaa::default(),
    ));

    render_graph_settings.trace = false;
}

fn update(mut cube: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
    for mut transform in cube.iter_mut() {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}
